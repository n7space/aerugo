#![allow(non_upper_case_globals)]

use aerugo::{InitApi, MessageQueueStorage, TaskletConfig, TaskletStorage, AERUGO};

#[derive(Default)]
struct TaskAContext;

fn task_a(_: u8, _: &mut TaskAContext) {}

#[derive(Default)]
struct TaskBContext;

fn task_b(_: u16, _: &mut TaskBContext) {}

static TASK_A: TaskletStorage<u8, TaskAContext> = TaskletStorage::new();
static TASK_B: TaskletStorage<u16, TaskBContext> = TaskletStorage::new();

static QUEUE_X: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();

fn main() -> ! {
    let task_a_config = TaskletConfig { name: "TaskA" };

    AERUGO
        .create_tasklet(task_a_config, task_a, &TASK_A)
        .expect("Unable to create TaskA");

    let task_b_config = TaskletConfig { name: "TaskB" };

    AERUGO
        .create_tasklet(task_b_config, task_b, &TASK_B)
        .expect("Unable to create TaskB");

    AERUGO
        .create_message_queue(&QUEUE_X)
        .expect("Unable to create QueueX");

    let queue_x_handle = QUEUE_X.create_handle().expect("Unable to create QueueX handle");

    AERUGO.start();
}
