#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::{
    hal::drivers::clocks_controller::{
        config::master_clock::{
            MasterClockConfig, MasterClockDivider, MasterClockSource, ProcessorClockPrescaler,
        },
        status::{MCOStatus, SlowClockSource},
        ClocksController, Interrupts as ClockInterrupts, Status as ClocksStatus,
    },
    Aerugo, InitApi, SystemHardwareConfig,
};
use calldwell::with_rtt_out;
use core::fmt::Write;
use rt::entry;

fn perform_status_test(clocks_controller: &mut ClocksController) {
    let current_status = clocks_controller.status();

    // Assuming default, startup configuration, status should be as follows:
    let default_status = ClocksStatus {
        mco_stabilized: false, // Crystal oscillator is not used
        plla_locked: false,    // PLLA is not configured
        master_clock_ready: true,
        utmi_pll_locked: false, // USB UTMI PLL is disabled
        slow_clock_source: SlowClockSource::InternalRCOscillator, // Crystal oscillator is not used
        pck_ready: [true; 8],
        main_clock_selected: true,
        main_rc_stabilized: true, // Main RC is used at startup
        mco_status: MCOStatus {
            // Crystal oscillator is not used, so clock failure detection is disabled
            clock_failure_event_detected: false,
            clock_failure_currently_detected: false,
            clock_failure_fault_active: false,
        },
        slow_oscillator_error: false, // Crystal oscillator is not used
    };

    if current_status != default_status {
        panic!(
            "unexpected status of PMC, expected {:?}, got {:?}",
            default_status, current_status
        );
    }

    // `status` method is tested by proxy in every function that configures clocks which
    // state is represented by flags in status register, and changing status can be performed
    // only by configuration. Therefore, there's no point in writing specific tests for `status`,
    // as the issues should be detected by configuration tests.

    with_rtt_out(|w, _| w.write_str("status test successful"));
}

fn perform_master_clock_test(clocks_controller: &mut ClocksController) {
    // Fetch and validate default configuration
    let original_master_clock_config = clocks_controller.master_clock_config();
    let expected_master_clock_config = MasterClockConfig {
        source: MasterClockSource::MainClock,
        prescaler: ProcessorClockPrescaler::NoDivision,
        divider: MasterClockDivider::NoDivision,
    };

    if original_master_clock_config != expected_master_clock_config {
        panic!(
            "unexpected default master clock configuration, expected {:?}, got {:?}",
            expected_master_clock_config, original_master_clock_config
        );
    }

    // Change MCU master clock configuration
    let new_master_clock_config = MasterClockConfig {
        source: MasterClockSource::MainClock,
        prescaler: ProcessorClockPrescaler::DivBy2,
        divider: MasterClockDivider::DivBy4,
    };

    clocks_controller.configure_master_clock(new_master_clock_config);
    if !clocks_controller.status().master_clock_ready {
        panic!("master clock is not ready after setting it's new configuration");
    }

    let read_master_clock_config = clocks_controller.master_clock_config();
    if read_master_clock_config != new_master_clock_config {
        panic!(
            "changing master clock configuration failed, expected {:?}, got {:?}",
            new_master_clock_config, read_master_clock_config
        );
    }

    // Restore previous configuration
    clocks_controller.configure_master_clock(original_master_clock_config);
    if !clocks_controller.status().master_clock_ready {
        panic!("master clock is not ready after restoring it's previous configuration");
    }

    let read_master_clock_config = clocks_controller.master_clock_config();
    if read_master_clock_config != original_master_clock_config {
        panic!(
            "restoring master clock configuration failed, expected {:?}, got {:?}",
            original_master_clock_config, read_master_clock_config
        );
    }

    with_rtt_out(|w, _| w.write_str("master clock test successful"));
}

fn perform_interrupt_config_test(clocks_controller: &mut ClocksController) {
    clocks_controller.enable_interrupts(ClockInterrupts::all());
    let masks_post_disable = clocks_controller.interrupts_masks();

    if masks_post_disable != ClockInterrupts::all() {
        panic!(
            "all interrupts were expected to be enabled, but instead got {:?}",
            masks_post_disable
        );
    }

    clocks_controller.disable_interrupts(ClockInterrupts::all());
    let masks_post_disable = clocks_controller.interrupts_masks();

    if masks_post_disable != ClockInterrupts::none() {
        panic!(
            "all interrupts were expected to be disabled, but instead got {:?}",
            masks_post_disable
        );
    }

    with_rtt_out(|w, _| w.write_str("interrupts config test successful"));
}

fn perform_main_rc_frequency_test(clocks_controller: &mut ClocksController) {
    let main_rc_frequency = clocks_controller.measure_main_rc_frequency();
    with_rtt_out(|w, _| write!(w.writer(), "{}", main_rc_frequency.to_Hz()).unwrap());
}

fn perform_clocks_controller_test(mut clocks_controller: ClocksController) {
    perform_status_test(&mut clocks_controller);
    perform_interrupt_config_test(&mut clocks_controller);
    perform_master_clock_test(&mut clocks_controller);
    perform_main_rc_frequency_test(&mut clocks_controller);
}

#[entry]
fn main() -> ! {
    calldwell::start_session();

    let (aerugo, peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    let clocks_controller = peripherals
        .clocks_controller
        .expect("clocks controller missing from peripherals structure");

    perform_clocks_controller_test(clocks_controller);

    aerugo.start();
}
