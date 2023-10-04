use aerugo::{
    logln, Aerugo, BooleanConditionHandle, BooleanConditionSet, BooleanConditionStorage, Duration,
    EventId, EventStorage, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};

enum MyEvents {
    EventA,
    EventB,
    EventC,
    Done,
}

impl From<MyEvents> for EventId {
    fn from(value: MyEvents) -> Self {
        match value {
            MyEvents::EventA => 1,
            MyEvents::EventB => 2,
            MyEvents::EventC => 3,
            MyEvents::Done => 4,
        }
    }
}

struct SetupContext {
    setup_condition: BooleanConditionHandle,
}

fn setup(_: (), context: &mut SetupContext, api: &'static dyn RuntimeApi) {
    api.schedule_event_in(MyEvents::EventA.into(), Duration::secs(3))
        .expect("Failed to schedule EventA");
    api.schedule_event_in(MyEvents::EventB.into(), Duration::secs(4))
        .expect("Failed to schedule EventB");
    api.schedule_event_in(MyEvents::EventC.into(), Duration::secs(5))
        .expect("Failed to schedule EventC");

    context.setup_condition.set_value(false);
}

#[derive(Default)]
struct TaskAContext {}

fn task_a(event_id: EventId, _: &mut TaskAContext, api: &'static dyn RuntimeApi) {
    logln!("Got event: {}", event_id);

    api.cancel_event(MyEvents::EventB.into())
        .expect("Failed to cancel EventB");

    api.schedule_event_in(MyEvents::Done.into(), Duration::secs(5))
        .expect("Failed to schedule Done");
}

#[derive(Default)]
struct TaskBContext {}

fn task_b(event_id: EventId, _: &mut TaskBContext, _: &'static dyn RuntimeApi) {
    logln!("Got event: {}", event_id);
}

#[derive(Default)]
struct TaskCContext {}

fn task_c(event_id: EventId, _: &mut TaskCContext, _: &'static dyn RuntimeApi) {
    logln!("Got event: {}", event_id);
}

#[derive(Default)]
struct DoneContext {}

fn done(_: EventId, _: &mut DoneContext, _: &'static dyn RuntimeApi) {
    std::process::exit(0);
}

static SETUP_STORAGE: TaskletStorage<(), SetupContext, 1> = TaskletStorage::new();
static TASK_A_STORAGE: TaskletStorage<EventId, TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<EventId, TaskBContext, 0> = TaskletStorage::new();
static TASK_C_STORAGE: TaskletStorage<EventId, TaskCContext, 0> = TaskletStorage::new();
static DONE_STORAGE: TaskletStorage<EventId, DoneContext, 0> = TaskletStorage::new();

static EVENT_A_STORAGE: EventStorage = EventStorage::new();
static EVENT_B_STORAGE: EventStorage = EventStorage::new();
static EVENT_C_STORAGE: EventStorage = EventStorage::new();
static DONE_EVENT_STORAGE: EventStorage = EventStorage::new();

static SETUP_CONDITION_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    aerugo.create_event(MyEvents::EventA.into(), &EVENT_A_STORAGE);
    aerugo.create_event(MyEvents::EventB.into(), &EVENT_B_STORAGE);
    aerugo.create_event(MyEvents::EventC.into(), &EVENT_C_STORAGE);
    aerugo.create_event(MyEvents::Done.into(), &DONE_EVENT_STORAGE);

    aerugo.create_boolean_condition(true, &SETUP_CONDITION_STORAGE);
    let setup_condition_handle = SETUP_CONDITION_STORAGE.create_handle().unwrap();

    let setup_config = TaskletConfig {
        name: "Setup",
        priority: 1,
    };
    let setup_context = SetupContext {
        setup_condition: setup_condition_handle,
    };
    aerugo.create_tasklet_with_context(setup_config, setup, setup_context, &SETUP_STORAGE);

    let task_a_config = TaskletConfig {
        name: "TaskA",
        priority: 0,
    };
    aerugo.create_tasklet(task_a_config, task_a, &TASK_A_STORAGE);

    let task_b_config = TaskletConfig {
        name: "TaskB",
        priority: 0,
    };
    aerugo.create_tasklet(task_b_config, task_b, &TASK_B_STORAGE);

    let task_c_config = TaskletConfig {
        name: "TaskC",
        priority: 0,
    };
    aerugo.create_tasklet(task_c_config, task_c, &TASK_C_STORAGE);

    let done_config = TaskletConfig {
        name: "Done",
        priority: 0,
    };
    aerugo.create_tasklet(done_config, done, &DONE_STORAGE);

    let setup_handle = SETUP_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_cyclic(&setup_handle, None);
    aerugo.set_tasklet_conditions(
        &setup_handle,
        BooleanConditionSet::from(setup_condition_handle),
    );

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_events(&task_a_handle, [MyEvents::EventA.into()]);

    let task_b_handle = TASK_B_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_events(&task_b_handle, [MyEvents::EventB.into()]);

    let task_c_handle = TASK_C_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_events(&task_c_handle, [MyEvents::EventC.into()]);

    let done_handle = DONE_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_events(&done_handle, [MyEvents::Done.into()]);

    aerugo.start();
}
