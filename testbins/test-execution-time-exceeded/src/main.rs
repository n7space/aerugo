use aerugo::{
    logln, Aerugo, Duration, EventId, EventStorage, InitApi, RuntimeApi, SystemHardwareConfig,
    TaskletConfig, TaskletStorage,
};

enum MyEvents {
    TimeExceeded,
}

impl From<MyEvents> for EventId {
    fn from(value: MyEvents) -> Self {
        match value {
            MyEvents::TimeExceeded => 1,
        }
    }
}

#[derive(Default)]
struct TaskAContext {
    cnt: u8,
}

fn task_a(_: (), context: &mut TaskAContext, _: &'static dyn RuntimeApi) {
    context.cnt = context.cnt.wrapping_add(1);

    let sleep_time = if context.cnt == 1 {
        logln!("Running 20ms");
        std::time::Duration::from_millis(20)
    } else {
        logln!("Running 150ms");
        std::time::Duration::from_millis(150)
    };

    std::thread::sleep(sleep_time);
}

#[derive(Default)]
struct TaskBContext {}

fn task_b(_: EventId, _: &mut TaskBContext, _: &'static dyn RuntimeApi) {
    logln!("Time exceeded");
    std::process::exit(0);
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<EventId, TaskBContext, 0> = TaskletStorage::new();
static TIME_EXCEEDED_EVENT_STORAGE: EventStorage = EventStorage::new();

fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    aerugo.create_event(MyEvents::TimeExceeded.into(), &TIME_EXCEEDED_EVENT_STORAGE);

    let task_a_config = TaskletConfig {
        name: "TaskA",
        priority: 1,
    };
    let task_a_context = TaskAContext { cnt: 0 };

    let task_b_config = TaskletConfig {
        name: "TaskB",
        priority: 1,
    };

    aerugo.create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE);
    aerugo.create_tasklet(task_b_config, task_b, &TASK_B_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_cyclic(&task_a_handle, Some(Duration::secs(1)), None);

    let task_b_handle = TASK_B_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_events(&task_b_handle, [MyEvents::TimeExceeded.into()]);

    let time_exceeded_handle = TIME_EXCEEDED_EVENT_STORAGE.create_handle().unwrap();
    aerugo.set_execution_time_exceeded_maximum_event(&time_exceeded_handle, Duration::millis(100));

    aerugo.start();
}
