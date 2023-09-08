#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use core::cell::RefCell;

use aerugo::hal::drivers::clocks_controller::config::PeripheralId;
use aerugo::hal::drivers::clocks_controller::ClocksController;
use aerugo::hal::drivers::timer::{
    channel_config::ChannelClock, waveform_config::WaveformModeConfig, Ch0, Channel, Waveform, TC1,
};
use cortex_m::interrupt::free as irq_free;
use cortex_m::interrupt::Mutex;

use aerugo::{
    hal::drivers::timer::Timer, logln, Aerugo, InitApi, RuntimeApi, SystemHardwareConfig,
    TaskletConfig, TaskletStorage,
};
use rt::entry;

static TIMER_CHANNEL: Mutex<RefCell<Option<Channel<TC1, Ch0, Waveform>>>> =
    Mutex::new(RefCell::new(None));

#[derive(Default)]
struct DummyTaskContext {
    acc: u16,
}

fn dummy_task(_: (), context: &mut DummyTaskContext, _: &'static dyn RuntimeApi) {
    context.acc = context.acc.wrapping_add(1);
    if context.acc % 1000 == 0 {
        irq_free(|cs| {
            // This is safe, because TIMER_CHANNEL is set before the scheduler starts.
            let timer_value = TIMER_CHANNEL
                .borrow(cs)
                .borrow()
                .as_ref()
                .unwrap()
                .counter_value();
            logln!("TC1 CH0: {}", timer_value);
        })
    }
}

static DUMMY_TASK_STORAGE: TaskletStorage<(), DummyTaskContext, 0> = TaskletStorage::new();

fn init_clocks(mut clocks_controller: ClocksController) {
    // Enable TC1 CH0 clock
    clocks_controller.enable_peripheral_clock(PeripheralId::TC1CH0);
}

fn init_timer(mut timer: Timer<TC1>) {
    let mut ch0 = timer
        .channel_0
        .take()
        .expect("Channel 0 of Timer 1 already taken")
        .into_waveform_channel(WaveformModeConfig::default());
    ch0.set_clock_source(ChannelClock::MckDividedBy8);
    ch0.enable();
    ch0.trigger();

    let status = ch0.status().clock_enabled;
    logln!("Clock is {}", if status { "enabled" } else { "disabled" });

    irq_free(|cs| {
        TIMER_CHANNEL.borrow(cs).replace(Some(ch0));
    });
}

fn init_tasks(aerugo: &'static impl InitApi) {
    logln!("Creating tasks...");
    let dummy_task_config = TaskletConfig {
        name: "DummyTask",
        ..Default::default()
    };
    let dummy_task_context = DummyTaskContext::default();

    aerugo
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

    logln!("Subscribing tasks...");

    aerugo
        .subscribe_tasklet_to_cyclic(&dummy_task_handle, None)
        .expect("Unable to subscribe dummy task to cyclic execution!");
}

#[entry]
fn main() -> ! {
    let (aerugo, peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    logln!("Hello, world! Aerugo initialized!");

    logln!("Doing stuff with timers...");
    let timer = Timer::new(peripherals.timer_counter1.expect("Timer 1 already used"));
    let clocks_controller = peripherals
        .clocks_controller
        .expect("Clocks controller already used");
    init_clocks(clocks_controller);
    init_timer(timer);

    init_tasks(aerugo);

    logln!("Starting the system!");
    aerugo.start();
}
