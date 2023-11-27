use aerugo::{
    hal::{
        drivers::spi::{Master, Spi},
        user_peripherals::SPI0,
    },
    logln, Duration, InitApi, RuntimeApi, TaskletConfig, TaskletStorage,
};
use lsm6dso::LSM6DSO;

static LSM_TASK_STORAGE: TaskletStorage<(), LSMTaskContext, 0> = TaskletStorage::new();

type Lsm = LSM6DSO<Spi<SPI0, Master>>;

pub struct LSMTaskContext {
    pub lsm: Lsm,
}

pub fn lsm_task(_: (), context: &mut LSMTaskContext, _: &'static dyn RuntimeApi) {
    let fifo_status = context.lsm.get_fifo_status().unwrap();
    for _ in 0..fifo_status.stored_words {
        logln!("{:?}", context.lsm.get_next_fifo_word().unwrap());
    }
}

pub fn init_system(aerugo: &'static impl InitApi, lsm: Lsm) {
    logln!("Initializing tasks...");

    let lsm_task_config = TaskletConfig {
        name: "LSMTask",
        ..Default::default()
    };

    let lsm_task_context = LSMTaskContext { lsm };

    aerugo.create_tasklet_with_context(
        lsm_task_config,
        lsm_task,
        lsm_task_context,
        &LSM_TASK_STORAGE,
    );

    let lsm_task_handle = LSM_TASK_STORAGE.create_handle().unwrap();

    logln!("Tasks created, subscribing...");

    aerugo.subscribe_tasklet_to_cyclic(&lsm_task_handle, Some(Duration::millis(100)), None);

    logln!("Tasks created and subscribed!");
}
