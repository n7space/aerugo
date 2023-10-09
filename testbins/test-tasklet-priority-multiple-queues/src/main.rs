use aerugo::{
    logln, Aerugo, InitApi, MessageQueueHandle, MessageQueueStorage, RuntimeApi,
    SystemHardwareConfig, TaskletConfig, TaskletStorage,
};

struct TaskAContext {
    cnt: u8,
    queue_x_handle: MessageQueueHandle<u8, 10>,
    queue_y_handle: MessageQueueHandle<u8, 10>,
}

struct TaskBContext {
    cnt: u8,
}

struct TaskCContext {
    cnt: u8,
}

fn task_a(_: (), context: &mut TaskAContext, _: &dyn RuntimeApi) {
    context.cnt += 1;

    if context.cnt <= 3 {
        context
            .queue_x_handle
            .send_data(1)
            .expect("Unable to send data from TaskA to QueueX");
        context
            .queue_x_handle
            .send_data(1)
            .expect("Unable to send data from TaskA to QueueX");
    }

    context
        .queue_y_handle
        .send_data(1)
        .expect("Unable to send data from TaskA to QueueY");
}

fn task_b(data: u8, context: &mut TaskBContext, _: &dyn RuntimeApi) {
    context.cnt += data;
    logln!("TaskB: {}", context.cnt);
}

fn task_c(data: u8, context: &mut TaskCContext, _: &dyn RuntimeApi) {
    context.cnt += data;
    logln!("TaskC: {}", context.cnt);

    if context.cnt == 5 {
        std::process::exit(0)
    }
}

static TASK_A_STORAGE: TaskletStorage<(), TaskAContext, 0> = TaskletStorage::new();
static TASK_B_STORAGE: TaskletStorage<u8, TaskBContext, 0> = TaskletStorage::new();
static TASK_C_STORAGE: TaskletStorage<u8, TaskCContext, 0> = TaskletStorage::new();

static QUEUE_X: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();
static QUEUE_Y: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();

fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    aerugo.create_message_queue(&QUEUE_X);
    aerugo.create_message_queue(&QUEUE_Y);

    let queue_x_handle = QUEUE_X.create_handle().unwrap();
    let queue_y_handle = QUEUE_Y.create_handle().unwrap();

    let task_a_config = TaskletConfig {
        name: "TaskA",
        priority: 0,
    };
    let task_a_context = TaskAContext {
        cnt: 0,
        queue_x_handle,
        queue_y_handle,
    };
    aerugo.create_tasklet_with_context(task_a_config, task_a, task_a_context, &TASK_A_STORAGE);

    let task_b_config = TaskletConfig {
        name: "TaskB",
        priority: 1,
    };
    let task_b_context = TaskBContext { cnt: 0 };
    aerugo.create_tasklet_with_context(task_b_config, task_b, task_b_context, &TASK_B_STORAGE);

    let task_c_config = TaskletConfig {
        name: "TaskC",
        priority: 0,
    };
    let task_c_context = TaskCContext { cnt: 0 };
    aerugo.create_tasklet_with_context(task_c_config, task_c, task_c_context, &TASK_C_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();
    let task_b_handle = TASK_B_STORAGE.create_handle().unwrap();
    let task_c_handle = TASK_C_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_cyclic(&task_a_handle, None, None);
    aerugo.subscribe_tasklet_to_queue(&task_b_handle, &queue_x_handle);
    aerugo.subscribe_tasklet_to_queue(&task_c_handle, &queue_y_handle);

    aerugo.start();
}
