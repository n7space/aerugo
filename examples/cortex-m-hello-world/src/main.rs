#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};

use aerugo::{InitApi, MessageQueueStorage, TaskletConfig, TaskletStorage, AERUGO};

#[allow(dead_code)]
struct TaskAData {
    a: u8,
    b: u32,
}

fn task_a(_: u8) {}

#[allow(dead_code)]
struct TaskBData {
    a: u16,
    b: u16,
}

fn task_b(_: u8) {}

static TASK_A_STORAGE: TaskletStorage<u8, TaskAData> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<u8, TaskBData> = TaskletStorage::new();
static QUEUE_X: MessageQueueStorage<u8, 16> = MessageQueueStorage::new();

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();

    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(800_000);
    syst.enable_interrupt();
    syst.enable_counter();

    AERUGO
        .create_tasklet(TaskletConfig::default(), task_a, &TASK_A_STORAGE)
        .unwrap();
    let tasklet_a_handle = TASK_A_STORAGE.create_handle().unwrap();

    AERUGO
        .create_tasklet(TaskletConfig::default(), task_b, &TASK_B_STORAGE)
        .unwrap();
    let tasklet_b_handle = TASK_B_STORAGE.create_handle().unwrap();

    AERUGO.create_message_queue(&QUEUE_X).unwrap();
    let queue_x_handle = QUEUE_X.create_queue_handle().unwrap();

    AERUGO
        .subscribe_tasklet_to_queue(&tasklet_a_handle, &queue_x_handle)
        .unwrap();
    AERUGO
        .subscribe_tasklet_to_queue(&tasklet_b_handle, &queue_x_handle)
        .unwrap();

    AERUGO.start()
}

#[exception]
fn SysTick() {
    static mut DATA: u8 = 0;

    *DATA = DATA.overflowing_add(1).0;

    let queue_x_handle = QUEUE_X.create_queue_handle().unwrap();
    queue_x_handle.send_data(*DATA).unwrap();
}
