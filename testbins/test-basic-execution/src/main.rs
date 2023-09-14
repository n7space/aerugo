use aerugo::{
    logln, Aerugo, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig, TaskletStorage,
};

#[derive(Default)]
struct TaskAContext {
    cnt: u8,
}

fn task_a(_: (), context: &mut TaskAContext, _: &dyn RuntimeApi) {
    if context.cnt < 2 {
        logln!("TaskA");
        context.cnt += 1;
    } else {
        std::process::exit(0);
    }
}

#[derive(Default)]
struct TaskBContext {
    cnt: u8,
}

fn task_b(_: (), context: &mut TaskBContext, _: &dyn RuntimeApi) {
    if context.cnt < 2 {
        logln!("TaskB");
        context.cnt += 1;
    } else {
        std::process::exit(0);
    }
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<(), TaskBContext, 0> = TaskletStorage::new();

fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };
    let task_a_context = TaskAContext { cnt: 0 };

    aerugo.create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE);

    let task_b_config = TaskletConfig {
        name: "TaskB",
        ..Default::default()
    };
    let task_b_context = TaskBContext { cnt: 0 };

    aerugo.create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    let task_b_handle = TASK_B_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_cyclic(&task_a_handle, None);
    aerugo.subscribe_tasklet_to_cyclic(&task_b_handle, None);

    aerugo.start();
}
