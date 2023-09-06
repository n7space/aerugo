use aerugo::{
    logln, Aerugo, BooleanConditionHandle, BooleanConditionSet, BooleanConditionStorage, InitApi,
    MessageQueueHandle, MessageQueueStorage, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};

struct TaskAContext {
    queue_handle: MessageQueueHandle<u8, 10>,
    enable_condition_handle: BooleanConditionHandle,
    done_condition_handle: BooleanConditionHandle,
}

struct TaskBContext {
    counter: u8,
    enable_condition_handle: BooleanConditionHandle,
    done_condition_handle: BooleanConditionHandle,
}

struct TaskCContext {
    enable_condition_handle: BooleanConditionHandle,
    done_condition_handle: BooleanConditionHandle,
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_a(_: (), context: &mut TaskAContext, _: &dyn RuntimeApi) {
    let enable_status = context.enable_condition_handle.get_value();
    let done_status = context.done_condition_handle.get_value();

    logln!("TaskA: {}, {}", enable_status, done_status);

    context
        .queue_handle
        .send_data(1)
        .expect("Unable to send data from TaskA");
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_b(data: u8, context: &mut TaskBContext, _: &dyn RuntimeApi) {
    logln!("TaskB: {}", data);

    context.counter += 1;

    if context.counter == 3 {
        context.enable_condition_handle.set_value(false);
        context.done_condition_handle.set_value(true);
    }
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_c(data: bool, context: &mut TaskCContext, _: &dyn RuntimeApi) {
    let enable_status = context.enable_condition_handle.get_value();
    let done_status = context.done_condition_handle.get_value();

    logln!("TaskC: {}, {}", enable_status, done_status);

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
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    aerugo
        .create_message_queue(&QUEUE_X_STORAGE)
        .expect("Unable to create QueueX");

    let queue_x_handle = QUEUE_X_STORAGE
        .create_handle()
        .expect("Unable to create QueueX handle");

    aerugo
        .create_boolean_condition(&ENABLE_CONDITION_STORAGE, true)
        .expect("Unable to create EnableCondition");
    aerugo
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
        enable_condition_handle,
        done_condition_handle,
    };
    let task_b_context = TaskBContext {
        counter: 0,
        enable_condition_handle,
        done_condition_handle,
    };
    let task_c_context = TaskCContext {
        enable_condition_handle,
        done_condition_handle,
    };

    aerugo
        .create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE)
        .expect("Unable to create TaskA");
    aerugo
        .create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE)
        .expect("Unable to create TaskB");
    aerugo
        .create_tasklet_with_context(task_c_config, task_c, task_c_context, &TASK_C_STORAGE)
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

    aerugo
        .subscribe_tasklet_to_cyclic(&task_a_handle, None)
        .expect("Unable to subscribe TaskA to cyclic");
    aerugo
        .subscribe_tasklet_to_queue(&task_b_handle, &queue_x_handle)
        .expect("Unable to subscribe TaskB to QueueX");
    aerugo
        .subscribe_tasklet_to_condition(&task_c_handle, &done_condition_handle)
        .expect("Unable to subscribe TaskC to DoneCondition");

    let task_a_condition_set = BooleanConditionSet::from(enable_condition_handle);
    aerugo
        .set_tasklet_conditions(&task_a_handle, task_a_condition_set)
        .expect("Unable to set TaskA condition set");

    aerugo.start();
}
