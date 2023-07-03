#![allow(non_upper_case_globals)]

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

static TASK_A: TaskletStorage<u8, TaskAData> = TaskletStorage::new();
static TASK_B: TaskletStorage<u8, TaskBData> = TaskletStorage::new();
static QUEUE_X: MessageQueueStorage<u8, 16> = MessageQueueStorage::new();

fn main() -> ! {
    AERUGO
        .create_tasklet(TaskletConfig::default(), task_a, &TASK_A)
        .unwrap();
    let tasklet_a_handle = TASK_A.create_handle().unwrap();

    AERUGO
        .create_tasklet(TaskletConfig::default(), task_b, &TASK_B)
        .unwrap();
    let tasklet_b_handle = TASK_B.create_handle().unwrap();

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
