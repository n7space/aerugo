use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};

use aerugo::{
    hal::{
        drivers::{
            nvic::{Interrupt, NVIC},
            uart::{Config, NotConfigured, ReceiverConfig, Uart},
            xdmac::{
                channel::{Channel, Configured},
                channel_status::ChannelStatusReader,
                events::ChannelEvents,
                status::StatusReader,
                transfer::{
                    AddressingMode, DataWidth, MicroblockLength, Peripheral, SystemBus,
                    TransferBlock, TransferLocation, TransferType, TriggerSource,
                },
                Xdmac,
            },
        },
        interrupt,
        user_peripherals::UART0,
    },
    time::RateExtU32,
    Mutex,
};
use calldwell::write_str;
use heapless::Vec;

/// This storage is used for passing XDMAC's status reader to IRQ.
/// It must be initialized before starting an IRQ-synchronized XDMAC transaction, otherwise the
/// program may panic.
/// This storage can be safely accessed outside of XDMAC IRQ only when no XDMAC transactions are in
/// progress
static mut STATUS_READER_STORAGE: Option<StatusReader> = None;

/// This storage is used for passing XDMAC's channel status reader to IRQ.
/// It must be initialized before starting an IRQ-synchronized XDMAC transaction, otherwise the
/// program may panic.
/// This storage can be safely accessed outside of XDMAC IRQ only when no XDMAC transactions are in
/// progress.
static mut CHANNEL_STATUS_READER_STORAGE: Option<ChannelStatusReader> = None;

/// This field is used by IRQ to indicate that a new channel event has happened and new ChannelEvents
/// instance has been placed in [`LAST_CHANNEL_EVENTS`].
static CHANNEL_IRQ_HANDLED: AtomicBool = AtomicBool::new(false);

/// This field stores channel events. It can be read when [`CHANNEL_IRQ_HANDLED`] is set to `true`.
static LAST_CHANNEL_EVENTS: Mutex<RefCell<Option<ChannelEvents>>> = Mutex::new(RefCell::new(None));

pub fn perform_test(mut xdmac: Xdmac, mut nvic: NVIC, uart: Uart<UART0, NotConfigured>) {
    write_str("Performing transfer tests...");

    // Place XDMAC status reader in IRQ storage.
    // This is safe, because XDMAC IRQ should be disabled.
    unsafe { STATUS_READER_STORAGE.replace(xdmac.take_status_reader().unwrap()) };

    test_mem2mem_32bit_transfer_polling(&mut xdmac);
    test_mem2mem_16bit_transfer_polling(&mut xdmac);
    test_mem2mem_8bit_transfer_polling(&mut xdmac);
    test_mem2mem_irq_transfer(&mut xdmac, &mut nvic);
    let uart = test_per2mem_and_mem2per_transfers(&mut xdmac, &mut nvic, uart);
    let _ = test_channel_suspend_and_disable(&mut xdmac, &mut nvic, uart);

    // Take XDMAC status reader from IRQ storage.
    // This is safe, because XDMAC IRQ should be disabled.
    unsafe { xdmac.return_status_reader(STATUS_READER_STORAGE.take().unwrap()) };

    write_str("Transfer tests finished successfully!");
}

