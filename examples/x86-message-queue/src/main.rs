#![allow(non_upper_case_globals)]

use aerugo::{log, InitApi, MessageQueueStorage, TaskletConfig, TaskletStorage, AERUGO};

#[derive(Default)]
struct TaskAContext;

fn task_a(data: u8, _: &mut TaskAContext) {
    log!("TaskA: {}", data)
}

static TASK_A: TaskletStorage<u8, TaskAContext> = TaskletStorage::new();

static QUEUE_X: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();

fn main() -> ! {
    let task_a_config = TaskletConfig { name: "TaskA" };

    AERUGO
        .create_tasklet(task_a_config, task_a, &TASK_A)
        .expect("Unable to create TaskA");

    AERUGO
        .create_message_queue(&QUEUE_X)
        .expect("Unable to create QueueX");

    let task_a_handle = TASK_A
        .create_handle()
        .expect("Unable to create TaskA handle");
    let queue_x_handle = QUEUE_X
        .create_handle()
        .expect("Unable to create QueueX handle");

    AERUGO
        .subscribe_tasklet_to_queue(&task_a_handle, &queue_x_handle)
        .expect("Unable to subscribe TaskA to QueueX");

    queue_x_handle.send_data(0).expect("Unable to send data to QueueX");
    queue_x_handle.send_data(1).expect("Unable to send data to QueueX");
    queue_x_handle.send_data(2).expect("Unable to send data to QueueX");

    AERUGO.start();
}
