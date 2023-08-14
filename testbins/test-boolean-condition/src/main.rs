use aerugo::{
    log, BooleanConditionHandle, BooleanConditionSet, BooleanConditionStorage, InitApi,
    MessageQueueHandle, MessageQueueStorage, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage, AERUGO,
};

struct TaskAContext {
    queue_handle: MessageQueueHandle<u8, 10>,
}

struct TaskBContext {
    counter: u8,
    enable_condition_handle: BooleanConditionHandle,
    done_condition_handle: BooleanConditionHandle,
}

#[derive(Default)]
struct TaskCContext {}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_a(_: (), context: &mut TaskAContext, _: &dyn RuntimeApi) {
    log!("TaskA");

    context
        .queue_handle
        .send_data(1)
        .expect("Unable to send data from TaskA");
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_b(data: u8, context: &mut TaskBContext, _: &dyn RuntimeApi) {
    log!("TaskB: {}", data);

    context.counter += 1;

    if context.counter == 3 {
        context.enable_condition_handle.set_value(false);
        context.done_condition_handle.set_value(true);
    }
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_c(data: bool, _: &mut TaskCContext, _: &dyn RuntimeApi) {
    log!("TaskC");

    if data {
        std::process::exit(0)
    }
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 1> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<u8, TaskBContext, 0> = TaskletStorage::new();
static TASK_C_STORAGE: TaskletStorage<bool, TaskCContext, 0> = TaskletStorage::new();

static QUEUE_X_STORAGE: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();

static ENABLE_CONDITION_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();
static DONE_CONDITION_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

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
    AERUGO
        .create_boolean_condition(&DONE_CONDITION_STORAGE, false)
        .expect("Unable to create DoneCondition");

    let enable_condition_handle = ENABLE_CONDITION_STORAGE
        .create_handle()
        .expect("Unable to create EnableCondition handle");
    let done_condition_handle = DONE_CONDITION_STORAGE
        .create_handle()
        .expect("Unable to create DoneCondition handle");

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };
    let task_b_config = TaskletConfig {
        name: "TaskB",
        ..Default::default()
    };
    let task_c_config = TaskletConfig {
        name: "TaskC",
        ..Default::default()
    };

    let task_a_context = TaskAContext {
        queue_handle: queue_x_handle,
    };
    let task_b_context = TaskBContext {
        counter: 0,
        enable_condition_handle,
        done_condition_handle,
    };

    AERUGO
        .create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE)
        .expect("Unable to create TaskA");
    AERUGO
        .create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE)
        .expect("Unable to create TaskB");
    AERUGO
        .create_tasklet(task_c_config, task_c, &TASK_C_STORAGE)
        .expect("Unable to create TaskC");

    let task_a_handle = TASK_A_STORAGE
        .create_handle()
        .expect("Unable to create TaskA handle");
    let task_b_handle = TASK_B_STORAGE
        .create_handle()
        .expect("Unable to create TaskB handle");
    let task_c_handle = TASK_C_STORAGE
        .create_handle()
        .expect("Unable to create TaskC handle");

    AERUGO
        .subscribe_tasklet_to_cyclic(&task_a_handle, None)
        .expect("Unable to subscribe TaskA to cyclic");
    AERUGO
        .subscribe_tasklet_to_queue(&task_b_handle, &queue_x_handle)
        .expect("Unable to subscribe TaskB to QueueX");
    AERUGO
        .subscribe_tasklet_to_condition(&task_c_handle, &done_condition_handle)
        .expect("Unable to subscribe TaskC to DoneCondition");

    let task_a_condition_set = BooleanConditionSet::from(enable_condition_handle);
    AERUGO
        .set_tasklet_conditions(&task_a_handle, task_a_condition_set)
        .expect("Unable to set TaskA condition set");

    AERUGO.start();
}
