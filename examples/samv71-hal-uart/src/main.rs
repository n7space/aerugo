#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use core::cell::RefCell;

use aerugo::Mutex;

use aerugo::hal::drivers::pio::{pin::Peripheral, Port};
use aerugo::hal::drivers::pmc::config::PeripheralId;
use aerugo::hal::drivers::uart::{Frequency, Mode, UART};
use aerugo::hal::user_peripherals::{PIOD, PMC, UART4};
use aerugo::{
    logln, Aerugo, Duration, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};
use rt::entry;

static UART: Mutex<RefCell<Option<UART<UART4>>>> = Mutex::new(RefCell::new(None));

fn uart_task(_: (), context: &mut UartTaskContext, _: &'static dyn RuntimeApi) {
    UART.lock(|uart| {
        let uart = uart.get_mut().as_mut().unwrap();
        uart.transmit_byte(context.byte_to_transmit, 1_000_000)
            .unwrap();
        let received_byte = uart.receive_byte(1_000_000).unwrap();
        logln!(
            "Transmitted {:#02X}, received {:#02X}",
            context.byte_to_transmit,
            received_byte
        );
    });

    context.byte_to_transmit = context.byte_to_transmit.wrapping_add(1);
}

#[derive(Default)]
struct UartTaskContext {
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

fn init_uart(mut uart: UART<UART4>) {
    uart.set_mode(Mode::LocalLoopback);
    uart.set_baudrate(12000, Frequency::MHz(12)).unwrap();
    uart.enable_receiver();
    uart.enable_transmitter();
    uart.reset_status();

    UART.lock(|uart_ref| uart_ref.replace(Some(uart)));
}

fn init_tasks(aerugo: &'static impl InitApi) {
    logln!("Initializing tasks...");

    let uart_task_config = TaskletConfig {
        name: "UartTask",
        ..Default::default()
    };

    let uart_task_context = UartTaskContext::default();

    aerugo.create_tasklet_with_context(
        uart_task_config,
        uart_task,
        uart_task_context,
        &UART_TASK_STORAGE,
    );

    let uart_task_handle = UART_TASK_STORAGE.create_handle().unwrap();

    logln!("Subscribing tasks...");

    aerugo.subscribe_tasklet_to_cyclic(&uart_task_handle, Some(Duration::secs(1)));
}

#[entry]
fn main() -> ! {
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    logln!("Hello, world! Aerugo initialized!");

    logln!("Initializing hardware...");
    let port = Port::new(peripherals.pio_d.take().unwrap());
    let pmc = peripherals.pmc.unwrap();
    let uart = UART::new(peripherals.uart_4.take().unwrap());
    init_clocks(pmc);
    init_pio(port);
    init_uart(uart);

    init_tasks(aerugo);

    logln!("Starting the system!");
    aerugo.start();
}
