#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::{
    hal::drivers::pmc::{
        config::{
            main_rc::MainRcFrequency,
            master_clock::{
                MasterClockConfig, MasterClockDivider, MasterClockSource, ProcessorClockPrescaler,
            },
            pck::{PCKConfig, PCKPrescaler, PCKSource, PCK},
            peripheral::{
                GenericClockConfig, GenericClockDivider, GenericClockSource, PeripheralClockConfig,
            },
            PeripheralId,
        },
        status::{MCOStatus, SlowClockSource},
        Interrupts as ClockInterrupts, Status as ClocksStatus, PMC,
    },
    Aerugo, InitApi, SystemHardwareConfig,
};
use calldwell::with_rtt_out;
use core::fmt::Write;
use rt::entry;

fn perform_status_test(pmc: &mut PMC) {
    let current_status = pmc.status();

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
            "unexpected status of PMC, expected {:#?}, got {:#?}",
            default_status, current_status
        );
    }

    // `status` method is tested by proxy in every function that configures clocks which
    // state is represented by flags in status register, and changing status can be performed
    // only by configuration. Therefore, there's no point in writing specific tests for `status`,
    // as the issues should be detected by configuration tests.

    with_rtt_out(|w, _| w.write_str("status test successful"));
}

fn perform_interrupt_config_test(pmc: &mut PMC) {
    pmc.enable_interrupts(ClockInterrupts::all());
    let masks_post_disable = pmc.interrupts_masks();

    if masks_post_disable != ClockInterrupts::all() {
        panic!(
            "all interrupts were expected to be enabled, but instead got {:#?}",
            masks_post_disable
        );
    }

    pmc.disable_interrupts(ClockInterrupts::all());
    let masks_post_disable = pmc.interrupts_masks();

    if masks_post_disable != ClockInterrupts::none() {
        panic!(
            "all interrupts were expected to be disabled, but instead got {:#?}",
            masks_post_disable
        );
    }

    with_rtt_out(|w, _| w.write_str("interrupts config test successful"));
}

fn send_measured_main_rc_frequency(pmc: &mut PMC) {
    let main_rc_frequency = pmc.measure_main_rc_frequency();
    with_rtt_out(|w, _| write!(w.writer(), "{}", main_rc_frequency.to_Hz()).unwrap());
}

fn validate_main_rc_frequency(pmc: &PMC, expected_frequency: MainRcFrequency) {
    let read_main_rc_frequency = pmc.main_rc_frequency();
    if read_main_rc_frequency != expected_frequency {
        panic!(
            "unexpected default main RC frequency, expected {:#?}, got {:#?}",
            expected_frequency, read_main_rc_frequency
        );
    }
}

fn perform_main_rc_config_test(pmc: &mut PMC) {
    // By default the frequency should be set to 12MHz
    validate_main_rc_frequency(pmc, MainRcFrequency::MainRc12MHz);
    send_measured_main_rc_frequency(pmc);

    // Change it to 8MHz and verify it
    pmc.set_main_rc_frequency(MainRcFrequency::MainRc8MHz);
    validate_main_rc_frequency(pmc, MainRcFrequency::MainRc8MHz);
    send_measured_main_rc_frequency(pmc);

    // Change it to 4MHz and verify it
    pmc.set_main_rc_frequency(MainRcFrequency::MainRc4MHz);
    validate_main_rc_frequency(pmc, MainRcFrequency::MainRc4MHz);
    send_measured_main_rc_frequency(pmc);

    // Restore original configuration
    pmc.set_main_rc_frequency(MainRcFrequency::MainRc12MHz);
    validate_main_rc_frequency(pmc, MainRcFrequency::MainRc12MHz);
    send_measured_main_rc_frequency(pmc);
}

fn perform_master_clock_config_test(pmc: &mut PMC) {
    // Fetch and validate default configuration
    let original_master_clock_config = pmc.master_clock_config();
    let expected_master_clock_config = MasterClockConfig {
        source: MasterClockSource::MainClock,
        prescaler: ProcessorClockPrescaler::NoDivision,
        divider: MasterClockDivider::NoDivision,
    };

    if original_master_clock_config != expected_master_clock_config {
        panic!(
            "unexpected default master clock configuration, expected {:#?}, got {:#?}",
            expected_master_clock_config, original_master_clock_config
        );
    }

    // Change MCU master clock configuration
    let new_master_clock_config = MasterClockConfig {
        source: MasterClockSource::MainClock,
        prescaler: ProcessorClockPrescaler::DivBy2,
        divider: MasterClockDivider::DivBy4,
    };

    pmc.configure_master_clock(new_master_clock_config);
    if !pmc.status().master_clock_ready {
        panic!("master clock is not ready after setting it's new configuration");
    }

    let read_master_clock_config = pmc.master_clock_config();
    if read_master_clock_config != new_master_clock_config {
        panic!(
            "changing master clock configuration failed, expected {:#?}, got {:#?}",
            new_master_clock_config, read_master_clock_config
        );
    }

    // Restore previous configuration
    pmc.configure_master_clock(original_master_clock_config);
    if !pmc.status().master_clock_ready {
        panic!("master clock is not ready after restoring it's previous configuration");
    }

    let read_master_clock_config = pmc.master_clock_config();
    if read_master_clock_config != original_master_clock_config {
        panic!(
            "restoring master clock configuration failed, expected {:#?}, got {:#?}",
            original_master_clock_config, read_master_clock_config
        );
    }

    with_rtt_out(|w, _| w.write_str("master clock test successful"));
}

