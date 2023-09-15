use aerugo::{
    logln, Aerugo, EventId, EventStorage, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};

enum MyEvents {
    SmallEvent,
    BigEvent,
}

impl From<MyEvents> for EventId {
    fn from(value: MyEvents) -> Self {
        match value {
            MyEvents::SmallEvent => 13,
            MyEvents::BigEvent => 42,
        }
    }
}

impl From<EventId> for MyEvents {
    fn from(value: EventId) -> Self {
        match value {
            13 => MyEvents::SmallEvent,
            42 => MyEvents::BigEvent,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct TaskAContext {
    acc: u8,
}

fn task_a(_: (), context: &mut TaskAContext, api: &'static dyn RuntimeApi) {
    match context.acc {
        0 => api
            .emit_event(MyEvents::BigEvent.into())
            .expect("Failed to emit BigEvent"),
        1 => api
            .emit_event(MyEvents::SmallEvent.into())
            .expect("Failed to emit SmallEvent"),
        2 => logln!("TaskA"),
        3 => std::process::exit(0),
        _ => unreachable!(),
    }

    context.acc += 1;
}

#[derive(Default)]
struct TaskBContext {}

fn task_b(value: EventId, _: &mut TaskBContext, api: &'static dyn RuntimeApi) {
    logln!("TaskB: {}", value);

    match value.into() {
        MyEvents::SmallEvent => api.clear_event_queue(),
        MyEvents::BigEvent => (),
    };
}

#[derive(Default)]
struct TaskCContext {}

fn task_c(value: EventId, _: &mut TaskCContext, _: &'static dyn RuntimeApi) {
    logln!("TaskC: {}", value);
}

#[derive(Default)]
struct TaskDContext {}

fn task_d(value: EventId, _: &mut TaskDContext, _: &'static dyn RuntimeApi) {
    logln!("TaskD: {}", value);
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<EventId, TaskBContext, 0> = TaskletStorage::new();
static TASK_C_STORAGE: TaskletStorage<EventId, TaskCContext, 0> = TaskletStorage::new();
static TASK_D_STORAGE: TaskletStorage<EventId, TaskDContext, 0> = TaskletStorage::new();

static SMALL_EVENT_STORAGE: EventStorage = EventStorage::new();
static BIG_EVENT_STORAGE: EventStorage = EventStorage::new();

fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    aerugo.create_event(MyEvents::SmallEvent.into(), &SMALL_EVENT_STORAGE);
    aerugo.create_event(MyEvents::BigEvent.into(), &BIG_EVENT_STORAGE);

    let task_a_config = TaskletConfig {
        name: "TaskA",
        priority: 0,
    };
    let task_a_context = TaskAContext { acc: 0 };

    aerugo.create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE);

    let task_b_config = TaskletConfig {
        name: "TaskB",
        priority: 3,
    };

    aerugo.create_tasklet(task_b_config, task_b, &TASK_B_STORAGE);

    let task_c_config = TaskletConfig {
        name: "TaskC",
        priority: 2,
    };

    aerugo.create_tasklet(task_c_config, task_c, &TASK_C_STORAGE);

    let task_d_config = TaskletConfig {
        name: "TaskD",
        priority: 1,
    };

    aerugo.create_tasklet(task_d_config, task_d, &TASK_D_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    let task_b_handle = TASK_B_STORAGE.create_handle().unwrap();
    let task_c_handle = TASK_C_STORAGE.create_handle().unwrap();
    let task_d_handle = TASK_D_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_cyclic(&task_a_handle, None);

    let task_b_events = [MyEvents::SmallEvent.into(), MyEvents::BigEvent.into()];
    aerugo.subscribe_tasklet_to_events(&task_b_handle, task_b_events);

    let task_c_events = [MyEvents::SmallEvent.into(), MyEvents::BigEvent.into()];
    aerugo.subscribe_tasklet_to_events(&task_c_handle, task_c_events);

    let task_d_events = [MyEvents::SmallEvent.into(), MyEvents::BigEvent.into()];
    aerugo.subscribe_tasklet_to_events(&task_d_handle, task_d_events);

    aerugo.start();
}
