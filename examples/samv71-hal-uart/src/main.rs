#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use aerugo::hal::drivers::pio::{pin::Peripheral, Port};
use aerugo::hal::drivers::pmc::config::PeripheralId;
use aerugo::hal::drivers::uart::{Bidirectional, Config, NotConfigured, ReceiverConfig, UART};
use aerugo::hal::user_peripherals::{PIOD, PMC, UART4};
use aerugo::time::RateExtU32;
use aerugo::{
    logln, Aerugo, Duration, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};
use rt::entry;

fn uart_task(_: (), context: &mut UartTaskContext, _: &'static dyn RuntimeApi) {
    let uart = &mut context.uart;
    uart.transmit_byte(context.byte_to_transmit, 1_000_000)
        .unwrap();
    let received_byte = uart.receive_byte(1_000_000).unwrap();
    logln!(
        "Transmitted {:#02X}, received {:#02X}",
        context.byte_to_transmit,
        received_byte
    );

    context.byte_to_transmit = context.byte_to_transmit.wrapping_add(1);
}

struct UartTaskContext {
    pub uart: UART<UART4, Bidirectional>,
    pub byte_to_transmit: u8,
}

static UART_TASK_STORAGE: TaskletStorage<(), UartTaskContext, 0> = TaskletStorage::new();

fn init_clocks(mut pmc: PMC) {
    pmc.enable_peripheral_clock(PeripheralId::PIOD);
    pmc.enable_peripheral_clock(PeripheralId::UART4);
}

fn init_pio(port: Port<PIOD>) {
    let mut pins = port.into_pins();
    pins[18].take().unwrap().into_peripheral_pin(Peripheral::C);
    pins[19].take().unwrap().into_peripheral_pin(Peripheral::C);
}

fn init_uart(uart: UART<UART4, NotConfigured>) -> UART<UART4, Bidirectional> {
    let mut uart = uart.into_bidirectional(
        Config::new(9600, 12.MHz()).unwrap(),
        ReceiverConfig {
            rx_filter_enabled: true,
        },
    );

    uart.switch_to_local_loopback_mode();
    uart
}

fn init_tasks(aerugo: &'static impl InitApi, uart: UART<UART4, Bidirectional>) {
    logln!("Initializing tasks...");

    let uart_task_config = TaskletConfig {
        name: "UartTask",
        ..Default::default()
    };

    let uart_task_context = UartTaskContext {
        uart,
        byte_to_transmit: 0,
    };

    aerugo.create_tasklet_with_context(
        uart_task_config,
        uart_task,
        uart_task_context,
        &UART_TASK_STORAGE,
    );

    let uart_task_handle = UART_TASK_STORAGE.create_handle().unwrap();

    logln!("Subscribing tasks...");

    aerugo.subscribe_tasklet_to_cyclic(&uart_task_handle, Some(Duration::secs(1)), None);
}

#[entry]
fn main() -> ! {
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    logln!("Hello, world! Aerugo initialized! Initializing hardware...");

    let port = Port::new(peripherals.pio_d.take().unwrap());
    let pmc = peripherals.pmc.unwrap();
    let uart = UART::new(peripherals.uart_4.take().unwrap());

    init_clocks(pmc);
    init_pio(port);
    init_tasks(aerugo, init_uart(uart));

    logln!("Starting the system!");
    aerugo.start();
}
