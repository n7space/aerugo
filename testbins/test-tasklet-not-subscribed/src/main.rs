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

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();

fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };
    let task_a_context = TaskAContext { cnt: 0 };

    aerugo.create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE);

    aerugo.start();
}
