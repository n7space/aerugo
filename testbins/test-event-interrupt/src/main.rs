#![no_std]
#![no_main]

extern crate calldwell;

use core::fmt::Write;

use aerugo::{
    Aerugo, EventHandle, EventId, EventStorage, InitApi, RuntimeApi, SystemHardwareConfig,
    TaskletConfig, TaskletStorage,
};
use calldwell::with_rtt_out;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use lazy_static::lazy_static;

enum MyEvents {
    Event42,
}

impl From<MyEvents> for EventId {
    fn from(value: MyEvents) -> Self {
        match value {
            MyEvents::Event42 => 42,
        }
    }
}

#[derive(Default)]
struct TaskAContext {}

#[allow(clippy::needless_pass_by_ref_mut)]
fn task_a(value: EventId, _: &mut TaskAContext, _: &dyn RuntimeApi) {
    with_rtt_out(|w, _| write!(w.writer(), "TaskA got {}", value).unwrap());
}

static TASK_A_STORAGE: TaskletStorage<EventId, TaskAContext, 1> = TaskletStorage::new();

static EVENT_42_STORAGE: EventStorage = EventStorage::new();

#[entry]
fn main() -> ! {
    calldwell::start_session();

    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    aerugo.create_event(MyEvents::Event42.into(), &EVENT_42_STORAGE);

    let task_a_config = TaskletConfig {
        name: "TaskA",
        ..Default::default()
    };

    aerugo.create_tasklet(task_a_config, task_a, &TASK_A_STORAGE);

    let task_a_handle = TASK_A_STORAGE.create_handle().unwrap();

    aerugo.subscribe_tasklet_to_events(&task_a_handle, [MyEvents::Event42.into()]);

    let mut systick = peripherals.systick.take().unwrap();
    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(0xBEEF);
    systick.enable_interrupt();
    systick.enable_counter();

    aerugo.start();
}

#[exception]
fn SysTick() {
    lazy_static! {
        static ref EVENT_HANDLE: EventHandle = EVENT_42_STORAGE.create_handle().unwrap();
    }

    EVENT_HANDLE.emit();
}
