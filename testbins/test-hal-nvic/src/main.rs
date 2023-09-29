#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use core::cell::RefCell;

use aerugo::hal::drivers::nvic::{Interrupt, InterruptPriority, NVIC};
use aerugo::hal::interrupt;
use aerugo::{Aerugo, InitApi, Mutex, SystemHardwareConfig};
use calldwell::write_str;
use heapless::Vec;
use rt::entry;

static TEST_IRQ_EXECUTED_FLAG: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TestIrqsProgress {
    AStarted,
    AFinished,
    BExecuted,
}

static TEST_IRQS_PROGRESS: Mutex<RefCell<Vec<TestIrqsProgress, 3>>> =
    Mutex::new(RefCell::new(Vec::new()));
static NVIC_INSTANCE: Mutex<RefCell<Option<NVIC>>> = Mutex::new(RefCell::new(None));

const TRIGGER_TEST_IRQ: Interrupt = Interrupt::TC2CH0;
const PRIORITY_TEST_IRQ_A: Interrupt = Interrupt::TC2CH1;
const PRIORITY_TEST_IRQ_B: Interrupt = Interrupt::TC2CH2;

fn perform_irq_masking_test(nvic: &mut NVIC) {
    // Mask and unmask interrupts, check if returned status is valid
    nvic.enable(TRIGGER_TEST_IRQ);
    nvic.enable(PRIORITY_TEST_IRQ_A);
    nvic.enable(PRIORITY_TEST_IRQ_B);

    assert!(nvic.is_enabled(TRIGGER_TEST_IRQ));
    assert!(nvic.is_enabled(PRIORITY_TEST_IRQ_A));
    assert!(nvic.is_enabled(PRIORITY_TEST_IRQ_B));

    nvic.disable(TRIGGER_TEST_IRQ);
    nvic.disable(PRIORITY_TEST_IRQ_B);

    assert!(nvic.is_disabled(TRIGGER_TEST_IRQ));
    assert!(nvic.is_enabled(PRIORITY_TEST_IRQ_A));
    assert!(nvic.is_disabled(PRIORITY_TEST_IRQ_B));

    nvic.disable(PRIORITY_TEST_IRQ_A);

    assert!(nvic.is_disabled(TRIGGER_TEST_IRQ));
    assert!(nvic.is_disabled(PRIORITY_TEST_IRQ_A));
    assert!(nvic.is_disabled(PRIORITY_TEST_IRQ_B));

    write_str("IRQ masking test successful");
}

fn clear_irq_exec_flag() {
    TEST_IRQ_EXECUTED_FLAG.lock(|flag_ref| flag_ref.replace(false));
}

// This will also reset the state.
fn test_irq_triggered() -> bool {
    let state = TEST_IRQ_EXECUTED_FLAG.lock(|flag_ref| *flag_ref.borrow());
    clear_irq_exec_flag();
    state
}

fn perform_irq_trigger_via_trigger_test(nvic: &mut NVIC) {
    clear_irq_exec_flag();

    // Check if enabled (masked) IRQ will trigger
    nvic.enable(TRIGGER_TEST_IRQ);
    nvic.trigger(TRIGGER_TEST_IRQ);

    assert!(
        test_irq_triggered(),
        "IRQ triggered, but the handler wasn't executed"
    );
    assert!(
        nvic.is_not_pending(TRIGGER_TEST_IRQ),
        "IRQ triggered and returned, but somehow is still pending"
    );
    assert!(
        nvic.is_inactive(TRIGGER_TEST_IRQ),
        "IRQ active in main thread"
    );

    // Check if disabled (masked) IRQ will not trigger until unmasked
    nvic.disable(TRIGGER_TEST_IRQ);
    nvic.trigger(TRIGGER_TEST_IRQ);

    assert!(
        !test_irq_triggered(),
        "IRQ triggered while masked, but the handler was executed"
    );
    assert!(
        nvic.is_pending(TRIGGER_TEST_IRQ),
        "IRQ triggered while masked, but it's not pending"
    );
    assert!(
        nvic.is_inactive(TRIGGER_TEST_IRQ),
        "IRQ active in main thread"
    );

    nvic.enable(TRIGGER_TEST_IRQ);
    assert!(
        test_irq_triggered(),
        "IRQ masked, triggered and unmasked, but the handler wasn't executed"
    );
    assert!(
        nvic.is_not_pending(TRIGGER_TEST_IRQ),
        "IRQ masked, triggered and unmasked, but is still pending"
    );
    assert!(
        nvic.is_inactive(TRIGGER_TEST_IRQ),
        "IRQ active in main thread"
    );

    nvic.disable(TRIGGER_TEST_IRQ);
}

