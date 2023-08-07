use aerugo::{log, InitApi, SystemHardwareConfig, TaskletConfig, TaskletStorage, AERUGO};

#[derive(Default)]
struct TaskAContext {
    cnt: u8,
}

fn task_a(_: (), context: &mut TaskAContext) {
    if context.cnt < 2 {
        log!("TaskA");
        context.cnt += 1;
    } else {
        std::process::exit(0);
    }
}

#[derive(Default)]
struct TaskBContext {
    cnt: u8,
}

fn task_b(_: (), context: &mut TaskBContext) {
    if context.cnt < 2 {
        log!("TaskB");
        context.cnt += 1;
    } else {
        std::process::exit(0);
    }
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<(), TaskBContext, 0> = TaskletStorage::new();

fn main() -> ! {
    AERUGO.initialize(SystemHardwareConfig::default());

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };
    let task_a_context = TaskAContext { cnt: 0 };

    AERUGO
        .create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE)
        .expect("Unable to create TaskA");

    let task_b_config = TaskletConfig {
        name: "TaskB",
        ..Default::default()
    };
    let task_b_context = TaskBContext { cnt: 0 };

    AERUGO
        .create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE)
        .expect("Unable to create TaskB");

    let task_a_handle = TASK_A_STORAGE
        .create_handle()
        .expect("Unable to create handle to TaskA");
    let task_b_handle = TASK_B_STORAGE
        .create_handle()
        .expect("Unable to create handle to TaskB");

    AERUGO
        .subscribe_tasklet_to_cyclic(&task_a_handle, None)
        .expect("Unable to set cyclic on TaskA");
    AERUGO
        .subscribe_tasklet_to_cyclic(&task_b_handle, None)
        .expect("Unable to set cyclic on TaskB");

    AERUGO.start();
}
