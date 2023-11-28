#![no_std]
#![no_main]

extern crate bitfield_enum;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

mod bounded_int;
mod ccsds;
pub mod command;
pub mod events;
pub mod task_get_execution_stats;
pub mod task_set_accelerometer_scale;
pub mod task_set_data_output_rate;
pub mod task_set_gyroscope_scale;
pub mod task_start_measurements;
pub mod task_stop_measurements;
pub mod task_uart_reader;

use crate::command::*;
use crate::events::*;
use crate::task_get_execution_stats::*;
use crate::task_set_accelerometer_scale::*;
use crate::task_set_data_output_rate::*;
use crate::task_set_gyroscope_scale::*;
use crate::task_start_measurements::*;
use crate::task_stop_measurements::*;
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

const TELECOMMAND_LENGTH: usize = 7;
type TelecommandBuffer = [u8; TELECOMMAND_LENGTH];

static mut TELECOMMAND_BUFFER: TelecommandBuffer = [0; TELECOMMAND_LENGTH];

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
/// program may panic.
static mut XDMAC_COMMAND_QUEUE_HANDLE: Option<MessageQueueHandle<TelecommandBuffer, 10>> = None;

static TASK_UART_READER_STORAGE: TaskletStorage<TelecommandBuffer, TaskUartReaderContext, 0> =
    TaskletStorage::new();
static TASK_START_MEASUREMENTS_STORAGE: TaskletStorage<EventId, TaskStartMeasurementsContext, 0> =
    TaskletStorage::new();
static TASK_STOP_MEASUREMENTS_STORAGE: TaskletStorage<EventId, TaskStopMeasurementsContext, 0> =
    TaskletStorage::new();
static TASK_SET_DATA_OUTPUT_RATE_STORAGE: TaskletStorage<
    OutputDataRate,
    TaskSetDataOutputRateContext,
    0,
> = TaskletStorage::new();
static TASK_SET_ACCELEROMETER_SCALE_STORAGE: TaskletStorage<
    AccelerometerScale,
    TaskSetAccelerometerScaleContext,
    0,
> = TaskletStorage::new();
static TASK_SET_GYROSCOPE_SCALE_STORAGE: TaskletStorage<
    GyroscopeScale,
    TaskSetGyroscopeScaleContext,
    0,
> = TaskletStorage::new();
static TASK_GET_EXECUTION_STATS_STORAGE: TaskletStorage<EventId, TaskGetExecutionStatsContext, 0> =
    TaskletStorage::new();

static QUEUE_COMMAND_STORAGE: MessageQueueStorage<TelecommandBuffer, 10> =
    MessageQueueStorage::new();
static QUEUE_SET_DATA_OUTPUT_RATE_STORAGE: MessageQueueStorage<OutputDataRate, 2> =
    MessageQueueStorage::new();
static QUEUE_SET_ACCELEROMETER_SCALE_STORAGE: MessageQueueStorage<AccelerometerScale, 2> =
    MessageQueueStorage::new();
static QUEUE_SET_GYROSCOPE_SCALE_STORAGE: MessageQueueStorage<GyroscopeScale, 2> =
    MessageQueueStorage::new();

static EVENT_START_STORAGE: EventStorage = EventStorage::new();
static EVENT_STOP_STORAGE: EventStorage = EventStorage::new();
static EVENT_STATS_STORAGE: EventStorage = EventStorage::new();

