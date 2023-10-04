use aerugo::{
    logln, Aerugo, Duration, EventId, EventStorage, InitApi, RuntimeApi, SystemHardwareConfig,
    TaskletConfig, TaskletStorage,
};

enum MyEvents {
    ScheduledEvent,
}

impl From<MyEvents> for EventId {
    fn from(value: MyEvents) -> Self {
        match value {
            MyEvents::ScheduledEvent => 1,
        }
    }
}

#[derive(Default)]
struct TaskAContext {}

fn task_a(_: (), _: &mut TaskAContext, api: &'static dyn RuntimeApi) {
    if !api
        .is_event_scheduled(MyEvents::ScheduledEvent.into())
        .unwrap()
    {
        api.schedule_event_at(MyEvents::ScheduledEvent.into(), Duration::secs(3))
            .expect("Failed to schedule ScheduledEvent");
    }
}

#[derive(Default)]
struct TaskBContext {}

fn task_b(_: EventId, _: &mut TaskBContext, api: &'static dyn RuntimeApi) {
    let current_time = api.get_system_time();

    logln!(
        "Current time is: {}s; Got event",
        current_time.duration_since_epoch().to_secs(),
    );

    std::process::exit(0);
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<EventId, TaskBContext, 0> = TaskletStorage::new();

static SCHEDULED_EVENT_STORAGE: EventStorage = EventStorage::new();

fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    aerugo.create_event(MyEvents::ScheduledEvent.into(), &SCHEDULED_EVENT_STORAGE);

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };

    aerugo.create_tasklet(task_a_config, task_a, &TASK_A_STORAGE);

    let task_b_config = TaskletConfig {
        name: "TaskB",
        ..Default::default()
    };

    aerugo.create_tasklet(task_b_config, task_b, &TASK_B_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_cyclic(&task_a_handle, Some(Duration::secs(1)));

    let task_b_handle = TASK_B_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_events(&task_b_handle, [MyEvents::ScheduledEvent.into()]);

    aerugo.start();
}
