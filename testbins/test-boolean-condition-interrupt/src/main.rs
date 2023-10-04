#![no_std]
#![no_main]

extern crate calldwell;

use core::fmt::Write;

use aerugo::{
    Aerugo, BooleanConditionHandle, BooleanConditionStorage, InitApi, RuntimeApi,
    SystemHardwareConfig, TaskletConfig, TaskletStorage,
};
use calldwell::with_rtt_out;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use lazy_static::lazy_static;

#[derive(Default)]
struct TaskAContext {}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_a(value: bool, _: &mut TaskAContext, _: &dyn RuntimeApi) {
    with_rtt_out(|w, _| write!(w.writer(), "TaskA got {}", value).unwrap());
}

static TASK_A_STORAGE: TaskletStorage<bool, TaskAContext, 1> = TaskletStorage::new();

static DONE_CONDITION_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

#[entry]
fn main() -> ! {
    calldwell::start_session();

    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    aerugo.create_boolean_condition(false, &DONE_CONDITION_STORAGE);

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };

    aerugo.create_tasklet(task_a_config, task_a, &TASK_A_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    let done_condition_handle = DONE_CONDITION_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_condition(&task_a_handle, &done_condition_handle);

    let mut systick = peripherals.systick.take().unwrap();
    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(0xBEEF);
    systick.enable_interrupt();
    systick.enable_counter();

    aerugo.start();
}

#[exception]
fn SysTick() {
    lazy_static! {
        static ref DONE_CONDITION_HANDLE: BooleanConditionHandle =
            DONE_CONDITION_STORAGE.create_handle().unwrap();
    }

    let current_value = DONE_CONDITION_HANDLE.get_value();
    with_rtt_out(|w, _| write!(w.writer(), "Value in the interrupt: {}", current_value).unwrap());

    DONE_CONDITION_HANDLE.set_value(true);
}