/// Performs simple 32-bit XDMAC memory-to-memory transfer. Polls for completion.
fn test_mem2mem_32bit_transfer_polling(xdmac: &mut Xdmac) {
    // Those should be 4-byte aligned.
    let source: Vec<u32, 0x400> = (0xBEEF00AB..0xBEEF04AB).collect();
    let mut destination: [u32; 0x400] = [0; 0x400];

    let source_location = TransferLocation {
        address: source.as_ptr() as *const (),
        interface: SystemBus::Interface0,
        addressing_mode: AddressingMode::Incremented,
    };

    let destination_location = TransferLocation {
        address: destination.as_mut_ptr() as *const (),
        interface: SystemBus::Interface0,
        addressing_mode: AddressingMode::Incremented,
    };

    let transfer = TransferBlock::new(
        source_location,
        destination_location,
        TransferType::MemoryToMemory,
        DataWidth::FourBytes,
    )
    .unwrap()
    .with_microblock_length(MicroblockLength::new(0x400).unwrap());

    let mut channel = xdmac
        .take_next_free_channel()
        .unwrap()
        .configure_transfer(transfer);

    channel.enable();
    channel.trigger();
    while channel.is_busy() {}

    assert_eq!(destination, source);

    xdmac.return_channel(channel.reset_state().unwrap());

    write_str("Mem2mem 32-bit polling transfer test successful!");
}

