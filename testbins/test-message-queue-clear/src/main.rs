use aerugo::{
    logln, Aerugo, InitApi, MessageQueueHandle, MessageQueueStorage, RuntimeApi,
    SystemHardwareConfig, TaskletConfig, TaskletStorage,
};

struct TaskAContext {
    queue_handle: MessageQueueHandle<u8, 10>,
}

struct TaskBContext {
    queue_handle: MessageQueueHandle<u8, 10>,
    cnt: u8,
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_a(_: (), context: &mut TaskAContext, _: &dyn RuntimeApi) {
    context
        .queue_handle
        .send_data(1)
        .expect("Unable to send data from TaskA");

    context
        .queue_handle
        .send_data(2)
        .expect("Unable to send data from TaskA");

    context
        .queue_handle
        .send_data(3)
        .expect("Unable to send data from TaskA");
}

fn task_b(data: u8, context: &mut TaskBContext, _: &dyn RuntimeApi) {
    logln!("TaskB: {}", data);

    context.queue_handle.clear();

    context.cnt += 1;

    if context.cnt == 3 {
        std::process::exit(0);
    }
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();
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
    let task_a_context = TaskAContext {
        queue_handle: queue_x_handle,
    };
    aerugo.create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE);

    let task_b_config = TaskletConfig {
        name: "TaskB",
        ..Default::default()
    };
    let task_b_context = TaskBContext {
        queue_handle: queue_x_handle,
        cnt: 0,
    };
    aerugo.create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    let task_b_handle = TASK_B_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_cyclic(&task_a_handle, None);
    aerugo.subscribe_tasklet_to_queue(&task_b_handle, &queue_x_handle);

    aerugo.start();
}
