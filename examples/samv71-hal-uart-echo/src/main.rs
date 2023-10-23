//! This example shows a simple implementation of UART echo server.
//!
//! It uses UART4 and configures it to work in bidirectional mode with
//! reception interrupts enabled. Interrupt handler passes data from UART
//! to message queue, which is consumed by echo task that re-transmits received
//! data.
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate critical_section;
extern crate lazy_static;
extern crate panic_rtt_target;

use aerugo::hal::drivers::nvic::{Interrupt, NVIC};
use aerugo::hal::drivers::pio::{pin::Peripheral, Port};
use aerugo::hal::drivers::pmc::config::PeripheralId;
use aerugo::hal::drivers::uart::reader::Reader;
use aerugo::hal::drivers::uart::writer::Writer;
use aerugo::hal::drivers::uart::{
    Bidirectional, Config, Interrupt as UartInterrupt, NotConfigured, ReceiverConfig, Uart,
};
use aerugo::hal::interrupt;
use aerugo::hal::user_peripherals::{PIOD, PMC, UART4};
use aerugo::time::RateExtU32;
use aerugo::{
    log, logln, Aerugo, InitApi, MessageQueueHandle, MessageQueueStorage, RuntimeApi,
    SystemHardwareConfig, TaskletConfig, TaskletStorage,
};
use lazy_static::lazy_static;
use rt::entry;

/// This storage is dedicated to UART IRQ handler.
/// It must be initialized before enabling UART IRQ.
/// This storage can be safely accessed outside of UART IRQ code only when UART IRQ is disabled.
static mut UART_READER_STORAGE: Option<Reader<UART4>> = None;

const UART_DATA_QUEUE_SIZE: usize = 128;
static UART_DATA_QUEUE: MessageQueueStorage<u8, UART_DATA_QUEUE_SIZE> = MessageQueueStorage::new();

static ECHO_TASK_STORAGE: TaskletStorage<u8, EchoTaskContext, 0> = TaskletStorage::new();

struct EchoTaskContext {
    pub writer: Writer<UART4>,
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn echo_task(byte: u8, context: &mut EchoTaskContext, _: &'static dyn RuntimeApi) {
    match context.writer.transmit_byte(byte, 1000) {
        Ok(_) => {
            log!("{}", byte as char);
        }
        Err(_) => {
            logln!("Writer timeout!");
        }
    }
}

fn init_clocks(mut pmc: PMC) {
    pmc.enable_peripheral_clock(PeripheralId::PIOD);
    pmc.enable_peripheral_clock(PeripheralId::UART4);
}

fn init_pio(port: Port<PIOD>) {
    let mut pins = port.into_pins();
    pins[18].take().unwrap().into_peripheral_pin(Peripheral::C);
    pins[19].take().unwrap().into_peripheral_pin(Peripheral::C);
}

fn init_uart(uart: Uart<UART4, NotConfigured>) -> Uart<UART4, Bidirectional> {
    let mut uart = uart.into_bidirectional(
        Config::new(57600, 12.MHz()).unwrap(),
        ReceiverConfig {
            rx_filter_enabled: true,
        },
    );

    uart.enable_interrupt(UartInterrupt::RxReady);
    uart.enable_interrupt(UartInterrupt::OverrunError);
    uart
}

fn init_nvic(mut nvic: NVIC) {
    nvic.enable(Interrupt::UART4);
}

fn init_system(aerugo: &'static impl InitApi, mut uart: Uart<UART4, Bidirectional>) {
    logln!("Initializing queues...");

    aerugo.create_message_queue(&UART_DATA_QUEUE);
    let queue_handle = UART_DATA_QUEUE.create_handle().unwrap();

    logln!("Initializing tasklets...");

    let echo_task_config = TaskletConfig {
        name: "EchoTask",
        ..Default::default()
    };

    let echo_task_context = EchoTaskContext {
        writer: uart.take_writer().unwrap(),
    };

    aerugo.create_tasklet_with_context(
        echo_task_config,
        echo_task,
        echo_task_context,
        &ECHO_TASK_STORAGE,
    );

    let echo_task_handle = ECHO_TASK_STORAGE.create_handle().unwrap();

    logln!("Subscribing tasks...");

    aerugo.subscribe_tasklet_to_queue(&echo_task_handle, &queue_handle);
}

#[entry]
fn main() -> ! {
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    logln!("Hello, world! Aerugo initialized! Initializing hardware...");

    let port = Port::new(peripherals.pio_d.take().unwrap());
    let pmc = peripherals.pmc.unwrap();
    let uart = Uart::new(peripherals.uart_4.take().unwrap());
    let nvic = NVIC::new(peripherals.nvic.take().unwrap());

    init_clocks(pmc);
    init_pio(port);
    let mut uart = init_uart(uart);

    // Store reader before NVIC is initialized, otherwise the interrupt handler may execute and
    // panic.
    // Safety: This is safe, as IRQs are not yet initialized.
    unsafe { UART_READER_STORAGE.replace(uart.take_reader().unwrap()) };

    init_nvic(nvic);
    init_system(aerugo, uart);

    logln!("Starting the system!");
    aerugo.start();
}

#[interrupt]
fn UART4() {
    lazy_static! {
        static ref QUEUE_HANDLE: MessageQueueHandle<u8, UART_DATA_QUEUE_SIZE> =
            UART_DATA_QUEUE.create_handle().unwrap();
    };

    // Safety: NVIC must enable interrupts after UART_READER_STORAGE is initialized, otherwise this
    // IRQ handler will panic after failed unwrap.
    let reader = unsafe { UART_READER_STORAGE.as_mut().unwrap() };

    let status = reader.status();
    reader.reset_status();

    if status.receiver_ready {
        QUEUE_HANDLE
            .send_data(reader.receive_byte(100).unwrap())
            .unwrap();
    }

    if status.overrun_error {
        logln!("OVERRUN!");
    }
}
