#![no_std]
#![no_main]

extern crate calldwell;

use core::fmt::Write;

use aerugo::{
    Aerugo, InitApi, MessageQueueHandle, MessageQueueStorage, RuntimeApi, SystemHardwareConfig,
    TaskletConfig, TaskletStorage,
};
use calldwell::with_rtt_out;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use lazy_static::lazy_static;

#[derive(Default)]
struct TaskAContext {}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_a(value: u32, _: &mut TaskAContext, _: &dyn RuntimeApi) {
    with_rtt_out(|w, _| write!(w.writer(), "TaskA got {}", value).unwrap());
}

static TASK_A_STORAGE: TaskletStorage<u32, TaskAContext, 1> = TaskletStorage::new();

static MESSAGE_QUEUE_STORAGE: MessageQueueStorage<u32, 10> = MessageQueueStorage::new();

#[entry]
fn main() -> ! {
    calldwell::start_session();

    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    aerugo.create_message_queue(&MESSAGE_QUEUE_STORAGE);

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };

    aerugo.create_tasklet(task_a_config, task_a, &TASK_A_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    let message_queue_handle = MESSAGE_QUEUE_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_queue(&task_a_handle, &message_queue_handle);

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
        static ref MESSAGE_QUEUE_HANDLE: MessageQueueHandle<u32, 10> =
            MESSAGE_QUEUE_STORAGE.create_handle().unwrap();
    }

    MESSAGE_QUEUE_HANDLE
        .send_data(42)
        .expect("Failed to send data to the queue");
}
