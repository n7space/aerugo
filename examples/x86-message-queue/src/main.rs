#![allow(non_upper_case_globals)]

use aerugo::{
    log, InitApi, MessageQueueHandle, MessageQueueStorage, TaskletConfig, TaskletStorage, AERUGO,
};

struct TaskAContext {
    queue_handle: MessageQueueHandle<u8, 10>,
}

struct TaskBContext {
    queue_handle: MessageQueueHandle<u8, 10>,
}

fn task_a(data: u8, context: &mut TaskAContext) {
    log!("TaskA: {}", data);
    context
        .queue_handle
        .send_data(data.wrapping_add(1))
        .expect("Unable to send data from TaskA");
}

fn task_b(data: u8, context: &mut TaskBContext) {
    log!("TaskB: {}", data);
    context
        .queue_handle
        .send_data(data.wrapping_add(1))
        .expect("Unable to send data from TaskB");
}

static TASK_A_STORAGE: TaskletStorage<u8, TaskAContext> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<u8, TaskBContext> = TaskletStorage::new();

static QUEUE_X: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();

fn main() -> ! {
    AERUGO
        .create_message_queue(&QUEUE_X)
        .expect("Unable to create QueueX");

    let queue_x_handle = QUEUE_X
        .create_handle()
        .expect("Unable to create QueueX handle");

    let task_a_config = TaskletConfig { name: "TaskA" };
    let task_b_config = TaskletConfig { name: "TaskB" };

    let task_a_context = TaskAContext {
        queue_handle: queue_x_handle,
    };
    let task_b_context = TaskBContext {
        queue_handle: queue_x_handle,
    };

    AERUGO
        .create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE)
        .expect("Unable to create TaskA");
    AERUGO
        .create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE)
        .expect("Unable to create TaskB");

    let task_a_handle = TASK_A_STORAGE
        .create_handle()
        .expect("Unable to create TaskA handle");
    let task_b_handle = TASK_B_STORAGE
        .create_handle()
        .expect("Unable to create TaskB handle");

    AERUGO
        .subscribe_tasklet_to_queue(&task_a_handle, &queue_x_handle)
        .expect("Unable to subscribe TaskA to QueueX");
    AERUGO
        .subscribe_tasklet_to_queue(&task_b_handle, &queue_x_handle)
        .expect("Unable to subscribe TaskB to QueueX");

    queue_x_handle
        .send_data(0)
        .expect("Unable to send data to QueueX");

    AERUGO.start();
}
