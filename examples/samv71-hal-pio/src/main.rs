#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use core::cell::RefCell;

use aerugo::hal::drivers::embedded_hal::digital::ToggleableOutputPin;
use aerugo::hal::drivers::pio::pin::InputMode;
use aerugo::hal::drivers::pio::{pin::OutputMode, Pin, Port, PIOC};
use aerugo::hal::drivers::pmc::{config::PeripheralId, PMC};
use aerugo::Mutex;

use aerugo::{
    logln, Aerugo, Duration, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};
use rt::entry;

static IO_OUT_PIN: Mutex<RefCell<Option<Pin<OutputMode>>>> = Mutex::new(RefCell::new(None));
static IO_IN_PIN: Mutex<RefCell<Option<Pin<InputMode>>>> = Mutex::new(RefCell::new(None));

fn pio_output_task(_: (), _: &mut PioTaskContext, _: &'static dyn RuntimeApi) {
    IO_OUT_PIN.lock(|pin_ref| {
        let pin = pin_ref.get_mut().as_mut().unwrap();
        logln!("Inspecting pin {:?}", pin);
        logln!("Current drive mode: {:#?}", pin.drive_mode());
        logln!("Current state: {:#?}", pin.state());

        pin.toggle().unwrap();
    })
}

fn pio_input_task(_: (), _: &mut PioTaskContext, _: &'static dyn RuntimeApi) {
    IO_IN_PIN.lock(|pin_ref| {
        let pin = pin_ref.get_mut().as_mut().unwrap();
        logln!("Inspecting pin {:?}", pin);
        logln!("Current state: {:#?}", pin.state());

        match pin.pull_resistor() {
            aerugo::hal::drivers::pio::pin::PullResistor::None => {
                logln!("Pin is not pulled down nor up");
                pin.pull_up();
            }
            aerugo::hal::drivers::pio::pin::PullResistor::Up => {
                logln!("Pin is pulled up");
                pin.pull_down();
            }
            aerugo::hal::drivers::pio::pin::PullResistor::Down => {
                logln!("Pin is pulled down");
                pin.disable_pull_resistor();
            }
        }
    })
}

#[derive(Default)]
struct PioTaskContext {}

static PIO_OUT_TASK_STORAGE: TaskletStorage<(), PioTaskContext, 0> = TaskletStorage::new();
static PIO_IN_TASK_STORAGE: TaskletStorage<(), PioTaskContext, 0> = TaskletStorage::new();

fn init_clocks(mut pmc: PMC) {
    pmc.enable_peripheral_clock(PeripheralId::PIOC);
}

fn init_pio(port: Port<PIOC>) {
    let mut pins = port.into_pins();
    let example_out_pin = pins[5].take().unwrap().into_output_pin();
    let example_in_pin = pins[10].take().unwrap().into_input_pin();

    IO_OUT_PIN.lock(|pin_ref| pin_ref.replace(Some(example_out_pin)));
    IO_IN_PIN.lock(|pin_ref| pin_ref.replace(Some(example_in_pin)));
}

fn init_tasks(aerugo: &'static impl InitApi) {
    logln!("Initializing tasks...");

    let pio_out_task_config = TaskletConfig {
        name: "PioTask",
        ..Default::default()
    };
    let pio_in_task_config = TaskletConfig {
        name: "PioTask",
        ..Default::default()
    };

    let pio_out_task_context = PioTaskContext::default();
    let pio_in_task_context = PioTaskContext::default();

    aerugo.create_tasklet_with_context(
        pio_out_task_config,
        pio_output_task,
        pio_out_task_context,
        &PIO_OUT_TASK_STORAGE,
    );
    aerugo.create_tasklet_with_context(
        pio_in_task_config,
        pio_input_task,
        pio_in_task_context,
        &PIO_IN_TASK_STORAGE,
    );

    let pio_out_task_handle = PIO_OUT_TASK_STORAGE.create_handle().unwrap();
    let pio_in_task_handle = PIO_IN_TASK_STORAGE.create_handle().unwrap();

    logln!("Subscribing tasks...");

    aerugo.subscribe_tasklet_to_cyclic(&pio_out_task_handle, Some(Duration::millis(1000)), None);
    aerugo.subscribe_tasklet_to_cyclic(&pio_in_task_handle, Some(Duration::millis(1000)), None);
}

#[entry]
fn main() -> ! {
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    logln!("Hello, world! Aerugo initialized!");

    logln!("Initializing hardware...");
    let port = Port::new(peripherals.pio_c.take().unwrap());
    let pmc = peripherals.pmc.expect("PMC already taken");
    init_clocks(pmc);
    init_pio(port);

    init_tasks(aerugo);

    logln!("Starting the system!");
    aerugo.start();
}
