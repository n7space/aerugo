use aerugo::{
    log, time::MillisDurationU32, InitApi, SystemHardwareConfig, TaskletConfig, TaskletStorage,
    AERUGO,
};

#[derive(Default)]
struct TaskAContext {
    acc: u8,
}

fn task_a(_: (), context: &mut TaskAContext) {
    context.acc = context.acc.wrapping_add(1);
    log!("TaskA: {}", context.acc);
}

#[derive(Default)]
struct TaskBContext {
    acc: u16,
}

fn task_b(_: (), context: &mut TaskBContext) {
    context.acc = context.acc.wrapping_add(2);
    log!("TaskB: {}", context.acc);
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<(), TaskBContext> = TaskletStorage::new();

fn main() -> ! {
    AERUGO.initialize(SystemHardwareConfig {
        watchdog_timeout: MillisDurationU32::secs(5),
    });

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };

    AERUGO
        .create_tasklet(task_a_config, task_a, &TASK_A_STORAGE)
        .expect("Unable to create TaskA");

    let task_b_config = TaskletConfig {
        name: "TaskB",
        ..Default::default()
    };
    let task_b_context = TaskBContext { acc: 0 };

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
