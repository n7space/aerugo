use aerugo::{log, InitApi, TaskletConfig, TaskletStorage, AERUGO};

#[derive(Default)]
struct TaskAContext {
    acc: u8,
}

fn task_a(data: u8, context: &mut TaskAContext) {
    context.acc = context.acc.wrapping_add(1);
    log!("TaskA: {} / {}", data, context.acc);
}

#[derive(Default)]
struct TaskBContext {
    acc: u16,
}

fn task_b(data: u16, context: &mut TaskBContext) {
    context.acc = context.acc.wrapping_add(2);
    log!("TaskB: {} / {}", data, context.acc);
}

static TASK_A_STORAGE: TaskletStorage<u8, TaskAContext> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<u16, TaskBContext> = TaskletStorage::new();

fn main() -> ! {
    let task_a_config = TaskletConfig { name: "TaskA" };

    AERUGO
        .create_tasklet(task_a_config, task_a, &TASK_A_STORAGE)
        .expect("Unable to create TaskA");

    let task_b_config = TaskletConfig { name: "TaskB" };
    let task_b_context = TaskBContext { acc: 0 };

    AERUGO
        .create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE)
        .expect("Unable to create TaskB");

    AERUGO.start();
}
