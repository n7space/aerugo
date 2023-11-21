use core::hint::spin_loop;
use core::sync::atomic::{AtomicBool, Ordering};

use aerugo::hal::{
    drivers::{
        nvic::{Interrupt, NVIC},
        spi::{
            chip_config::{
                BitsPerTransfer, ChipConfig, ChipSelectBehavior, ClockPhase, ClockPolarity,
                SerialClockDivider,
            },
            config::{MasterConfig, SelectedChip},
            metadata::SPIMetadata,
            Master, NotConfigured, Spi,
        },
        xdmac::{
            channel::ChannelStatusReader,
            events::ChannelEvents,
            status::StatusReader as XdmacStatusReader,
            transfer::{
                AddressingMode, DataWidth, MicroblockLength, SystemBus, TransferBlock,
                TransferLocation, TransferType, TriggerSource,
            },
            Xdmac,
        },
    },
    interrupt,
    user_peripherals::SPI0,
};
use calldwell::write_str;

/// LSM6DSO chip select signal.
const LSM6DSO_CHIP: SelectedChip = SelectedChip::Chip1;
/// LSM6DSO SPI clock divider. Default peripheral clock is 12MHz, LSM6DSO can work with up to 10MHz,
/// but for safety a 480kHz clock will be used instead (12MHz/25).
const LSM6DSO_SPI_CLOCK_DIVIDER: u8 = 25;
/// LSM6DSO buffers size. Defines maximum possible transfer length.
const LSM6DSO_BUFFER_SIZE: usize = 32;
/// LSM6DSO SPI interface.
#[allow(non_camel_case_types)]
type LSM6DSO_SPI = SPI0;

static mut XDMAC_STATUS_READER: Option<XdmacStatusReader> = None;
static mut XDMAC_CHANNEL_STATUS_READER: Option<ChannelStatusReader> = None;
static SPI_TRANSACTION_FINISHED: AtomicBool = AtomicBool::new(false);

pub fn perform_test(spi: Spi<LSM6DSO_SPI, NotConfigured>, xdmac: &mut Xdmac, nvic: &mut NVIC) {
    write_str("Beginning LSM6DSO communication tests w/ XDMAC...");

    let mut spi = spi.into_master(MasterConfig::new(LSM6DSO_CHIP));
    // SPI config validated with LSM6DSO datasheet.
    spi.configure_chip(
        LSM6DSO_CHIP,
        ChipConfig {
            clock_polarity: ClockPolarity::HighWhenInactive,
            clock_phase: ClockPhase::DataChangedOnLeadingEdge,
            chip_select_behavior: ChipSelectBehavior::DeactivateAfterLastTransfer,
            bits_per_transfer: BitsPerTransfer::Bits8,
            clock_divider: SerialClockDivider::new(LSM6DSO_SPI_CLOCK_DIVIDER).unwrap(),
            delay_before_first_clock: 0,
            delay_between_consecutive_transfers: 0,
        },
    );

    // Technically, one buffer could be used for both RX and TX, but considering Rust safety rules,
    // it might be cleaner to separate those :)
    let mut command_buffer: [u8; LSM6DSO_BUFFER_SIZE] = [0; LSM6DSO_BUFFER_SIZE];
    let mut response_buffer: [u8; LSM6DSO_BUFFER_SIZE] = [0; LSM6DSO_BUFFER_SIZE];

    unsafe { XDMAC_STATUS_READER = xdmac.take_status_reader() };

    let mut rx_channel = xdmac.take_next_free_channel().unwrap();
    unsafe { XDMAC_CHANNEL_STATUS_READER = rx_channel.take_status_reader() };
    rx_channel.set_events_state(ChannelEvents {
        end_of_block: false,
        end_of_list: true,
        end_of_disable: false,
        end_of_flush: false,
        read_bus_error: true,
        write_bus_error: true,
        request_overflow_error: true,
    });
    rx_channel.enable_interrupt();

    let tx_channel = xdmac.take_next_free_channel().unwrap();

    // First, an ID will be read until LSM6DSO responds with valid one, to verify the connection.
    // LSM6DSO may not respond correctly first time, but should respond eventually.
    const READ_REQUEST: u8 = 0x80;
    const WHO_AM_I_ADDRESS: u8 = 0x0F;
    const WHO_AM_I_VALUE: u8 = 0x6C;

    command_buffer[0] = READ_REQUEST | WHO_AM_I_ADDRESS;

    // XDMAC transaction setup:
    let mut rx_channel = rx_channel.configure_transfer(get_rx_transfer_block(
        &spi,
        response_buffer.as_mut_ptr() as *mut (),
        2,
    ));
    let mut tx_channel = tx_channel.configure_transfer(get_tx_transfer_block(
        &spi,
        command_buffer.as_ptr() as *const (),
        2,
    ));

    // Good to go, transaction can be started now.
    nvic.enable(Interrupt::XDMAC);

    rx_channel.enable();
    tx_channel.enable();

    wait_for_spi_transaction();

    // First transaction may not pass, so let's try again - second one should be correct.
    rx_channel.repeat_transfer();
    tx_channel.repeat_transfer();
    rx_channel.enable();
    tx_channel.enable();

    wait_for_spi_transaction();

    nvic.disable(Interrupt::XDMAC);

    assert_eq!(response_buffer[1], WHO_AM_I_VALUE);

    write_str("All LSM6DSO communication tests w/ XDMAC done!");
}

