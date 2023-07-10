#![allow(non_upper_case_globals)]

use aerugo::{log, InitApi, TaskletConfig, TaskletStorage, AERUGO};

static TASK_A_STORAGE: TaskletStorage<u8, ()> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<u16, ()> = TaskletStorage::new();

fn task_a(data: u8) {
    log!("TaskA: {}", data);
}

fn task_b(data: u16) {
    log!("TaskB: {}", data);
}

fn main() -> ! {
    let task_a_config = TaskletConfig { name: "TaskA" };
    AERUGO
        .create_tasklet(task_a_config, task_a, &TASK_A_STORAGE)
        .expect("Unable to create TaskA");

    let task_b_config = TaskletConfig { name: "TaskB" };
    AERUGO
        .create_tasklet(task_b_config, task_b, &TASK_B_STORAGE)
        .expect("Unable to create TaskB");

    AERUGO.start();
}
