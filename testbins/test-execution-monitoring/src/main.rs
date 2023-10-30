use aerugo::{
    logln, Aerugo, Duration, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};

#[derive(Default)]
struct MonitorContext {
    cnt: u8,
}

fn monitor(_: (), context: &mut MonitorContext, api: &'static dyn RuntimeApi) {
    for tasklet_id in api.query_tasklets() {
        let execution_stats = api.get_execution_statistics(tasklet_id);

        if let Some(stats) = execution_stats {
            logln!("Tasklet: #{}", stats.tasklet_id());

            if let Some(min) = stats.minimum_execution_time() {
                logln!("Min: {}", min.to_millis());
            }
            if let Some(max) = stats.maximum_execution_time() {
                logln!("Max: {}", max.to_millis());
            }
            if let Some(avg) = stats.average_execution_time() {
                logln!("Avg: {}", avg.to_millis());
            }
        }
    }

    context.cnt = context.cnt.wrapping_add(1);
    if context.cnt == 2 {
        std::process::exit(0);
    }
}

#[derive(Default)]
struct TaskAContext {
    cnt: u8,
}

fn task_a(_: (), context: &mut TaskAContext, _: &'static dyn RuntimeApi) {
    context.cnt = context.cnt.wrapping_add(1);

    let sleep_time = if context.cnt % 2 == 0 {
        std::time::Duration::from_millis(10)
    } else {
        std::time::Duration::from_millis(20)
    };

    std::thread::sleep(sleep_time);
}

#[derive(Default)]
struct TaskBContext {
    cnt: u8,
}

fn task_b(_: (), context: &mut TaskBContext, _: &'static dyn RuntimeApi) {
    context.cnt = context.cnt.wrapping_add(1);

    let sleep_time = if context.cnt % 2 == 0 {
        std::time::Duration::from_millis(20)
    } else {
        std::time::Duration::from_millis(50)
    };

    std::thread::sleep(sleep_time);
}

static MONITOR_STORAGE: TaskletStorage<(), MonitorContext, 0> = TaskletStorage::new();
static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<(), TaskBContext, 0> = TaskletStorage::new();

fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    let task_a_config = TaskletConfig {
        name: "TaskA",
        priority: 1,
    };
    let task_a_context = TaskAContext { cnt: 0 };

    let task_b_config = TaskletConfig {
        name: "TaskB",
        priority: 1,
    };
    let task_b_context = TaskBContext { cnt: 0 };

    let monitor_config = TaskletConfig {
        name: "Monitor",
        priority: 0,
    };
    let monitor_context = MonitorContext { cnt: 0 };

    aerugo.create_tasklet_with_context(monitor_config, monitor, monitor_context, &MONITOR_STORAGE);
    aerugo.create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE);
    aerugo.create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE);

    let monitor_handle = MONITOR_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_cyclic(&monitor_handle, Some(Duration::secs(3)), None);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_cyclic(&task_a_handle, Some(Duration::secs(1)), None);

    let task_b_handle = TASK_B_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_cyclic(&task_b_handle, Some(Duration::secs(2)), None);

    aerugo.start();
}
