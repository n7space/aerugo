use aerugo::{
    log, EventId, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig, TaskletStorage, AERUGO,
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
        2 => log!("TaskA"),
        3 => std::process::exit(0),
        _ => unreachable!(),
    }

    context.acc += 1;
}

#[derive(Default)]
struct TaskBContext {}

fn task_b(value: EventId, _: &mut TaskBContext, api: &'static dyn RuntimeApi) {
    log!("TaskB: {}", value);

    match value.into() {
        MyEvents::SmallEvent => api.clear_event_queue(),
        MyEvents::BigEvent => (),
    };
}

#[derive(Default)]
struct TaskCContext {}

fn task_c(value: EventId, _: &mut TaskCContext, _: &'static dyn RuntimeApi) {
    log!("TaskC: {}", value);
}

#[derive(Default)]
struct TaskDContext {}

fn task_d(value: EventId, _: &mut TaskDContext, _: &'static dyn RuntimeApi) {
    log!("TaskD: {}", value);
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<EventId, TaskBContext, 0> = TaskletStorage::new();
static TASK_C_STORAGE: TaskletStorage<EventId, TaskCContext, 0> = TaskletStorage::new();
static TASK_D_STORAGE: TaskletStorage<EventId, TaskDContext, 0> = TaskletStorage::new();

fn main() -> ! {
    AERUGO.initialize(SystemHardwareConfig::default());

    AERUGO
        .create_event(MyEvents::SmallEvent.into())
        .expect("Unable to create SmallEvent");
    AERUGO
        .create_event(MyEvents::BigEvent.into())
        .expect("Unable to create BigEvent");

    let task_a_config = TaskletConfig {
        name: "TaskA",
        priority: 0,
    };
    let task_a_context = TaskAContext { acc: 0 };

    AERUGO
        .create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE)
        .expect("Unable to create TaskA");

    let task_b_config = TaskletConfig {
        name: "TaskB",
        priority: 3,
    };

    AERUGO
        .create_tasklet(task_b_config, task_b, &TASK_B_STORAGE)
        .expect("Unable to create TaskB");

    let task_c_config = TaskletConfig {
        name: "TaskC",
        priority: 2,
    };

    AERUGO
        .create_tasklet(task_c_config, task_c, &TASK_C_STORAGE)
        .expect("Unable to create TaskC");

    let task_d_config = TaskletConfig {
        name: "TaskD",
        priority: 1,
    };

    AERUGO
        .create_tasklet(task_d_config, task_d, &TASK_D_STORAGE)
        .expect("Unable to create TaskD");

    let task_a_handle = TASK_A_STORAGE
        .create_handle()
        .expect("Unable to create handle to TaskA");
    let task_b_handle = TASK_B_STORAGE
        .create_handle()
        .expect("Unable to create handle to TaskB");
    let task_c_handle = TASK_C_STORAGE
        .create_handle()
        .expect("Unable to create handle to TaskC");
    let task_d_handle = TASK_D_STORAGE
        .create_handle()
        .expect("Unable to create handle to TaskD");

    AERUGO
        .subscribe_tasklet_to_cyclic(&task_a_handle, None)
        .expect("Unable to set cyclic on TaskA");
    AERUGO
        .subscribe_tasklet_to_events(&task_b_handle)
        .expect("Unable to subscribe TaskB to events")
        .enable(MyEvents::SmallEvent.into())
        .expect("Unable to enable SmallEvent for TaskB")
        .enable(MyEvents::BigEvent.into())
        .expect("Unable to enable BigEvent for TaskB");
    AERUGO
        .subscribe_tasklet_to_events(&task_c_handle)
        .expect("Unable to subscribe TaskC to events")
        .enable(MyEvents::SmallEvent.into())
        .expect("Unable to enable SmallEvent for TaskC")
        .enable(MyEvents::BigEvent.into())
        .expect("Unable to enable BigEvent for TaskC");
    AERUGO
        .subscribe_tasklet_to_events(&task_d_handle)
        .expect("Unable to subscribe TaskD to events")
        .enable(MyEvents::SmallEvent.into())
        .expect("Unable to enable SmallEvent for TaskD")
        .enable(MyEvents::BigEvent.into())
        .expect("Unable to enable BigEvent for TaskD");

    AERUGO.start();
}
