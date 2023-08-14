#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;

use core::cell::RefCell;

use aerugo::hal::drivers::timer::{
    channel_config::ChannelClock, waveform_config::WaveformModeConfig, Ch0, Channel, Waveform, TC1,
};
use cortex_m::interrupt::free as irq_free;
use cortex_m::interrupt::Mutex;
use panic_semihosting as _;

use aerugo::{
    hal::drivers::timer::Timer, time::MillisDurationU32, InitApi, SystemHardwareConfig,
    TaskletConfig, TaskletStorage, AERUGO,
};
use cortex_m_semihosting::hprintln;
use rt::entry;

static TIMER_CHANNEL: Mutex<RefCell<Option<Channel<TC1, Ch0, Waveform>>>> =
    Mutex::new(RefCell::new(None));

#[derive(Default)]
struct DummyTaskContext {
    acc: u16,
}

fn dummy_task(_: (), context: &mut DummyTaskContext) {
    context.acc = context.acc.wrapping_add(1);
    if context.acc % 1000 == 0 {
        hprintln!("I'm running!");

        irq_free(|cs| {
            // This is safe, because TIMER_CHANNEL is set before the scheduler starts.
            let timer_value = TIMER_CHANNEL
                .borrow(cs)
                .borrow()
                .as_ref()
                .unwrap()
                .counter_value();
            hprintln!("TC1 CH0: {}", timer_value);
        })
    }
}

static DUMMY_TASK_STORAGE: TaskletStorage<(), DummyTaskContext, 0> = TaskletStorage::new();

fn init_timer(timer: &mut Timer<TC1>) {
    let ch0 = timer
        .channel_0
        .take()
        .expect("Channel 0 of Timer 1 already taken")
        .into_waveform_channel(WaveformModeConfig::default());
    ch0.set_clock_source(ChannelClock::MckDividedBy8);
    ch0.enable();
    ch0.trigger();

    let status = ch0.read_and_clear_status().clock_enabled;
    hprintln!("Clock is {}", if status { "enabled" } else { "disabled" });

    irq_free(|cs| {
        TIMER_CHANNEL.borrow(cs).replace(Some(ch0));
    });
}

fn init_tasks() {
    hprintln!("Creating tasks...");
    let dummy_task_config = TaskletConfig {
        name: "DummyTask",
        ..Default::default()
    };
    let dummy_task_context = DummyTaskContext::default();

    AERUGO
        .create_tasklet_with_context(
            dummy_task_config,
            dummy_task,
            dummy_task_context,
            &DUMMY_TASK_STORAGE,
        )
        .expect("Unable to create dummy task!");

    let dummy_task_handle = DUMMY_TASK_STORAGE
        .create_handle()
        .expect("Unable to create handle to dummy task!");

    hprintln!("Subscribing tasks...");

    AERUGO
        .subscribe_tasklet_to_cyclic(&dummy_task_handle, None)
        .expect("Unable to subscribe dummy task to cyclic execution!");
}

#[entry]
fn main() -> ! {
    hprintln!("Hello, world! Initializing Aerugo...");

    AERUGO.initialize(SystemHardwareConfig {
        watchdog_timeout: MillisDurationU32::secs(5),
    });

    hprintln!("Doing stuff with timers...");
    let peripherals = AERUGO
        .peripherals()
        .expect("peripherals are not initialized")
        .expect("peripherals already taken");

    let mut timer = Timer::new(peripherals.timer_counter1.expect("Timer 1 already used"));
    // TODO when PMC driver is done: actually enable timer clock so it starts correctly.
    init_timer(&mut timer);

    init_tasks();

    hprintln!("Starting the system!");
    AERUGO.start();
}
