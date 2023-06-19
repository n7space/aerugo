#![allow(non_upper_case_globals)]

use aerugo::{InitApi, TaskletConfig, TaskletStorage, AERUGO, logln};

static TASK_A: TaskletStorage<u8, ()> = TaskletStorage::new();

fn main() -> ! {
    let task_a_config = TaskletConfig { name: "TaskA" };
    AERUGO.create_tasklet(task_a_config, &TASK_A).expect("Unable to create TaskA");

    let task_a = TASK_A.create_handle().expect("Unable to create TaskA handle");
    logln!("{}", task_a.get_name());

    loop {

    }

    // AERUGO.start();
}