fn wait_for_spi_transaction() {
    while !SPI_TRANSACTION_FINISHED.load(Ordering::SeqCst) {
        spin_loop();
    }
    SPI_TRANSACTION_FINISHED.store(false, Ordering::SeqCst);
}

#[interrupt]
fn XDMAC() {
    let status_reader = unsafe { XDMAC_STATUS_READER.as_mut().unwrap() };
    let channel_status_reader = unsafe { XDMAC_CHANNEL_STATUS_READER.as_mut().unwrap() };

    if status_reader.get_pending_channels()[channel_status_reader.id()] {
        let status = channel_status_reader.get_pending_events();

        if status.read_bus_error {
            panic!("XDMAC read bus error");
        }

        if status.write_bus_error {
            panic!("XDMAC write bus error");
        }

        if status.request_overflow_error {
            panic!("XDMAC request overflow error");
        }

        if status.end_of_list {
            SPI_TRANSACTION_FINISHED.store(true, Ordering::SeqCst);
        }
    }
}

fn get_rx_transfer_block(
    spi: &Spi<LSM6DSO_SPI, Master>,
    destination: *mut (),
    data_length: u32,
) -> TransferBlock {
    let source = TransferLocation {
        address: spi.xdmac_rx_address(),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Fixed,
    };

    let destination = TransferLocation {
        address: destination as *const (),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    TransferBlock::new(
        source,
        destination,
        TransferType::PeripheralToMemory(LSM6DSO_SPI::DMA_RX_PERIPHERAL, TriggerSource::Hardware),
        DataWidth::Byte,
    )
    .unwrap()
    .with_microblock_length(MicroblockLength::new(data_length).unwrap())
}

fn get_tx_transfer_block(
    spi: &Spi<LSM6DSO_SPI, Master>,
    source: *const (),
    data_length: u32,
) -> TransferBlock {
    let source = TransferLocation {
        address: source,
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    let destination = TransferLocation {
        address: spi.xdmac_tx_address(),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Fixed,
    };

    TransferBlock::new(
        source,
        destination,
        TransferType::MemoryToPeripheral(LSM6DSO_SPI::DMA_TX_PERIPHERAL, TriggerSource::Hardware),
        DataWidth::Byte,
    )
    .unwrap()
    .with_microblock_length(MicroblockLength::new(data_length).unwrap())
}
