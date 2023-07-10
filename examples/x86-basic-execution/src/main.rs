#![allow(non_upper_case_globals)]

use aerugo::{InitApi, TaskletConfig, TaskletStorage, AERUGO};

static TASK_A: TaskletStorage<u8, ()> = TaskletStorage::new();
static TASK_B: TaskletStorage<u16, ()> = TaskletStorage::new();

fn main() -> ! {
    let task_a_config = TaskletConfig { name: "TaskA" };
    AERUGO.create_tasklet(task_a_config, &TASK_A).expect("Unable to create TaskA");

    let task_b_config = TaskletConfig { name: "TaskB" };
    AERUGO.create_tasklet(task_b_config, &TASK_B).expect("Unable to create TaskB");

    AERUGO.start();
}
