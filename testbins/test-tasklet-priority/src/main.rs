use aerugo::{
    log, InitApi, MessageQueueHandle, MessageQueueStorage, SystemHardwareConfig, TaskletConfig,
    TaskletStorage, AERUGO,
};

struct TaskAContext {
    queue_handle: MessageQueueHandle<u8, 10>,
}

struct TaskBContext {
    cnt: u8,
}

struct TaskCContext {
    cnt: u8,
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_a(_: (), context: &mut TaskAContext) {
    context
        .queue_handle
        .send_data(1)
        .expect("Unable to send data from TaskA");
}

fn task_b(data: u8, context: &mut TaskBContext) {
    context.cnt += data;
    log!("TaskB: {}", context.cnt);

    if context.cnt == 5 {
        std::process::exit(0)
    }
}

fn task_c(data: u8, context: &mut TaskCContext) {
    context.cnt += data;
    log!("TaskC: {}", context.cnt);

    if context.cnt == 5 {
        std::process::exit(0)
    }
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<u8, TaskBContext> = TaskletStorage::new();
static TASK_C_STORAGE: TaskletStorage<u8, TaskCContext> = TaskletStorage::new();

static QUEUE_X: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();

fn main() -> ! {
    AERUGO.initialize(SystemHardwareConfig::default());

    AERUGO
        .create_message_queue(&QUEUE_X)
        .expect("Unable to create QueueX");

    let queue_x_handle = QUEUE_X
        .create_handle()
        .expect("Unable to create QueueX handle");

    let task_a_config = TaskletConfig {
        name: "TaskA",
        priority: 0,
    };
    let task_a_context = TaskAContext {
        queue_handle: queue_x_handle,
    };
    AERUGO
        .create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE)
        .expect("Unable to create TaskA");

    let task_b_config = TaskletConfig {
        name: "TaskB",
        priority: 1,
    };
    let task_b_context = TaskBContext { cnt: 0 };
    AERUGO
        .create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE)
        .expect("Unable to create TaskB");

    let task_c_config = TaskletConfig {
        name: "TaskC",
        priority: 0,
    };
    let task_c_context = TaskCContext { cnt: 0 };
    AERUGO
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

    AERUGO
        .subscribe_tasklet_to_cyclic(&task_a_handle, None)
        .expect("Unable to set cyclic on TaskA");
    AERUGO
        .subscribe_tasklet_to_queue(&task_b_handle, &queue_x_handle)
        .expect("Unable to subscribe TaskB to QueueX");
    AERUGO
        .subscribe_tasklet_to_queue(&task_c_handle, &queue_x_handle)
        .expect("Unable to subscribe TaskC to QueueX");

    AERUGO.start();
}
