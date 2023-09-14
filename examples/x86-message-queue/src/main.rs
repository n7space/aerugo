use aerugo::{
    logln, Aerugo, InitApi, MessageQueueHandle, MessageQueueStorage, RuntimeApi,
    SystemHardwareConfig, TaskletConfig, TaskletStorage,
};

struct TaskAContext {
    queue_handle: MessageQueueHandle<u8, 10>,
}

struct TaskBContext {
    queue_handle: MessageQueueHandle<u8, 10>,
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_a(data: u8, context: &mut TaskAContext, _: &dyn RuntimeApi) {
    logln!("TaskA: {}", data);
    context
        .queue_handle
        .send_data(data.wrapping_add(1))
        .expect("Unable to send data from TaskA");
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_b(data: u8, context: &mut TaskBContext, _: &dyn RuntimeApi) {
    logln!("TaskB: {}", data);
    context
        .queue_handle
        .send_data(data.wrapping_add(1))
        .expect("Unable to send data from TaskB");
}

static TASK_A_STORAGE: TaskletStorage<u8, TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<u8, TaskBContext, 0> = TaskletStorage::new();

static QUEUE_X: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();

fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    aerugo.create_message_queue(&QUEUE_X);

    let queue_x_handle = QUEUE_X.create_handle().unwrap();

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
        queue_handle: queue_x_handle,
    };

    aerugo.create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE);
    aerugo.create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    let task_b_handle = TASK_B_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_queue(&task_a_handle, &queue_x_handle);
    aerugo.subscribe_tasklet_to_queue(&task_b_handle, &queue_x_handle);

    queue_x_handle
        .send_data(0)
        .expect("Unable to send data to QueueX");

    aerugo.start();
}