fn perform_irq_trigger_via_pend_test(nvic: &mut NVIC) {
    clear_irq_exec_flag();

    // Check if enabled (masked) IRQ will trigger after pending it
    nvic.enable(TRIGGER_TEST_IRQ);
    nvic.pend(TRIGGER_TEST_IRQ);

    assert!(
        test_irq_triggered(),
        "IRQ pend, but the handler wasn't executed"
    );
    assert!(
        nvic.is_not_pending(TRIGGER_TEST_IRQ),
        "IRQ pend and returned, but somehow is still pending"
    );
    assert!(
        nvic.is_inactive(TRIGGER_TEST_IRQ),
        "IRQ active in main thread"
    );

    // Check if disabled (masked) IRQ will not trigger after pending it, until unmasked
    nvic.disable(TRIGGER_TEST_IRQ);
    nvic.pend(TRIGGER_TEST_IRQ);

    assert!(
        !test_irq_triggered(),
        "IRQ pend while masked, but the handler was executed"
    );
    assert!(
        nvic.is_pending(TRIGGER_TEST_IRQ),
        "IRQ pend while masked, but it's not pending"
    );
    assert!(
        nvic.is_inactive(TRIGGER_TEST_IRQ),
        "IRQ active in main thread"
    );

    nvic.enable(TRIGGER_TEST_IRQ);
    assert!(
        test_irq_triggered(),
        "IRQ masked, pend and unmasked, but the handler wasn't executed"
    );
    assert!(
        nvic.is_not_pending(TRIGGER_TEST_IRQ),
        "IRQ masked, pend and unmasked, but is still pending"
    );
    assert!(
        nvic.is_inactive(TRIGGER_TEST_IRQ),
        "IRQ active in main thread"
    );

    // Check if disabled (masked) IRQ will not trigger if pend, unpend and unmasked
    nvic.disable(TRIGGER_TEST_IRQ);
    nvic.pend(TRIGGER_TEST_IRQ);
    nvic.unpend(TRIGGER_TEST_IRQ);
    nvic.enable(TRIGGER_TEST_IRQ);

    assert!(
        !test_irq_triggered(),
        "IRQ was pend and unpend while masked, yet the handler was executed"
    );
    assert!(
        nvic.is_not_pending(TRIGGER_TEST_IRQ),
        "IRQ was pend and unpend while masked, yet it is still pending"
    );
    assert!(
        nvic.is_inactive(TRIGGER_TEST_IRQ),
        "IRQ active in main thread"
    );
}

fn perform_irq_triggering_test(nvic: &mut NVIC) {
    perform_irq_trigger_via_trigger_test(nvic);
    perform_irq_trigger_via_pend_test(nvic);
    write_str("IRQ triggering test successful");
}

fn trigger_irq_and_share_nvic(mut nvic: NVIC, interrupt: Interrupt) {
    NVIC_INSTANCE.lock(|nvic_ref| {
        nvic.trigger(interrupt);
        nvic_ref.replace(Some(nvic));
    });
}

fn take_nvic_from_global_instance() -> NVIC {
    NVIC_INSTANCE.lock(|nvic_ref| nvic_ref.take().unwrap())
}

fn set_and_validate_priority(nvic: &mut NVIC, interrupt: Interrupt, priority_value: u8) {
    let priority: InterruptPriority = priority_value.try_into().unwrap();
    nvic.set_priority(interrupt, priority);
    let read_priority = nvic.get_priority(interrupt);

    assert!(
        read_priority == priority,
        "Tried to set priority of {:?} to {:?}, got {:?} instead!",
        interrupt,
        priority,
        read_priority
    );
}

fn validate_irq_execution_order(expected_order: [TestIrqsProgress; 3]) {
    TEST_IRQS_PROGRESS.lock(|progress_ref| {
        let progress_ref = progress_ref.get_mut();
        let progress_slice = progress_ref.as_slice();
        assert!(
            progress_slice == expected_order,
            "Unexpected order of IRQ execution, expected {:?}, got {:?}",
            expected_order,
            progress_slice
        );

        progress_ref.clear();
    });
}

