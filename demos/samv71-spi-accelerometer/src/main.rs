#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use aerugo::hal::drivers::pio::{pin::Peripheral, Port};
use aerugo::hal::drivers::pmc::config::PeripheralId;
use aerugo::hal::drivers::uart::reader::Reader;
use aerugo::hal::drivers::uart::{Bidirectional, Config, NotConfigured, ReceiverConfig, Uart};
use aerugo::hal::user_peripherals::{PIOD, PMC, UART4};
use aerugo::time::RateExtU32;
use aerugo::{
    logln, Aerugo, Duration, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};
use rt::entry;

struct TaskUartReaderContext {
    reader: Reader<UART4>,
}

fn task_uart_reader(_: (), context: &mut TaskUartReaderContext, _: &'static dyn RuntimeApi) {
    match context.reader.receive_byte(1) {
        Ok(byte) => logln!("Received: {:#04x}", byte),
        Err(_) => (),
    }
}

static TASK_UART_READER_STORAGE: TaskletStorage<(), TaskUartReaderContext, 0> =
    TaskletStorage::new();

#[entry]
fn main() -> ! {
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    let pmc = peripherals.pmc.unwrap();
    init_clocks(pmc);

    let port = Port::new(peripherals.pio_d.take().unwrap());
    init_pio(port);

    let uart = Uart::new(peripherals.uart_4.take().unwrap());
    let uart_configured = init_uart(uart);

    init_tasks(aerugo, uart_configured);

    aerugo.start();
}

fn init_clocks(mut pmc: PMC) {
    pmc.enable_peripheral_clock(PeripheralId::PIOD);
    pmc.enable_peripheral_clock(PeripheralId::UART4);
}

fn init_pio(port: Port<PIOD>) {
    let mut pins = port.into_pins();
    pins[18].take().unwrap().into_peripheral_pin(Peripheral::C);
    pins[19].take().unwrap().into_peripheral_pin(Peripheral::D);
}

fn init_uart(uart: Uart<UART4, NotConfigured>) -> Uart<UART4, Bidirectional> {
    let uart_config = Config::new(9600, 12.MHz()).unwrap();
    let recv_config = ReceiverConfig {
        rx_filter_enabled: true,
    };

    uart.into_bidirectional(uart_config, recv_config)
}

fn init_tasks(aerugo: &'static impl InitApi, mut uart: Uart<UART4, Bidirectional>) {
    let reader = uart.take_reader().unwrap();

    let task_uart_reader_config = TaskletConfig {
        name: "UartReader",
        ..Default::default()
    };
    let task_uart_reader_context = TaskUartReaderContext { reader };

    aerugo.create_tasklet_with_context(
        task_uart_reader_config,
        task_uart_reader,
        task_uart_reader_context,
        &TASK_UART_READER_STORAGE,
    );

    let task_uart_reader_handle = TASK_UART_READER_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_cyclic(&task_uart_reader_handle, Some(Duration::secs(1)), None);
}
