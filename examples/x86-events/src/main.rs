use aerugo::{
    log, EventId, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig, TaskletStorage, AERUGO,
};

enum MyEvents {
    Event1,
    Event42,
    Event255,
}

impl From<MyEvents> for EventId {
    fn from(value: MyEvents) -> Self {
        match value {
            MyEvents::Event1 => 1,
            MyEvents::Event42 => 42,
            MyEvents::Event255 => 255,
        }
    }
}

#[derive(Default)]
struct TaskAContext {
    acc: u8,
}

fn task_a(_: (), context: &mut TaskAContext, api: &'static dyn RuntimeApi) {
    context.acc = context.acc.wrapping_add(1);

    match context.acc {
        1 => api
            .emit_event(MyEvents::Event1.into())
            .expect("Failed to emit Event1"),
        42 => api
            .emit_event(MyEvents::Event42.into())
            .expect("Failed to emit Event42"),
        255 => api
            .emit_event(MyEvents::Event255.into())
            .expect("Failed to emit Event255"),
        _ => (),
    };
}

#[derive(Default)]
struct TaskBContext {}

fn task_b(value: EventId, _: &mut TaskBContext, _: &'static dyn RuntimeApi) {
    log!("TaskB: {}", value);
}

#[derive(Default)]
struct TaskCContext {}

fn task_c(value: EventId, _: &mut TaskCContext, _: &'static dyn RuntimeApi) {
    log!("TaskC: {}", value);
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<EventId, TaskBContext, 0> = TaskletStorage::new();
static TASK_C_STORAGE: TaskletStorage<EventId, TaskCContext, 0> = TaskletStorage::new();

fn main() -> ! {
    AERUGO.initialize(SystemHardwareConfig::default());

    AERUGO
        .create_event(MyEvents::Event1.into())
        .expect("Unable to create Event1");
    AERUGO
        .create_event(MyEvents::Event42.into())
        .expect("Unable to create Event42");
    AERUGO
        .create_event(MyEvents::Event255.into())
        .expect("Unable to create Event255");

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };
    let task_a_context = TaskAContext { acc: 0 };

    AERUGO
        .create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE)
        .expect("Unable to create TaskA");

    let task_b_config = TaskletConfig {
        name: "TaskB",
        ..Default::default()
    };

    AERUGO
        .create_tasklet(task_b_config, task_b, &TASK_B_STORAGE)
        .expect("Unable to create TaskB");

    let task_c_config = TaskletConfig {
        name: "TaskC",
        ..Default::default()
    };

    AERUGO
        .create_tasklet(task_c_config, task_c, &TASK_C_STORAGE)
        .expect("Unable to create TaskC");

    let task_a_handle = TASK_A_STORAGE
        .create_handle()
        .expect("Unable to create handle to TaskA");
    let task_b_handle = TASK_B_STORAGE
        .create_handle()
        .expect("Unable to create handle to TaskB");
    let task_c_handle = TASK_C_STORAGE
        .create_handle()
        .expect("Unable to create handle to TaskC");

    AERUGO
        .subscribe_tasklet_to_cyclic(&task_a_handle, None)
        .expect("Unable to set cyclic on TaskA");
    AERUGO
        .subscribe_tasklet_to_events(&task_b_handle)
        .expect("Unable to subscribe TaskB to events")
        .enable(MyEvents::Event1.into())
        .expect("Unable to enable Event1 for TaskB")
        .enable(MyEvents::Event42.into())
        .expect("Unable to enable Event42 for TaskB");
    AERUGO
        .subscribe_tasklet_to_events(&task_c_handle)
        .expect("Unable to subscribe TaskC to events")
        .enable(MyEvents::Event42.into())
        .expect("Unable to enable Event42 for TaskC")
        .enable(MyEvents::Event255.into())
        .expect("Unable to enable Event255 for TaskC");

    AERUGO.start();
}
