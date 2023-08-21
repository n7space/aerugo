#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as runtime;
extern crate panic_rtt_target;
extern crate rtt_target as rtt;

use aerugo::{
    time::MillisDurationU32, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage, AERUGO,
};
use rtt::rprintln;
use runtime::entry;

#[derive(Default)]
struct DummyTaskContext {
    acc: u16,
}

fn dummy_task(_: (), context: &mut DummyTaskContext, api: &'static dyn RuntimeApi) {
    context.acc = context.acc.wrapping_add(1);
    if context.acc % 300 == 0 {
        let time = api.get_system_time().duration_since_epoch().to_secs();
        rprintln!("Current time is {}s", time);
    }
}

static DUMMY_TASK_STORAGE: TaskletStorage<(), DummyTaskContext, 0> = TaskletStorage::new();

#[entry]
fn main() -> ! {
    rtt::rtt_init_print!();

    rprintln!("Hello, world! Initializing Aerugo...");

    AERUGO.initialize(SystemHardwareConfig {
        watchdog_timeout: MillisDurationU32::secs(5),
    });

    rprintln!("Creating tasks...");
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

    rprintln!("Subscribing tasks...");

    AERUGO
        .subscribe_tasklet_to_cyclic(&dummy_task_handle, None)
        .expect("Unable to subscribe dummy task to cyclic execution!");

    rprintln!("Starting the system!");

    AERUGO.start();
}
