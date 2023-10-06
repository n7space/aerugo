#![no_std]
#![no_main]

extern crate calldwell;

use core::fmt::Write;

use aerugo::{
    Aerugo, Duration, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig, TaskletStorage,
};
use calldwell::with_rtt_out;
use cortex_m_rt::entry;

#[derive(Default)]
struct TaskAContext {}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_a(_: (), _: &mut TaskAContext, api: &'static dyn RuntimeApi) {
    let current_time = api.get_system_time().duration_since_epoch().to_secs();

    with_rtt_out(|w, _| write!(w.writer(), "TaskA executed at {current_time}s").unwrap());
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();

#[entry]
fn main() -> ! {
    calldwell::start_session();

    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };

    aerugo.create_tasklet(task_a_config, task_a, &TASK_A_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_cyclic(
        &task_a_handle,
        Some(Duration::secs(1)),
        Some(Duration::secs(2)),
    );

    aerugo.start();
}
