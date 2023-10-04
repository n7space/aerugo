use aerugo::{
    logln, Aerugo, Duration, EventId, EventStorage, InitApi, RuntimeApi, SystemHardwareConfig,
    TaskletConfig, TaskletStorage,
};

enum MyEvents {
    OnceEvent,
    RepeatedEvent,
}

impl From<MyEvents> for EventId {
    fn from(value: MyEvents) -> Self {
        match value {
            MyEvents::OnceEvent => 1,
            MyEvents::RepeatedEvent => 2,
        }
    }
}

impl From<EventId> for MyEvents {
    fn from(value: EventId) -> Self {
        match value {
            1 => MyEvents::OnceEvent,
            2 => MyEvents::RepeatedEvent,
            _ => unreachable!(),
        }
    }
}

impl core::fmt::Debug for MyEvents {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            MyEvents::OnceEvent => write!(f, "OnceEvent"),
            MyEvents::RepeatedEvent => write!(f, "RepeatedEvent"),
        }
    }
}

#[derive(Default)]
struct TaskAContext {
    done: bool,
}

fn task_a(_: (), context: &mut TaskAContext, api: &'static dyn RuntimeApi) {
    if !context.done {
        api.schedule_event_at(MyEvents::OnceEvent.into(), Duration::secs(5))
            .expect("Failed to emit OnceEvent");
        context.done = true
    }

    api.schedule_event_in(MyEvents::RepeatedEvent.into(), Duration::secs(1))
        .expect("Failed to emit RepeatedEvent");
}

#[derive(Default)]
struct TaskBContext {}

fn task_b(value: EventId, _: &mut TaskBContext, api: &'static dyn RuntimeApi) {
    let current_time = api.get_system_time();
    let event: MyEvents = value.into();

    logln!(
        "Seconds: {}; Got {:?}",
        current_time.duration_since_epoch().to_secs(),
        event
    );
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<EventId, TaskBContext, 0> = TaskletStorage::new();

static ONCE_EVENT_STORAGE: EventStorage = EventStorage::new();
static REPEATED_EVENT_STORAGE: EventStorage = EventStorage::new();

fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    aerugo.create_event(MyEvents::OnceEvent.into(), &ONCE_EVENT_STORAGE);
    aerugo.create_event(MyEvents::RepeatedEvent.into(), &REPEATED_EVENT_STORAGE);

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };
    let task_a_context = TaskAContext { done: false };

    aerugo.create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE);

    let task_b_config = TaskletConfig {
        name: "TaskB",
        ..Default::default()
    };

    aerugo.create_tasklet(task_b_config, task_b, &TASK_B_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_cyclic(&task_a_handle, Some(Duration::secs(2)));

    let task_b_handle = TASK_B_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_events(
        &task_b_handle,
        [MyEvents::OnceEvent.into(), MyEvents::RepeatedEvent.into()],
    );

    aerugo.start();
}
