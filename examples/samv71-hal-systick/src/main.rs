#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use core::cell::RefCell;

use aerugo::Mutex;

use aerugo::{
    logln, Aerugo, Duration, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};
use cortex_m::peripheral::syst::SystClkSource;
use rt::{entry, exception};

static COUNTER: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

#[derive(Default)]
struct DummyTaskContext {}

static DUMMY_TASK_STORAGE: TaskletStorage<(), DummyTaskContext, 0> = TaskletStorage::new();

fn dummy_task(_: (), _: &mut DummyTaskContext, _: &'static dyn RuntimeApi) {
    COUNTER.lock(|value_ref| logln!("SysTick IRQ happened {} times.", value_ref.borrow()));
}

fn init_tasks(aerugo: &'static impl InitApi) {
    let dummy_task_config = TaskletConfig {
        name: "DummyTask",
        ..Default::default()
    };
    let dummy_task_context = DummyTaskContext::default();

    aerugo.create_tasklet_with_context(
        dummy_task_config,
        dummy_task,
        dummy_task_context,
        &DUMMY_TASK_STORAGE,
    );

    let dummy_task_handle = DUMMY_TASK_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_cyclic(&dummy_task_handle, Some(Duration::secs(1)));
}

#[entry]
fn main() -> ! {
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    let mut systick = peripherals.systick.take().unwrap();

    init_tasks(aerugo);

    // Configure SysTick to trigger interrupts with arbitrary frequency using core clock as source.
    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(0xBEEF);
    systick.enable_interrupt();
    systick.enable_counter();

    aerugo.start();
}

#[exception]
fn SysTick() {
    COUNTER.lock(|counter_ref| {
        let counter = counter_ref.get_mut();
        *counter += 1;
    });
}
