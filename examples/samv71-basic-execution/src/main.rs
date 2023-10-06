#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use aerugo::{
    logln, Aerugo, Duration, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};
use rt::entry;

#[derive(Default)]
struct DummyTaskContext {}

fn dummy_task(_: (), _: &mut DummyTaskContext, _: &dyn RuntimeApi) {
    logln!("I'm running!");
}

static DUMMY_TASK_STORAGE: TaskletStorage<(), DummyTaskContext, 0> = TaskletStorage::new();

#[entry]
fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    logln!("Hello, world! Aerugo initialized!");

    logln!("Creating tasks...");
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

    logln!("Subscribing tasks...");

    aerugo.subscribe_tasklet_to_cyclic(&dummy_task_handle, Some(Duration::secs(1)), None);

    logln!("Starting the system!");

    aerugo.start();
}
