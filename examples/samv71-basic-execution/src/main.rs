#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use aerugo::{
    time::MillisDurationU32, InitApi, SystemHardwareConfig, TaskletConfig, TaskletStorage, AERUGO,
};
use cortex_m_semihosting::hprintln;
use rt::entry;

#[derive(Default)]
struct DummyTaskContext {
    acc: u16,
}

fn dummy_task(_: (), context: &mut DummyTaskContext) {
    context.acc = context.acc.wrapping_add(1);
    if context.acc % 250 == 0 {
        hprintln!("I'm running!");
    }
}

static DUMMY_TASK_STORAGE: TaskletStorage<(), DummyTaskContext> = TaskletStorage::new();

#[entry]
fn main() -> ! {
    hprintln!("Hello, world! Initializing Aerugo...");

    AERUGO.initialize(SystemHardwareConfig {
        watchdog_timeout: MillisDurationU32::secs(5),
    });

    hprintln!("Creating tasks...");
    let dummy_task_config = TaskletConfig {
        name: "DummyTask",
        ..Default::default()
    };
    let dummy_task_context = DummyTaskContext::default();

    AERUGO
        .create_tasklet_with_context(
            dummy_task_config,
            dummy_task,
            dummy_task_context,
            &DUMMY_TASK_STORAGE,
        )
        .expect("Unable to create dummy task!");

    let dummy_task_handle = DUMMY_TASK_STORAGE
        .create_handle()
        .expect("Unable to create handle to dummy task!");

    hprintln!("Subscribing tasks...");

    AERUGO
        .subscribe_tasklet_to_cyclic(&dummy_task_handle, None)
        .expect("Unable to subscribe dummy task to cyclic execution!");

    hprintln!("Starting the system!");

    AERUGO.start();
}