/// Performs simple 16-bit XDMAC memory-to-memory transfer. Polls for completion.
fn test_mem2mem_16bit_transfer_polling(xdmac: &mut Xdmac) {
    // Those should be 2-byte aligned.
    let source: Vec<u16, 0x200> = (0x1000..0x1200).collect();
    let mut destination: [u16; 0x200] = [0; 0x200];

    let source_location = TransferLocation {
        address: source.as_ptr() as *const (),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    let destination_location = TransferLocation {
        address: destination.as_mut_ptr() as *const (),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    let transfer = TransferBlock::new(
        source_location,
        destination_location,
        TransferType::MemoryToMemory,
        DataWidth::TwoBytes,
    )
    .unwrap()
    .with_microblock_length(MicroblockLength::new(0x200).unwrap());

    let mut channel = xdmac
        .take_next_free_channel()
        .unwrap()
        .configure_transfer(transfer);

    channel.enable();
    while channel.is_busy() {}

    assert_eq!(destination, source);

    xdmac.return_channel(channel.reset_state().unwrap());

    write_str("Mem2mem 16-bit polling transfer test successful!");
}

/// Performs simple 8-bit XDMAC memory-to-memory transfer. Polls for completion.
fn test_mem2mem_8bit_transfer_polling(xdmac: &mut Xdmac) {
    let source: Vec<u8, 128> = (0..128).collect();
    let mut destination: [u8; 128] = [0; 128];

    let source_location = TransferLocation {
        address: source.as_ptr() as *const (),
        interface: SystemBus::Interface0,
        addressing_mode: AddressingMode::Incremented,
    };

    let destination_location = TransferLocation {
        address: destination.as_mut_ptr() as *const (),
        interface: SystemBus::Interface0,
        addressing_mode: AddressingMode::Incremented,
    };

    let transfer = TransferBlock::new(
        source_location,
        destination_location,
        TransferType::MemoryToMemory,
        DataWidth::Byte,
    )
    .unwrap()
    .with_microblock_length(MicroblockLength::new(128).unwrap());

    let mut channel = xdmac
        .take_next_free_channel()
        .unwrap()
        .configure_transfer(transfer);

    channel.enable();
    while channel.is_busy() {}

    assert_eq!(destination, source);

    xdmac.return_channel(channel.reset_state().unwrap());

    write_str("Mem2mem 8-bit polling transfer test successful!");
}

fn wait_for_xdmac_irq() {
    while !CHANNEL_IRQ_HANDLED.load(Ordering::SeqCst) {
        core::hint::spin_loop()
    }
    CHANNEL_IRQ_HANDLED.store(false, Ordering::SeqCst);
}

/// Performs simple XDMAC memory-to-memory transfer. Waits for interrupt for completion.
fn test_mem2mem_irq_transfer(xdmac: &mut Xdmac, nvic: &mut NVIC) {
    // Those should be 4-byte aligned.
    let source: Vec<u32, 0x200> = (0xDDDD00FF..0xDDDD02FF).collect();
    let mut destination: [u32; 0x200] = [0; 0x200];

    let source_location = TransferLocation {
        address: source.as_ptr() as *const (),
        interface: SystemBus::Interface0,
        addressing_mode: AddressingMode::Incremented,
    };

    let destination_location = TransferLocation {
        address: destination.as_mut_ptr() as *const (),
        interface: SystemBus::Interface0,
        addressing_mode: AddressingMode::Incremented,
    };

    let transfer = TransferBlock::new(
        source_location,
        destination_location,
        TransferType::MemoryToMemory,
        DataWidth::FourBytes,
    )
    .unwrap()
    .with_microblock_length(MicroblockLength::new(0x200).unwrap());

    let mut channel = xdmac.take_next_free_channel().unwrap();

    channel.set_events_state(ChannelEvents {
        end_of_block: true,
        end_of_list: true,
        end_of_disable: false,
        end_of_flush: false,
        read_bus_error: true,
        write_bus_error: true,
        request_overflow_error: true,
    });
    channel.enable_interrupt();

    let mut channel = channel.configure_transfer(transfer);
    unsafe { CHANNEL_STATUS_READER_STORAGE.replace(channel.take_status_reader().unwrap()) };

    // No more touching the global XDMAC IRQ-related state, except what's guarded by atomics,
    // beyond this point.
    nvic.enable(Interrupt::XDMAC);

    channel.enable();

    wait_for_xdmac_irq();

    LAST_CHANNEL_EVENTS.lock(|events| {
        let events = events.take().unwrap();
        assert!(events.end_of_block);
        assert!(events.end_of_list);
    });

    nvic.disable(Interrupt::XDMAC);
    // Global state related to XDMAC IRQ can be touched again.

    assert_eq!(destination, source);

    channel.return_status_reader(unsafe { CHANNEL_STATUS_READER_STORAGE.take().unwrap() });
    // reset_state disables events and channel interrupts.
    xdmac.return_channel(channel.reset_state().unwrap());

    write_str("Mem2mem interrupt transfer test successful!");
}

fn flush_channel(channel: &mut Channel<Configured>) {
    channel.flush();
    wait_for_xdmac_irq();
    LAST_CHANNEL_EVENTS.lock(|events| {
        let events = events.take().unwrap();
        assert!(!events.end_of_block);
        assert!(!events.end_of_list);
        assert!(!events.end_of_disable);
        assert!(events.end_of_flush);
    });
    // To make sure the data is *actually* there.
    // asm::dsb();
}

/// UART in this test is configured in local loopback mode.
/// Test performs peripheral-to-memory transfer using UART as source peripheral (for extracting the
/// data out of UART), and memory-to-peripheral transfer using UART as destination peripheral (for
/// putting the data into UART).
///
/// This test also validates flush operation, as the TX channel is flushed after the last byte is
/// transmitted.
///
/// This test also verifies whether an errata workaround is applied - if data width is less than
/// full word (32-bit), and either source or destination address is fixed, then it's addressing
/// mode must be set to data striding on microblock boundary with the data stride set to -1, as
/// otherwise the address would be incremented. It still is, but it's also automatically decremented,
/// so it cancels out :D
fn test_per2mem_and_mem2per_transfers(
    xdmac: &mut Xdmac,
    nvic: &mut NVIC,
    uart: Uart<UART0, NotConfigured>,
) -> Uart<UART0, NotConfigured> {
    // Prepare UART
    let mut uart = uart.into_bidirectional(
        Config::new(9600, 12.MHz()).unwrap(),
        ReceiverConfig {
            rx_filter_enabled: true,
        },
    );
    uart.switch_to_local_loopback_mode();

    // Offset by 1, because first value is `1`, not `0`.
    const TRANSFER_LENGTH: usize = (u8::MAX as usize) - 1;
    // Sanity check
    assert!(TRANSFER_LENGTH < u8::MAX as usize);

    // Prepare I/O buffers
    let source: Vec<u8, { TRANSFER_LENGTH }> = (1..u8::MAX).collect();
    let mut destination: [u8; TRANSFER_LENGTH] = [0; TRANSFER_LENGTH];

    // Prepare transfers
    let rx_source_location = TransferLocation {
        address: uart.xdmac_rx_address(),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Fixed,
    };

    let rx_destination_location = TransferLocation {
        address: destination.as_mut_ptr() as *const (),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    let transfer_microblock_length = MicroblockLength::new(TRANSFER_LENGTH as u32).unwrap();

    let rx_transfer = TransferBlock::new(
        rx_source_location,
        rx_destination_location,
        TransferType::PeripheralToMemory(Peripheral::UART0_RX, TriggerSource::Hardware),
        DataWidth::Byte,
    )
    .unwrap()
    .with_microblock_length(transfer_microblock_length);

    let tx_source_location = TransferLocation {
        address: source.as_ptr() as *const (),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    let tx_destination_location = TransferLocation {
        address: uart.xdmac_tx_address(),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Fixed,
    };

    let tx_transfer = TransferBlock::new(
        tx_source_location,
        tx_destination_location,
        TransferType::MemoryToPeripheral(Peripheral::UART0_TX, TriggerSource::Hardware),
        DataWidth::Byte,
    )
    .unwrap()
    .with_microblock_length(transfer_microblock_length);

    // Get channels for transfers
    let mut rx_channel = xdmac.take_next_free_channel().unwrap();
    let tx_channel = xdmac.take_next_free_channel().unwrap();

    // RX channel will be queried using an interrupt. End of RX implies end of TX, as both transfers
    // have the same data length.
    rx_channel.set_events_state(ChannelEvents {
        end_of_block: true,
        end_of_list: true,
        end_of_disable: false,
        end_of_flush: true,
        read_bus_error: true,
        write_bus_error: true,
        request_overflow_error: true,
    });
    rx_channel.enable_interrupt();

    let mut rx_channel = rx_channel.configure_transfer(rx_transfer);
    let mut tx_channel = tx_channel.configure_transfer(tx_transfer);

    unsafe { CHANNEL_STATUS_READER_STORAGE.replace(rx_channel.take_status_reader().unwrap()) };

    // No more touching the global XDMAC IRQ-related state, except what's guarded by atomics,
    // beyond this point.
    nvic.enable(Interrupt::XDMAC);

    // Start the transactions. RX must be started first.
    rx_channel.enable();
    tx_channel.enable();

    // First IRQ should indicate that transfer has been finished.
    wait_for_xdmac_irq();

    LAST_CHANNEL_EVENTS.lock(|events| {
        let events = events.take().unwrap();
        assert!(events.end_of_block);
        assert!(events.end_of_list);
        assert!(!events.end_of_flush);
    });

    // Flush the channel for good measure.
    flush_channel(&mut rx_channel);

    nvic.disable(Interrupt::XDMAC);
    // Global state related to XDMAC IRQ can be touched again.

    rx_channel.return_status_reader(unsafe { CHANNEL_STATUS_READER_STORAGE.take().unwrap() });
    // reset_state disables events and channel interrupts.
    xdmac.return_channel(rx_channel.reset_state().unwrap());
    xdmac.return_channel(tx_channel.reset_state().unwrap());

    assert_eq!(destination, source);

    write_str("Per2mem and mem2per transfers test successful!");

    uart.disable()
}

/// UART in this test is configured in local loopback mode.
/// Test performs peripheral-to-memory transfer using UART as source peripheral (for extracting the
/// data out of UART). Transmission via UART is done manually, and after transmitting less data than
/// XDMAC transfer is configured to fetch, the channel is suspended. Then, more data is tried to be
/// transmitted, and it should be placed in channel's FIFO until the channel operation is resumed.
/// After that, channel is flushed and resumed, more data is transmitted, and channel is suspended
/// again and disabled before the transaction is completed.
#[allow(clippy::assertions_on_constants)] // too insane to be sane
fn test_channel_suspend_and_disable(
    xdmac: &mut Xdmac,
    nvic: &mut NVIC,
    uart: Uart<UART0, NotConfigured>,
) -> Uart<UART0, NotConfigured> {
    // Prepare UART
    let mut uart = uart.into_bidirectional(
        Config::new(9600, 12.MHz()).unwrap(),
        ReceiverConfig {
            rx_filter_enabled: true,
        },
    );
    uart.switch_to_local_loopback_mode();

    let mut tx = uart.take_writer().unwrap();

    const TRANSFER_LENGTH: usize = 200;
    const FIRST_TRANSFER_CHUNK_LENGTH: usize = 100;
    const SECOND_TRANSFER_CHUNK_LENGTH: usize = 50;
    const TRANSMITTED_DATA_LENGTH: usize =
        FIRST_TRANSFER_CHUNK_LENGTH + SECOND_TRANSFER_CHUNK_LENGTH;
    assert!(TRANSFER_LENGTH < u8::MAX as usize);
    assert!(TRANSMITTED_DATA_LENGTH < TRANSFER_LENGTH);
    // This check is required, as the second chunk is transmitted when write is suspended, and
    // therefore is buffered in FIFO.
    assert!(SECOND_TRANSFER_CHUNK_LENGTH < xdmac.fifo_size());
    const REMAINING_BYTES: usize =
        TRANSFER_LENGTH - (FIRST_TRANSFER_CHUNK_LENGTH + SECOND_TRANSFER_CHUNK_LENGTH);

    // Prepare I/O buffers
    let mut destination: [u8; TRANSFER_LENGTH] = [0; TRANSFER_LENGTH];

    // This buffer contains the expected destination state after the first chunk has been transmitted.
    let mut expected_destination_post_first_chunk: Vec<u8, { TRANSFER_LENGTH }> =
        (1..=(FIRST_TRANSFER_CHUNK_LENGTH as u8)).collect();
    expected_destination_post_first_chunk
        .extend((0..(SECOND_TRANSFER_CHUNK_LENGTH + REMAINING_BYTES)).map(|_| 0));

    // We expect that first two chunks of the data are in the buffer, and the rest is empty.
    let mut expected_destination: Vec<u8, { TRANSFER_LENGTH }> =
        (1..=(TRANSMITTED_DATA_LENGTH as u8)).collect();
    expected_destination.extend((0..REMAINING_BYTES).map(|_| 0));

    // Prepare transfer
    let rx_source_location = TransferLocation {
        address: uart.xdmac_rx_address(),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Fixed,
    };

    let rx_destination_location = TransferLocation {
        address: destination.as_mut_ptr() as *const (),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    let rx_transfer = TransferBlock::new(
        rx_source_location,
        rx_destination_location,
        TransferType::PeripheralToMemory(Peripheral::UART0_RX, TriggerSource::Hardware),
        DataWidth::Byte,
    )
    .unwrap()
    .with_microblock_length(MicroblockLength::new(TRANSFER_LENGTH as u32).unwrap());

    // Get and configure channel for transfer
    let mut rx_channel = xdmac.take_next_free_channel().unwrap();
    rx_channel.set_events_state(ChannelEvents {
        end_of_block: true,
        end_of_list: true,
        end_of_disable: true,
        end_of_flush: true,
        read_bus_error: true,
        write_bus_error: true,
        request_overflow_error: true,
    });
    rx_channel.enable_interrupt();
    let mut rx_channel = rx_channel.configure_transfer(rx_transfer);
    unsafe { CHANNEL_STATUS_READER_STORAGE.replace(rx_channel.take_status_reader().unwrap()) };

    // No more touching the global XDMAC IRQ-related state, except what's guarded by atomics,
    // beyond this point.
    nvic.enable(Interrupt::XDMAC);

    // Start the reception.
    rx_channel.enable();

    // Transmit the first chunk of data.
    tx.transmit_bytes(
        &expected_destination_post_first_chunk[0..FIRST_TRANSFER_CHUNK_LENGTH],
        u32::MAX,
    )
    .unwrap();

    // Flush the channel for good measure.
    flush_channel(&mut rx_channel);

    // Verify that the buffer contains the first chunk of data
    assert_eq!(expected_destination_post_first_chunk, destination);

    // Suspend the channel's write operation.
    // This will cause all data read from UART to be buffered in channel's FIFO until either FIFO
    // is full, or write is resumed.
    rx_channel.suspend_write();
    assert!(rx_channel.is_write_suspended());

    // Transmit the second chunk of data into channel's FIFO.
    tx.transmit_bytes(
        &expected_destination[FIRST_TRANSFER_CHUNK_LENGTH..TRANSMITTED_DATA_LENGTH],
        u32::MAX,
    )
    .unwrap();

    // Verify that the buffer still contains *only* the first chunk of data
    assert_eq!(expected_destination_post_first_chunk, destination);

    // Resume the channel's write operation and flush it.
    rx_channel.resume_write();
    assert!(!rx_channel.is_write_suspended());
    flush_channel(&mut rx_channel);

    // Now, the buffer should be in it's final state.
    assert_eq!(expected_destination, destination);

    // Last check - suspend the channel's read operation and verify that by trying to transmit new
    // data.
    rx_channel.suspend_read();
    assert!(rx_channel.is_read_suspended());

    tx.transmit_byte(0xDD, u32::MAX).unwrap();
    tx.flush(u32::MAX).unwrap();
    flush_channel(&mut rx_channel);

    // Buffer's state should be unchanged.
    assert_eq!(expected_destination, destination);

    // But after resuming read and flushing it, the data should appear.
    rx_channel.resume_read();
    assert!(!rx_channel.is_read_suspended());
    flush_channel(&mut rx_channel);

    assert_eq!(destination[TRANSMITTED_DATA_LENGTH], 0xDD);

    // At last, disable the channel.
    rx_channel.disable();

    // And resume it, to check if an IRQ will fire to indicate that.
    rx_channel.enable();
    wait_for_xdmac_irq();
    LAST_CHANNEL_EVENTS.lock(|events| {
        let events = events.take().unwrap();
        assert!(!events.end_of_block);
        assert!(!events.end_of_list);
        assert!(events.end_of_disable);
        assert!(!events.end_of_flush);
    });

    nvic.disable(Interrupt::XDMAC);
    // Global state related to XDMAC IRQ can be touched again.

    rx_channel.return_status_reader(unsafe { CHANNEL_STATUS_READER_STORAGE.take().unwrap() });
    xdmac.return_channel(rx_channel.reset_state().unwrap());

    write_str("Channel management operations (suspend/flush/disable) test successful!");

    uart.put_writer(tx);
    uart.disable()
}

#[interrupt]
fn XDMAC() {
    let channel_status_reader = unsafe { CHANNEL_STATUS_READER_STORAGE.as_mut().unwrap() };

    let status = unsafe {
        STATUS_READER_STORAGE
            .as_mut()
            .unwrap()
            .get_pending_channels()
    };

    if status[channel_status_reader.id()] {
        let events = channel_status_reader.get_pending_events();

        // It's not necessary to validate errors in test code, as they are validated immediately
        // here.
        if events.read_bus_error {
            panic!("XDMAC read bus error detected");
        }

        if events.write_bus_error {
            panic!("XDMAC write bus error detected");
        }

        if events.request_overflow_error {
            panic!("XDMAC request overflow error detected");
        }

        LAST_CHANNEL_EVENTS.lock(|events_storage| {
            events_storage.get_mut().replace(events);
        });

        CHANNEL_IRQ_HANDLED.store(true, Ordering::SeqCst);
    }
}
