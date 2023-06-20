#![allow(non_upper_case_globals)]

use aerugo::{InitApi, TaskletConfig, TaskletStorage, AERUGO};

static TASK_A: TaskletStorage<u8, ()> = TaskletStorage::new();

fn main() -> ! {
    let task_a_config = TaskletConfig { name: "TaskA" };
    AERUGO.create_tasklet(task_a_config, &TASK_A).expect("Unable to create TaskA");

    AERUGO.start();
}
