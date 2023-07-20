#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use aerugo::hal::embedded_hal::watchdog::Watchdog as _;
use aerugo::hal::peripherals::watchdog::{Watchdog, WatchdogConfiguration};
use aerugo::{
    InitApi, MessageQueueHandle, MessageQueueStorage, TaskletConfig, TaskletStorage, AERUGO,
};
use cortex_m_semihosting::hprintln;
use rt::entry;

struct WdtTaskContext {
    watchdog: Watchdog,
    counter: u32,
    queue_handle: MessageQueueHandle<u32, 10>,
}

fn watchdog_task(_counter: u32, context: &mut WdtTaskContext) {
    context.watchdog.feed();
    context.counter = context.counter.wrapping_add(1);

    context
        .queue_handle
        .send_data(context.counter)
        .expect("Unable to send data from task.");
}

static WATCHDOG_TASK_STORAGE: TaskletStorage<u32, WdtTaskContext> = TaskletStorage::new();
static DATA_QUEUE: MessageQueueStorage<u32, 10> = MessageQueueStorage::new();

#[entry]
fn main() -> ! {
    hprintln!("Hello world, initializing Aerugo...");
    AERUGO.initialize();
    hprintln!("Aerugo initialized! Setting up task queues...");

    AERUGO
        .create_message_queue(&DATA_QUEUE)
        .expect("Unable to create data queue!");

    let queue_handle = DATA_QUEUE
        .create_handle()
        .expect("Unable to create data queue handle!");

    hprintln!("Task queues initialized! Setting up peripherals...");

    let peripherals = AERUGO.peripherals();
    let mut watchdog = Watchdog::new(peripherals.mcu_pac.WDT);
    watchdog.configure(WatchdogConfiguration {
        counter: 0xFF,
        run_in_idle: true,
        run_in_debug: true,
        ..Default::default()
    });

    hprintln!("Peripherals initialized! Setting up tasks...");

    let watchdog_task_config = TaskletConfig {
        name: "WatchdogTask",
    };

    let watchdog_task_context = WdtTaskContext {
        watchdog: watchdog,
        counter: 0,
        queue_handle,
    };

    AERUGO
        .create_tasklet_with_context(
            watchdog_task_config,
            watchdog_task,
            watchdog_task_context,
            &WATCHDOG_TASK_STORAGE,
        )
        .expect("Failed to create watchdog task!");

    let watchdog_task_handle = WATCHDOG_TASK_STORAGE
        .create_handle()
        .expect("Unable to create watchdog task handle");

    AERUGO
        .subscribe_tasklet_to_queue(&watchdog_task_handle, &queue_handle)
        .expect("Unable to subscribe watchdog task to data queue");

    hprintln!("Tasks created, starting!");

    queue_handle
        .send_data(0)
        .expect("Unable to send data to queue");

    AERUGO.start();
}
