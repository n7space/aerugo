use aerugo::{
    logln, BooleanConditionHandle, BooleanConditionSet, BooleanConditionStorage, InitApi,
    MessageQueueHandle, MessageQueueStorage, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage, AERUGO,
};

struct TaskAContext {
    queue_handle: MessageQueueHandle<u8, 10>,
}

struct TaskBContext {
    counter: u8,
    condition_handle: BooleanConditionHandle,
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_a(_: (), context: &mut TaskAContext, _: &dyn RuntimeApi) {
    context
        .queue_handle
        .send_data(1)
        .expect("Unable to send data from TaskA");
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_b(data: u8, context: &mut TaskBContext, _: &dyn RuntimeApi) {
    logln!("TaskB: {}", data);

    context.counter += 1;

    if context.counter == 5 {
        context.condition_handle.set_value(false);
    }
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 1> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<u8, TaskBContext, 0> = TaskletStorage::new();

static QUEUE_X_STORAGE: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();

static ENABLE_CONDITION_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

fn main() -> ! {
    AERUGO.initialize(SystemHardwareConfig::default());

    AERUGO
        .create_message_queue(&QUEUE_X_STORAGE)
        .expect("Unable to create QueueX");

    let queue_x_handle = QUEUE_X_STORAGE
        .create_handle()
        .expect("Unable to create QueueX handle");

    AERUGO
        .create_boolean_condition(&ENABLE_CONDITION_STORAGE, true)
        .expect("Unable to create EnableCondition");

    let enable_condition_handle = ENABLE_CONDITION_STORAGE
        .create_handle()
        .expect("Unable to create EnableCondition handle");

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };
    let task_b_config = TaskletConfig {
        name: "TaskB",
        ..Default::default()
    };

    let task_a_context = TaskAContext {
        queue_handle: queue_x_handle,
    };
    let task_b_context = TaskBContext {
        counter: 0,
        condition_handle: enable_condition_handle,
    };

    AERUGO
        .create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE)
        .expect("Unable to create TaskA");
    AERUGO
        .create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE)
        .expect("Unable to create TaskB");

    let task_a_handle = TASK_A_STORAGE
        .create_handle()
        .expect("Unable to create TaskA handle");
    let task_b_handle = TASK_B_STORAGE
        .create_handle()
        .expect("Unable to create TaskB handle");

    AERUGO
        .subscribe_tasklet_to_cyclic(&task_a_handle, None)
        .expect("Unable to subscribe TaskA to cyclic");
    AERUGO
        .subscribe_tasklet_to_queue(&task_b_handle, &queue_x_handle)
        .expect("Unable to subscribe TaskB to QueueX");

    let task_a_condition_set = BooleanConditionSet::from(enable_condition_handle);
    AERUGO
        .set_tasklet_conditions(&task_a_handle, task_a_condition_set)
        .expect("Unable to set TaskA condition set");

    AERUGO.start();
}
