#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use aerugo::{
    logln, time::MillisDurationU32, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage, AERUGO,
};
use rt::entry;

#[derive(Default)]
struct DummyTaskContext {
    acc: u16,
}

fn dummy_task(_: (), context: &mut DummyTaskContext, api: &'static dyn RuntimeApi) {
    context.acc = context.acc.wrapping_add(1);
    if context.acc % 300 == 0 {
        let time = api.get_system_time().duration_since_epoch();
        let time_seconds = time.to_secs();
        let time_millis = time.to_millis() % 1000;
        logln!("Current time is {}s {}ms", time_seconds, time_millis);
    }
}

static DUMMY_TASK_STORAGE: TaskletStorage<(), DummyTaskContext, 0> = TaskletStorage::new();

#[entry]
fn main() -> ! {
    AERUGO.initialize(SystemHardwareConfig {
        watchdog_timeout: MillisDurationU32::secs(5),
        ..Default::default()
    });

    logln!("Hello, world! Aerugo initialized!");

    logln!("Creating tasks...");
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

    logln!("Subscribing tasks...");

    AERUGO
        .subscribe_tasklet_to_cyclic(&dummy_task_handle, None)
        .expect("Unable to subscribe dummy task to cyclic execution!");

    logln!("Starting the system!");

    AERUGO.start();
}
