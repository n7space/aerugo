#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::{
    hal::drivers::timer::{
        channel_config::ChannelClock, waveform_config::WaveformModeConfig, Ch0, Channel, Timer,
        Waveform, TC1,
    },
    hal::{
        drivers::interrupt, drivers::pac::NVIC, drivers::pac::PMC,
        drivers::timer::channel_config::ChannelInterrupts,
    },
    Aerugo, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig, TaskletStorage,
};
use calldwell::with_rtt_out;
use core::{cell::RefCell, fmt::Write, ops::AddAssign};
use cortex_m::interrupt::{free as irq_free, Mutex};
use rt::entry;

static TIMER_CHANNEL: Mutex<RefCell<Option<Channel<TC1, Ch0, Waveform>>>> =
    Mutex::new(RefCell::new(None));
static TIMER_IRQ_COUNT: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

#[derive(Default)]
struct TimerTestTaskContext {
    acc: u32,
}

fn timer_test_task(_: (), context: &mut TimerTestTaskContext, _: &dyn RuntimeApi) {
    context.acc = context.acc.wrapping_add(1);

    if context.acc % 100 == 0 {
        let irq_count = get_irq_count();
        with_rtt_out(|w, _| write!(w.writer(), "{}", irq_count).unwrap());

        if context.acc == 1000 {
            disable_channel();
        }

        if context.acc == 2000 {
            change_channels_clock_source();
            enable_and_trigger_channel();
        }
    }
}

static TIMER_TEST_TASK_STORAGE: TaskletStorage<(), TimerTestTaskContext, 0> = TaskletStorage::new();

fn initialize_tasks(aerugo: &'static impl InitApi) {
    let timer_test_task_config = TaskletConfig {
        name: "TimerTestTask",
        ..Default::default()
    };

    let timer_test_task_context = TimerTestTaskContext::default();

    aerugo
        .create_tasklet_with_context(
            timer_test_task_config,
            timer_test_task,
            timer_test_task_context,
            &TIMER_TEST_TASK_STORAGE,
        )
        .expect("Unable to create timer test task!");

    let timer_test_task_handle = TIMER_TEST_TASK_STORAGE
        .create_handle()
        .expect("Unable to create timer test task handle!");

    aerugo
        .subscribe_tasklet_to_cyclic(&timer_test_task_handle, None)
        .expect("Unable to subscribe timer test task to cyclic execution!");
}

fn initialize_nvic() {
    unsafe {
        // Enable TC0 CH0 interrupt
        NVIC::unmask(interrupt::TC3);
    }
}

fn initialize_pmc(pmc: PMC) {
    // Enable TC1 CH0 clock
    pmc.pcer0.write(|w| w.pid26().set_bit());
}

fn initialize_timer(mut timer: Timer<TC1>) {
    // Enable waveform mode
    let mut channel = timer
        .channel_0
        .take()
        .expect("TC1 Ch0 already taken")
        .into_waveform_channel(WaveformModeConfig::default());

    // Use non-default clock source
    channel.set_clock_source(ChannelClock::MckDividedBy8);

    // Enable overflow interrupt
    channel.enable_interrupts(ChannelInterrupts {
        counter_overflow: true,
        ..Default::default()
    });

    // Put channel's instance in global context
    irq_free(|cs| {
        TIMER_CHANNEL.borrow(cs).replace(Some(channel));
    });
}

fn enable_and_trigger_channel() {
    irq_free(|cs| {
        let channel_ref = TIMER_CHANNEL.borrow(cs).borrow();
        let channel = channel_ref.as_ref().unwrap();
        channel.enable();
        channel.trigger();
    });
}

fn disable_channel() {
    irq_free(|cs| {
        let channel_ref = TIMER_CHANNEL.borrow(cs).borrow();
        let channel = channel_ref.as_ref().unwrap();
        channel.disable();
    });
}

fn change_channels_clock_source() {
    irq_free(|cs| {
        let mut channel_ref = TIMER_CHANNEL.borrow(cs).borrow_mut();
        let channel = channel_ref.as_mut().unwrap();
        channel.set_clock_source(ChannelClock::MckDividedBy32);
    });
}

fn clear_channel_irq_flags() {
    irq_free(|cs| {
        let mut channel_ref = TIMER_CHANNEL.borrow(cs).borrow_mut();
        let channel = channel_ref.as_mut().unwrap();
        channel.status();
    });
}

fn get_irq_count() -> u32 {
    irq_free(|cs| *TIMER_IRQ_COUNT.borrow(cs).borrow())
}

#[entry]
fn main() -> ! {
    calldwell::start_session();

    let (aerugo, peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    let timer = Timer::new(peripherals.timer_counter1.expect("TC1 already taken!"));

    initialize_nvic();
    initialize_pmc(peripherals.pmc.expect("PMC already taken!"));
    initialize_timer(timer);

    initialize_tasks(aerugo);
    enable_and_trigger_channel();

    aerugo.start();
}

#[interrupt]
fn TC3() {
    clear_channel_irq_flags();

    irq_free(|cs| {
        TIMER_IRQ_COUNT.borrow(cs).borrow_mut().add_assign(1);
    });
}
