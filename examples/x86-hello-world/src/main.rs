#![allow(non_upper_case_globals)]

use aerugo::{InitApi, MessageQueueStorage, TaskletConfig, TaskletStorage, AERUGO};

#[allow(dead_code)]
#[derive(Default)]
struct TaskAData {
    a: u8,
    b: u32,
}

fn task_a(_: u8, _: &mut TaskAData) {}

#[allow(dead_code)]
#[derive(Default)]
struct TaskBData {
    a: u16,
    b: u16,
}

fn task_b(_: u8, _: &mut TaskBData) {}

static TASK_A_STORAGE: TaskletStorage<u8, TaskAData> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<u8, TaskBData> = TaskletStorage::new();
static QUEUE_X: MessageQueueStorage<u8, 16> = MessageQueueStorage::new();

fn main() -> ! {
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
