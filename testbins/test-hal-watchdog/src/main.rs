#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use aerugo::{
    time::MillisDurationU32, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage, AERUGO,
};
use calldwell::with_rtt_out;
use core::fmt::Write;
use cortex_m::asm;
use rt::entry;

// Test scenario:
// - Configure Aerugo with watchdog that will reset the MCU after 3 seconds
// - Execute a task that will run shorter than 3 seconds and send a message to host
// - Execute a task that will run longer than 3 seconds
// - Validate that MCU has rebooted

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
    calldwell::initialize();
    wait_for_host();

    AERUGO.initialize(SystemHardwareConfig {
        watchdog_timeout: MillisDurationU32::secs(3),
    });

    initialize_tasks();

    AERUGO.start();
}

fn wait_for_host() {
    let mut input_buffer: [u8; 128] = [0; 128];

    calldwell::with_rtt_out(|w, _| w.write_str("mcu ready"));
    let response_length = calldwell::with_rtt_in(|r, _| r.read(&mut input_buffer));

    if let Err(e) = response_length {
        calldwell::with_rtt_out(|w, _| {
            write!(
                w.writer(),
                "an error occurred while receiving response from host: {:?}",
                e
            )
            .expect("Unable to send data via RTT")
        });
    }
}