fn perform_processor_clock_status_test(pmc: &PMC) {
    if !pmc.processor_clock_enabled() {
        panic!("processor clock disabled, yet nothing has disabled it explicitly");
    }
    with_rtt_out(|w, _| w.write_str("processor clock status test successful"));
}

fn perform_programmable_clocks_config_test(pmc: &mut PMC) {
    // PCK6 is enabled because Aerugo is using it internally.
    let expected_status = [false, false, false, false, false, false, true, false];
    let actual_status = pmc.programmable_clocks_status();
    if actual_status != expected_status {
        panic!(
            "unexpected programmable clocks status, expected {:#?}, got {:#?}",
            expected_status, actual_status
        );
    }

    let pck_config = PCKConfig {
        source: PCKSource::SlowClock,
        prescaler: PCKPrescaler::new(10).unwrap(),
    };
    let tested_clock = PCK::PCK2;

    // Configure PCK2 and validate the configuration
    pmc.configure_programmable_clock(tested_clock, pck_config);
    let actual_config = pmc.programmable_clock_config(tested_clock);
    if pck_config != actual_config {
        panic!(
            "unexpected configuration read from {:#?}, expected {:#?}, got {:#?}",
            tested_clock, pck_config, actual_config
        );
    }

    pmc.enable_programmable_clock(tested_clock);
    if !pmc.programmable_clock_enabled(tested_clock) {
        panic!(
            "{:#?} was enabled, but clock controller reports it's disabled",
            tested_clock
        );
    }

    pmc.disable_programmable_clock(tested_clock);
    if pmc.programmable_clock_enabled(tested_clock) {
        panic!(
            "{:#?} was disabled, but clock controller reports it's enabled",
            tested_clock
        );
    }

    with_rtt_out(|w, _| w.write_str("programmable clocks config test successful"));
}

fn perform_peripheral_clocks_config_test(pmc: &mut PMC) {
    // I2S is selected because it's the only peripheral that supports generic clock configuration.
    // Other peripherals will simply not react to generic clock settings.
    let tested_peripheral = PeripheralId::I2SC0;

    // Clock should be disabled by default
    let default_clock_config = pmc.peripheral_clocks_config(tested_peripheral);
    let expected_clock_config = PeripheralClockConfig::default();

    if default_clock_config != expected_clock_config {
        panic!(
            "default clock config for {:#?} is invalid, expected {:#?}, got {:#?}",
            tested_peripheral, expected_clock_config, default_clock_config
        );
    }

    // Enable peripheral clock
    pmc.enable_peripheral_clock(tested_peripheral);
    if !pmc.peripheral_clocks_config(tested_peripheral).enabled {
        panic!(
            "peripheral clock for {:#?} was enabled, but configuration indicates it's disabled",
            tested_peripheral
        );
    }

    // Disable peripheral clock
    pmc.disable_peripheral_clock(tested_peripheral);
    if pmc.peripheral_clocks_config(tested_peripheral).enabled {
        panic!(
            "peripheral clock for {:#?} was disabled, but configuration indicates it's enabled",
            tested_peripheral
        );
    }

    // Configure generic clock
    let expected_generic_clock_config = GenericClockConfig {
        enabled: true,
        source: GenericClockSource::MasterClock,
        divider: GenericClockDivider::new(123).unwrap(),
    };
    let expected_peripheral_clock_config = PeripheralClockConfig {
        enabled: true,
        generic_clock: expected_generic_clock_config,
    };

    pmc.configure_peripheral_clocks(tested_peripheral, expected_peripheral_clock_config);

    // Validate the configuration
    let read_config = pmc.peripheral_clocks_config(tested_peripheral);
    if read_config != expected_peripheral_clock_config {
        panic!(
            "peripheral clock config for {:#?} invalid, expected {:#?}, got {:#?}",
            tested_peripheral, expected_peripheral_clock_config, read_config
        );
    }

    // Check if generic clock configuration stays in the register after peripheral clock is disabled
    pmc.disable_peripheral_clock(tested_peripheral);
    let read_config = pmc.peripheral_clocks_config(tested_peripheral);
    if (read_config.generic_clock.source != expected_generic_clock_config.source)
        || (read_config.generic_clock.divider != expected_generic_clock_config.divider)
    {
        panic!(
            "generic clock configuration (excluding state) is not retained after disabling peripheral clock, expected {:#?}, got {:#?}", 
            expected_generic_clock_config,
            read_config.generic_clock
        );
    }

    with_rtt_out(|w, _| w.write_str("peripheral clocks config test successful"));
}

fn perform_pmc_test(mut pmc: PMC) {
    perform_status_test(&mut pmc);
    perform_interrupt_config_test(&mut pmc);
    perform_main_rc_config_test(&mut pmc);
    perform_master_clock_config_test(&mut pmc);
    perform_processor_clock_status_test(&pmc);
    perform_programmable_clocks_config_test(&mut pmc);
    perform_peripheral_clocks_config_test(&mut pmc);
    with_rtt_out(|w, _| w.write_str("all tests finished successfully"));
}

#[entry]
fn main() -> ! {
    calldwell::start_session();

    let (aerugo, peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    let pmc = peripherals
        .pmc
        .expect("clocks controller missing from peripherals structure");

    perform_pmc_test(pmc);

    aerugo.start();
}