#[entry]
fn main() -> ! {
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    logln!("Hello, world!");

    logln!("Initializing clocks...");
    let pmc = peripherals.pmc.unwrap();
    init_clocks(pmc);
    logln!("Clock initialized!");

    logln!("Initializing PIO...");
    let port = Port::new(peripherals.pio_d.take().unwrap());
    init_pio(port);
    logln!("PIO initialized!");

    logln!("Initializing UART...");
    let uart = Uart::new(peripherals.uart_4.take().unwrap());
    let mut uart = init_uart(uart);
    logln!("UART initialized!");

    logln!("Initializing DMA...");
    let xdmac = Xdmac::new(peripherals.xdmac.take().unwrap());
    init_xdmac(xdmac, &mut uart);
    logln!("DMA initialized!");

    logln!("Initializing NVIC...");
    let mut nvic = NVIC::new(peripherals.nvic.take().unwrap());
    nvic.enable(Interrupt::XDMAC);
    logln!("NVIC initialized!");

    logln!("Initializing the system...");
    init_system(aerugo);
    logln!("System initialized!");

    unsafe {
        XDMAC_RX_CHANNEL.as_mut().unwrap().enable();
    }

    logln!("Starting the system...");
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
        address: unsafe { TELECOMMAND_BUFFER.as_mut_ptr() as *const () },
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    let transfer_microblock_length = MicroblockLength::new(TELECOMMAND_LENGTH as u32).unwrap();

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

    aerugo.create_message_queue(&QUEUE_SET_DATA_OUTPUT_RATE_STORAGE);
    let queue_set_data_output_rate_handle =
        QUEUE_SET_DATA_OUTPUT_RATE_STORAGE.create_handle().unwrap();

    aerugo.create_message_queue(&QUEUE_SET_ACCELEROMETER_SCALE_STORAGE);
    let queue_set_accelerometer_scale_handle = QUEUE_SET_ACCELEROMETER_SCALE_STORAGE
        .create_handle()
        .unwrap();

    aerugo.create_message_queue(&QUEUE_SET_GYROSCOPE_SCALE_STORAGE);
    let queue_set_gyroscope_scale_handle =
        QUEUE_SET_GYROSCOPE_SCALE_STORAGE.create_handle().unwrap();

    // Events

    aerugo.create_event(CommandEvent::Start.into(), &EVENT_START_STORAGE);
    aerugo.create_event(CommandEvent::Stop.into(), &EVENT_STOP_STORAGE);
    aerugo.create_event(CommandEvent::GetExecutionStats.into(), &EVENT_STATS_STORAGE);

    // UART reader

    let task_uart_reader_context = TaskUartReaderContext {
        data_output_rate_queue: queue_set_data_output_rate_handle.clone(),
        accelerometer_scale_queue: queue_set_accelerometer_scale_handle.clone(),
        gyroscope_scale_queue: queue_set_gyroscope_scale_handle.clone(),
    };

    aerugo.create_tasklet_with_context(
        TaskletConfig {
            name: "UartReader",
            ..Default::default()
        },
        task_uart_reader,
        task_uart_reader_context,
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

    aerugo.subscribe_tasklet_to_events(
        &task_start_measurements_handle,
        [CommandEvent::Start.into()],
    );

    // Stop measurements

    aerugo.create_tasklet(
        TaskletConfig {
            name: "StopMeasurements",
            ..Default::default()
        },
        task_stop_measurements,
        &TASK_STOP_MEASUREMENTS_STORAGE,
    );

    let task_stop_measurements_handle = TASK_STOP_MEASUREMENTS_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_events(&task_stop_measurements_handle, [CommandEvent::Stop.into()]);

    // Set data output rate

    aerugo.create_tasklet(
        TaskletConfig {
            name: "SetDataOutputRate",
            ..Default::default()
        },
        task_set_data_output_rate,
        &TASK_SET_DATA_OUTPUT_RATE_STORAGE,
    );

    let task_set_data_output_rate_handle =
        TASK_SET_DATA_OUTPUT_RATE_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_queue(
        &task_set_data_output_rate_handle,
        &queue_set_data_output_rate_handle,
    );

    // Set accelerometer scale

    aerugo.create_tasklet(
        TaskletConfig {
            name: "SetAccelerometerScale",
            ..Default::default()
        },
        task_set_accelerometer_scale,
        &TASK_SET_ACCELEROMETER_SCALE_STORAGE,
    );

    let task_set_accelerometer_scale_handle = TASK_SET_ACCELEROMETER_SCALE_STORAGE
        .create_handle()
        .unwrap();

    aerugo.subscribe_tasklet_to_queue(
        &task_set_accelerometer_scale_handle,
        &queue_set_accelerometer_scale_handle,
    );

    // Set gyroscope scale

    aerugo.create_tasklet(
        TaskletConfig {
            name: "SetGyroscopeScale",
            ..Default::default()
        },
        task_set_gyroscope_scale,
        &TASK_SET_GYROSCOPE_SCALE_STORAGE,
    );

    let task_set_gyroscope_scale_handle = TASK_SET_GYROSCOPE_SCALE_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_queue(
        &task_set_gyroscope_scale_handle,
        &queue_set_gyroscope_scale_handle,
    );

    // Get execution stats

    aerugo.create_tasklet(
        TaskletConfig {
            name: "GetExecutionStats",
            ..Default::default()
        },
        task_get_execution_stats,
        &TASK_GET_EXECUTION_STATS_STORAGE,
    );

    let task_get_execution_stats_handle = TASK_GET_EXECUTION_STATS_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_events(
        &task_get_execution_stats_handle,
        [CommandEvent::GetExecutionStats.into()],
    );

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
        let result = XDMAC_COMMAND_QUEUE_HANDLE
            .unwrap()
            .send_data(TELECOMMAND_BUFFER);

        if result.is_err() {
            logln!("Failed to send command to the queue");
        }
    }

    rx_channel.repeat_transfer();
    rx_channel.enable();
}
