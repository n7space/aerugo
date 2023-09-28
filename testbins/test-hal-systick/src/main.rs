#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::hal::user_peripherals::SYST;
use aerugo::{
    Aerugo, Duration, InitApi, Mutex, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    TaskletStorage,
};
use calldwell::with_rtt_out;
use core::cell::RefCell;
use core::fmt::Write;
use rt::{entry, exception};

static SYSTICK_COUNTER: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

#[derive(Default)]
struct SystickTestTaskContext {
    acc: u32,
    systick: Option<SYST>,
}

static SYSTICK_TEST_TASK_STORAGE: TaskletStorage<(), SystickTestTaskContext, 0> =
    TaskletStorage::new();

fn systick_test_task(_: (), context: &mut SystickTestTaskContext, _: &dyn RuntimeApi) {
    context.acc = context.acc.wrapping_add(1);

    // Send systick counter value to test suite
    let systick_counter = get_systick_counter();
    with_rtt_out(|w, _| write!(w.writer(), "{}", systick_counter).unwrap());

    // Reconfigure systick when next test step needs to be done
    if context.acc == 10 {
        let systick = context.systick.as_mut().unwrap();
        systick.disable_counter();
    }

    if context.acc == 20 {
        // Slower systick == higher reload
        let systick = context.systick.as_mut().unwrap();
        test_systick_reload(systick, 0xDEAD);
    }
}

fn clear_systick_counter() {
    SYSTICK_COUNTER.lock(|counter_ref| counter_ref.replace(0));
}

fn get_systick_counter() -> u32 {
    SYSTICK_COUNTER.lock(|counter_ref| *counter_ref.borrow())
}

fn test_systick_reload(systick: &mut SYST, reload_value: u32) {
    assert!(
        reload_value <= 0xFFFFFF,
        "Invalid reload value, expected value in range [0, 0xFFFFFF], got {}",
        reload_value
    );

    clear_systick_counter();
    systick.disable_counter();
    assert!(
        !systick.is_counter_enabled(),
        "Systick counter disabled, but it reports as being enabled!"
    );
    systick.clear_current();
    systick.set_reload(reload_value);
    systick.enable_counter();
    assert!(
        systick.is_counter_enabled(),
        "Systick counter enabled, but it reports as being disabled!"
    );
}

fn perform_systick_irq_management_test(systick: &mut SYST) {
    systick.disable_interrupt();
    assert!(
        !systick.is_interrupt_enabled(),
        "Systick interrupt was disabled, but it's reported as enabled"
    );

    systick.enable_interrupt();
    assert!(
        systick.is_interrupt_enabled(),
        "Systick interrupt was enabled, but it's reported as disabled"
    );

    systick.disable_interrupt();
    assert!(
        !systick.is_interrupt_enabled(),
        "Systick interrupt was disabled, but it's reported as enabled"
    );
}

fn initialize_test_task(aerugo: &'static impl InitApi, mut systick: SYST) {
    let systick_test_task_config = TaskletConfig {
        name: "SystickTestTask",
        ..Default::default()
    };

    let mut systick_test_task_context = SystickTestTaskContext::default();
    test_systick_reload(&mut systick, 0xBEEF);
    systick_test_task_context.systick.replace(systick);

    aerugo.create_tasklet_with_context(
        systick_test_task_config,
        systick_test_task,
        systick_test_task_context,
        &SYSTICK_TEST_TASK_STORAGE,
    );

    let systick_test_task_handle = SYSTICK_TEST_TASK_STORAGE.create_handle().unwrap();
    aerugo.subscribe_tasklet_to_cyclic(&systick_test_task_handle, Some(Duration::millis(100)));
}

#[entry]
fn main() -> ! {
    calldwell::start_session();
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    let mut systick = peripherals.systick.take().unwrap();
    perform_systick_irq_management_test(&mut systick);

    systick.enable_interrupt();
    initialize_test_task(aerugo, systick);
    aerugo.start();
}

#[exception]
fn SysTick() {
    SYSTICK_COUNTER.lock(|counter_ref| {
        let counter = counter_ref.get_mut();
        *counter += 1;
    });
}
