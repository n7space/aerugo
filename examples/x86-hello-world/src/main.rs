#![allow(non_upper_case_globals)]

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

fn main() -> ! {
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