fn perform_irq_priority_test(mut nvic: NVIC) -> NVIC {
    nvic.enable(PRIORITY_TEST_IRQ_A);
    nvic.enable(PRIORITY_TEST_IRQ_B);

    // Context: IRQ B is triggered in IRQ A handler
    // Case 1: IRQ A has lower priority than IRQ B (higher number = lower priority).
    // IRQ A priority: 5
    // IRQ B priority: 3
    // Expected effect: IRQ B pre-empted IRQ A
    set_and_validate_priority(&mut nvic, PRIORITY_TEST_IRQ_A, 5);
    set_and_validate_priority(&mut nvic, PRIORITY_TEST_IRQ_B, 3);
    trigger_irq_and_share_nvic(nvic, PRIORITY_TEST_IRQ_A);
    validate_irq_execution_order([
        TestIrqsProgress::AStarted,
        TestIrqsProgress::BExecuted,
        TestIrqsProgress::AFinished,
    ]);

    // Case 2: IRQ A has the same priority as IRQ B.
    // IRQ A priority: 4
    // IRQ B priority: 4
    // Expected effect: IRQ B executed after IRQ A
    let mut nvic = take_nvic_from_global_instance();
    set_and_validate_priority(&mut nvic, PRIORITY_TEST_IRQ_A, 4);
    set_and_validate_priority(&mut nvic, PRIORITY_TEST_IRQ_B, 4);
    trigger_irq_and_share_nvic(nvic, PRIORITY_TEST_IRQ_A);
    validate_irq_execution_order([
        TestIrqsProgress::AStarted,
        TestIrqsProgress::AFinished,
        TestIrqsProgress::BExecuted,
    ]);

    // Case 3: IRQ A has higher priority than IRQ B.
    // IRQ A priority: 0
    // IRQ B priority: 1
    // Expected effect: IRQ B executed after IRQ A
    let mut nvic = take_nvic_from_global_instance();
    set_and_validate_priority(&mut nvic, PRIORITY_TEST_IRQ_A, 0);
    set_and_validate_priority(&mut nvic, PRIORITY_TEST_IRQ_B, 1);
    trigger_irq_and_share_nvic(nvic, PRIORITY_TEST_IRQ_A);
    validate_irq_execution_order([
        TestIrqsProgress::AStarted,
        TestIrqsProgress::AFinished,
        TestIrqsProgress::BExecuted,
    ]);

    write_str("IRQ priority test successful");
    take_nvic_from_global_instance()
}

fn perform_nvic_test(mut nvic: NVIC) {
    perform_irq_masking_test(&mut nvic);
    perform_irq_triggering_test(&mut nvic);
    perform_irq_priority_test(nvic);
    write_str("all tests finished successfully");
}

#[entry]
fn main() -> ! {
    calldwell::start_session();
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    let nvic = NVIC::new(peripherals.nvic.take().unwrap());
    perform_nvic_test(nvic);

    aerugo.start();
}

/// Timer 2 Channel 0 IRQ
#[interrupt]
fn TC6() {
    TEST_IRQ_EXECUTED_FLAG.lock(|flag_ref| flag_ref.replace(true));
}

/// Timer 2 Channel 1 IRQ
#[interrupt]
fn TC7() {
    TEST_IRQS_PROGRESS.lock(|progress_ref| {
        progress_ref
            .get_mut()
            .push(TestIrqsProgress::AStarted)
            .unwrap()
    });

    NVIC_INSTANCE.lock(|nvic_ref| {
        nvic_ref
            .get_mut()
            .as_mut()
            .unwrap()
            .trigger(PRIORITY_TEST_IRQ_B)
    });

    TEST_IRQS_PROGRESS.lock(|progress_ref| {
        progress_ref
            .get_mut()
            .push(TestIrqsProgress::AFinished)
            .unwrap()
    });
}

/// Timer 2 Channel 2 IRQ
#[interrupt]
fn TC8() {
    TEST_IRQS_PROGRESS.lock(|progress_ref| {
        progress_ref
            .get_mut()
            .push(TestIrqsProgress::BExecuted)
            .unwrap()
    });
}
