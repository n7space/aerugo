#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

pub mod command;
pub mod events;
pub mod task_start_measurements;
pub mod task_uart_reader;

use crate::command::*;
use crate::events::*;
use crate::task_start_measurements::*;
use crate::task_uart_reader::*;

use aerugo::{
    hal::{
        drivers::{
            nvic::{Interrupt, NVIC},
            pio::{pin::Peripheral as PioPeripheral, Port},
            pmc::config::PeripheralId,
            uart::{Bidirectional, Config, NotConfigured, ReceiverConfig, Uart},
            xdmac::{
                channel::{Channel, Configured},
                channel_status::ChannelStatusReader,
                events::ChannelEvents,
                status::StatusReader,
                transfer::{
                    AddressingMode, DataWidth, MicroblockLength, Peripheral as XdmacPeripheral,
                    SystemBus, TransferBlock, TransferLocation, TransferType, TriggerSource,
                },
                Xdmac,
            },
        },
        interrupt,
        user_peripherals::{PIOD, PMC, UART4},
    },
    logln,
    time::RateExtU32,
    Aerugo, EventId, EventStorage, InitApi, MessageQueueHandle, MessageQueueStorage,
    SystemHardwareConfig, TaskletConfig, TaskletStorage,
};
use rt::entry;

const TRANSFER_LENGTH: usize = 7;
type TransferArrayType = [u8; TRANSFER_LENGTH];

static mut MESSAGE_BUFFER: TransferArrayType = [0; TRANSFER_LENGTH];

/// This is used for passing XDMAC's status reader to IRQ.
/// It must be initialized before starting an IRQ-synchronized XDMAC transaction, otherwise the
/// program may panic.
/// This can be safely accessed outside of XDMAC IRQ only when no XDMAC transactions are in progress.
static mut XDMAC_STATUS_READER: Option<StatusReader> = None;
/// This is used for passing XDMAC's channel status reader to IRQ.
/// It must be initialized before starting an IRQ-synchronized XDMAC transaction, otherwise the
/// program may panic.
/// This can be safely accessed outside of XDMAC IRQ only when no XDMAC transactions are in progress.
static mut XDMAC_CHANNEL_STATUS_READER: Option<ChannelStatusReader> = None;
/// This is used for passing XDMAC's channel to IRQ.
/// It must be initialized before starting an IRQ-synchronized XDMAC transaction, otherwise the
/// program may panic.
/// This can be safely accessed outside of XDMAC IRQ only when no XDMAC transactions are in progress.
static mut XDMAC_RX_CHANNEL: Option<Channel<Configured>> = None;
/// This is used for passing command queue handle to IRQ.
/// It must be initialized before starting an IRQ-synchronized XDMAC transaction, otherwise the
/// probram may panic.
static mut XDMAC_COMMAND_QUEUE_HANDLE: Option<MessageQueueHandle<TransferArrayType, 10>> = None;

static TASK_UART_READER_STORAGE: TaskletStorage<TransferArrayType, TaskUartReaderContext, 0> =
    TaskletStorage::new();
static TASK_START_MEASUREMENTS_STORAGE: TaskletStorage<EventId, TaskStartMeasurementsContext, 0> =
    TaskletStorage::new();

static QUEUE_COMMAND_STORAGE: MessageQueueStorage<TransferArrayType, 10> =
    MessageQueueStorage::new();

static EVENT_START_STORAGE: EventStorage = EventStorage::new();
static EVENT_STOP_STORAGE: EventStorage = EventStorage::new();

#[entry]
fn main() -> ! {
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    let pmc = peripherals.pmc.unwrap();
    init_clocks(pmc);

    let port = Port::new(peripherals.pio_d.take().unwrap());
    init_pio(port);

    let uart = Uart::new(peripherals.uart_4.take().unwrap());
    let mut uart = init_uart(uart);

    let xdmac = Xdmac::new(peripherals.xdmac.take().unwrap());
    init_xdmac(xdmac, &mut uart);

    let mut nvic = NVIC::new(peripherals.nvic.take().unwrap());
    nvic.enable(Interrupt::XDMAC);

    init_system(aerugo);

    unsafe {
        XDMAC_RX_CHANNEL.as_mut().unwrap().enable();
    }

    aerugo.start();
}

fn init_clocks(mut pmc: PMC) {
    pmc.enable_peripheral_clock(PeripheralId::PIOD);
    pmc.enable_peripheral_clock(PeripheralId::UART4);
    pmc.enable_peripheral_clock(PeripheralId::XDMAC);
}

