#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]

extern crate panic_semihosting;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};

use aerugo::{Aerugo, InitApi, MessageQueueStorage, TaskletConfig, TaskletStorage};

static AERUGO: Aerugo = Aerugo::new();

#[allow(dead_code)]
struct TaskAData {
    a: u8,
    b: u32,
}

#[allow(dead_code)]
struct TaskBData {
    a: u16,
    b: u16,
}

static tasklet_a: TaskletStorage<u8, TaskAData> = TaskletStorage::new();
static tasklet_b: TaskletStorage<u8, TaskBData> = TaskletStorage::new();
static queue_x: MessageQueueStorage<u8, 16> = MessageQueueStorage::new();

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();

    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(800_000);
    syst.enable_interrupt();
    syst.enable_counter();

    AERUGO
        .create_tasklet(TaskletConfig::default(), &tasklet_a)
        .unwrap();
    let tasklet_a_handle = tasklet_a.create_task_handle().unwrap();

    AERUGO
        .create_tasklet(TaskletConfig::default(), &tasklet_b)
        .unwrap();
    let tasklet_b_handle = tasklet_b.create_task_handle().unwrap();

    AERUGO.create_message_queue(&queue_x).unwrap();
    let queue_x_handle = queue_x.create_queue_handle().unwrap();

    AERUGO
        .subscribe_tasklet_to_queue(&tasklet_a_handle, &queue_x_handle)
        .unwrap();
    AERUGO
        .subscribe_tasklet_to_queue(&tasklet_b_handle, &queue_x_handle)
        .unwrap();

    AERUGO.start_scheduler();
}

#[exception]
fn SysTick() {
    static mut data: u8 = 0;

    *data = data.overflowing_add(1).0;

    let queue_x_handle = queue_x.create_queue_handle().unwrap();
    queue_x_handle.send_data(*data).unwrap();
}
