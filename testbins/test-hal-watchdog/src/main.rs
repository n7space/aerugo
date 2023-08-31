#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::{
    hal::drivers::Milliseconds, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage, AERUGO,
};
use calldwell::with_rtt_out;
use cortex_m::asm;
use rt::entry;

#[derive(Default)]
struct ShortTaskContext {
    acc: u16,
}

#[derive(Default)]
struct LongTaskContext {
    acc: u32,
}

fn short_task(_: (), context: &mut ShortTaskContext, _: &dyn RuntimeApi) {
    with_rtt_out(|w, _| w.write_str("short task started"));

    while context.acc != 1000 {
        context.acc = context.acc.wrapping_add(1);
        asm::nop();
    }

    with_rtt_out(|w, _| w.write_str("short task ended"));
}

fn long_task(_: (), context: &mut LongTaskContext, _: &dyn RuntimeApi) {
    with_rtt_out(|w, _| w.write_str("long task started"));

    while context.acc != 4000000000 {
        context.acc = context.acc.wrapping_add(1);
        asm::nop();
    }

    with_rtt_out(|w, _| w.write_str("long task ended"));
}

static SHORT_TASK_STORAGE: TaskletStorage<(), ShortTaskContext, 0> = TaskletStorage::new();
static LONG_TASK_STORAGE: TaskletStorage<(), LongTaskContext, 0> = TaskletStorage::new();

fn initialize_tasks() {
    let short_task_config = TaskletConfig {
        name: "ShortTask",
        ..Default::default()
    };

    let long_task_config = TaskletConfig {
        name: "LongTask",
        ..Default::default()
    };

    let short_task_context = ShortTaskContext::default();
    let long_task_context = LongTaskContext::default();

    AERUGO
        .create_tasklet_with_context(
            short_task_config,
            short_task,
            short_task_context,
            &SHORT_TASK_STORAGE,
        )
        .expect("Unable to create short task!");

    AERUGO
        .create_tasklet_with_context(
            long_task_config,
            long_task,
            long_task_context,
            &LONG_TASK_STORAGE,
        )
        .expect("Unable to create long task!");

    let short_task_handle = SHORT_TASK_STORAGE
        .create_handle()
        .expect("Unable to create short task handle!");

    let long_task_handle = LONG_TASK_STORAGE
        .create_handle()
        .expect("Unable to create short task handle!");

    AERUGO
        .subscribe_tasklet_to_cyclic(&short_task_handle, None)
        .expect("Unable to subscribe short task to cyclic execution!");
    AERUGO
        .subscribe_tasklet_to_cyclic(&long_task_handle, None)
        .expect("Unable to subscribe long task to cyclic execution!");
}

#[entry]
fn main() -> ! {
    calldwell::start_session();

    AERUGO.initialize(SystemHardwareConfig {
        watchdog_timeout: Milliseconds::secs(5),
        ..Default::default()
    });

    initialize_tasks();

    AERUGO.start();
}