fn init_pio(port: Port<PIOD>) {
    let mut pins = port.into_pins();
    pins[18]
        .take()
        .unwrap()
        .into_peripheral_pin(PioPeripheral::C);
    pins[19]
        .take()
        .unwrap()
        .into_peripheral_pin(PioPeripheral::D);
}

fn init_uart(uart: Uart<UART4, NotConfigured>) -> Uart<UART4, Bidirectional> {
    let uart_config = Config::new(9600, 12.MHz()).unwrap();
    let recv_config = ReceiverConfig {
        rx_filter_enabled: true,
    };

    uart.into_bidirectional(uart_config, recv_config)
}

fn init_xdmac(mut xdmac: Xdmac, uart: &mut Uart<UART4, Bidirectional>) {
    // Place XDMAC status reader in IRQ storage.
    // This is safe, because XDMAC IRQ should be disabled.
    unsafe { XDMAC_STATUS_READER.replace(xdmac.take_status_reader().unwrap()) };

    let rx_source_location = TransferLocation {
        address: uart.xdmac_rx_address(),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Fixed,
    };

    let rx_destination_location = TransferLocation {
        address: unsafe { MESSAGE_BUFFER.as_mut_ptr() as *const () },
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    let transfer_microblock_length = MicroblockLength::new(TRANSFER_LENGTH as u32).unwrap();

    let rx_transfer = TransferBlock::new(
        rx_source_location,
        rx_destination_location,
        TransferType::PeripheralToMemory(XdmacPeripheral::UART4_RX, TriggerSource::Hardware),
        DataWidth::Byte,
    )
    .unwrap()
    .with_microblock_length(transfer_microblock_length);

    let mut rx_channel = xdmac.take_next_free_channel().unwrap();

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

    unsafe { XDMAC_CHANNEL_STATUS_READER.replace(rx_channel.take_status_reader().unwrap()) };

    unsafe {
        XDMAC_RX_CHANNEL.replace(rx_channel);
    }
}

fn init_system(aerugo: &'static impl InitApi) {
    // Queues

    aerugo.create_message_queue(&QUEUE_COMMAND_STORAGE);
    let queue_command_handle = QUEUE_COMMAND_STORAGE.create_handle().unwrap();

    // Events

    aerugo.create_event(CommandEvent::Start.into(), &EVENT_START_STORAGE);
    aerugo.create_event(CommandEvent::Stop.into(), &EVENT_STOP_STORAGE);

    // UART reader

    aerugo.create_tasklet(
        TaskletConfig {
            name: "UartReader",
            ..Default::default()
        },
        task_uart_reader,
        &TASK_UART_READER_STORAGE,
    );

    let task_uart_reader_handle = TASK_UART_READER_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_queue(&task_uart_reader_handle, &queue_command_handle);

    // Start measurements

    aerugo.create_tasklet(
        TaskletConfig {
            name: "StartMeasurements",
            ..Default::default()
        },
        task_start_measurements,
        &TASK_START_MEASUREMENTS_STORAGE,
    );

    let task_start_measurements_handle = TASK_START_MEASUREMENTS_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_events(&task_start_measurements_handle, [CommandEvent::Start.into()]);

    // Post-init

    unsafe {
        XDMAC_COMMAND_QUEUE_HANDLE.replace(queue_command_handle);
    }
}

#[interrupt]
fn XDMAC() {
    logln!("XDMAC");

    let rx_channel = unsafe { XDMAC_RX_CHANNEL.as_mut().unwrap() };

    let channel_status_reader = unsafe { XDMAC_CHANNEL_STATUS_READER.as_mut().unwrap() };

    let status = unsafe { XDMAC_STATUS_READER.as_mut().unwrap().get_pending_channels() };

    if status[channel_status_reader.id()] {
        let events = channel_status_reader.get_pending_events();

        if events.read_bus_error {
            panic!("XDMAC read bus error detected");
        }
        if events.write_bus_error {
            panic!("XDMAC write bus error detected");
        }
        if events.request_overflow_error {
            panic!("XDMAC request overflow error detected");
        }
    }

    unsafe {
        XDMAC_COMMAND_QUEUE_HANDLE
            .unwrap()
            .send_data(MESSAGE_BUFFER)
            .expect("Failed to send command to the queue");
    }

    rx_channel.repeat_transfer();
    rx_channel.enable();
}