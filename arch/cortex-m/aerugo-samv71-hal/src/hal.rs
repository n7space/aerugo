//! System HAL implementation for Cortex-M SAMV71 target.

use aerugo_hal::critical_section;
use aerugo_hal::{AerugoHal, CriticalSection, Instant, SystemHardwareConfig};

use crate::cortex_m;
use crate::error::HalError;
use crate::system_peripherals::SystemPeripherals;
use crate::user_peripherals::UserPeripherals;
use samv71_hal::pac::{self, PMC, TC0};
use samv71_hal::timer::channel_config::ChannelClock;
use samv71_hal::timer::timer_config::{ExternalClock, ExternalClockSource};
use samv71_hal::timer::waveform_config::{
    ComparisonEffect, OutputSignalEffects, WaveformModeConfig,
};
use samv71_hal::timer::{Ch0, Ch1, Ch2, Channel, Timer, Waveform};
use samv71_hal::watchdog::{Watchdog, WatchdogConfig};

/// Global system peripherals instance, used internally by HAL.
///
/// # Safety
/// Mutex is not used here, because it would imply a critical section at every access to HAL.
/// Safety of this cell is managed by HAL instead, guaranteeing that undefined behavior will not occur.
static mut HAL_SYSTEM_PERIPHERALS: Option<SystemPeripherals> = None;

/// HAL implementation for Cortex-M based SAMV71 MCU.
pub struct Hal;

impl Hal {
    /// Creates user peripherals instance.
    ///
    /// This function steals PAC peripherals and returns a [`UserPeripherals`] structure
    /// containing all peripherals that are available to user via HAL drivers.
    ///
    /// Some of these peripherals are taken from SystemPeripherals structure, hence
    /// this function should not be called before finishing HAL initialization (via
    /// [`AerugoHal::configure_hardware] function).
    ///
    /// This function executes in critical section, as it modifies HAL_SYSTEM_PERIPHERALS.
    ///
    /// # Safety
    /// This function can be called successfully only once, after HAL initialization.
    /// If called before that, or multiple times, it will return [`None`], as some of
    /// the required peripherals will be missing.
    ///
    /// # Returns
    /// [`Some(UserPeripherals)`] if called for the first time after HAL initialization,
    /// [`None`] otherwise.
    pub fn create_user_peripherals() -> Option<UserPeripherals> {
        Hal::execute_critical(|_| {
            if let Some(system_peripherals) = unsafe { &mut HAL_SYSTEM_PERIPHERALS } {
                let mcu_peripherals = unsafe { pac::Peripherals::steal() };
                let core_peripherals = unsafe { pac::CorePeripherals::steal() };

                // Check if PMC is available, return `None` if it's not.
                system_peripherals.pmc.as_ref()?;

                Some(UserPeripherals {
                    chip_id: Some(mcu_peripherals.CHIPID),
                    timer_counter1: Some(mcu_peripherals.TC1),
                    timer_counter2: Some(mcu_peripherals.TC2),
                    timer_counter3: Some(mcu_peripherals.TC3),
                    pmc: system_peripherals.pmc.take(),
                    nvic: Some(core_peripherals.NVIC),
                })
            } else {
                None
            }
        })
    }

    /// Initializes global HAL instance using PAC peripherals.
    ///
    /// Calling this function begins HAL initialization process. This process must be finished
    /// by calling [`AerugoHal::configure_hardware`]. Until then, no other HAL functions should
    /// be called, as they will most likely fail.
    ///
    /// # Safety
    /// This function is safe to call only once. It should be called in critical section.
    /// Subsequent calls will return an error, indicating that HAL instance has already been created.
    ///
    /// # Return
    /// `()` on success, [`HalError::HalAlreadyInitialized`] if called more than once.
    fn initialize() -> Result<(), HalError> {
        // SAFETY:
        // This function can be successfully called only once, and we're in critical section,
        // so there's no possible way that this memory will accessed somewhere else until this
        // section is finished.
        let is_hal_created = unsafe { HAL_SYSTEM_PERIPHERALS.is_some() };
        if is_hal_created {
            return Err(HalError::HalAlreadyInitialized);
        }

        unsafe { HAL_SYSTEM_PERIPHERALS.replace(Hal::create_system_peripherals()) };

        Ok(())
    }

    /// Creates system peripherals of HAL.
    ///
    /// This function steals PAC peripherals and returns a [`SystemPeripherals`] structure
    /// containing peripherals used by [`AerugoHal`] API implementation.
    ///
    /// # Safety
    /// This function should be only called once inside [`Hal::initialize`].
    /// Subsequent calls will return valid peripherals, but it's not possible to
    /// guarantee safety if multiple instances of peripherals are used in the system.
    fn create_system_peripherals() -> SystemPeripherals {
        let mcu_peripherals = unsafe { pac::Peripherals::steal() };

        SystemPeripherals {
            watchdog: Watchdog::new(mcu_peripherals.WDT),
            timer: Timer::new(mcu_peripherals.TC0),
            timer_ch0: None,
            timer_ch1: None,
            timer_ch2: None,
            pmc: Some(mcu_peripherals.PMC),
        }
    }
}

impl AerugoHal for Hal {
    type Error = HalError;

    /// This function performs SAMV71 hardware configuration required for the HAL to work correctly.
    /// It also initializes the HAL.
    ///
    /// This function executes in critical section, as it modifies HAL_SYSTEM_PERIPHERALS.
    ///
    /// # Safety
    /// This function is safe to call only once.
    /// Subsequent calls will return an error, indicating that HAL instance has already been initialized.
    ///
    /// # Return
    /// `()` on success, [`HalError`] if HAL was already initialized.
    fn configure_hardware(config: SystemHardwareConfig) -> Result<(), HalError> {
        let result = Hal::execute_critical(|_| {
            Hal::initialize()?;

            // SAFETY: Immutable access to system peripherals is safe, as we're in critical section
            // of single-core MCU and no other references to peripherals should exist at this time.
            let is_hal_created = unsafe { HAL_SYSTEM_PERIPHERALS.is_some() };
            if !is_hal_created {
                return Err(HalError::HalNotInitialized);
            }

            // SAFETY: Mutable access to system peripherals is safe, as we're in critical section
            // of single-core MCU and no other references to peripherals should exist at this time.
            // We also checked that peripherals exist, so it should realistically never panic.
            let peripherals = unsafe {
                HAL_SYSTEM_PERIPHERALS
                    .as_mut()
                    .expect("HAL is not initialized")
            };

            if peripherals.pmc.is_none() {
                // If PMC is not available, it means that system has already been initialized,
                // so this function cannot proceed.
                return Err(HalError::HardwareAlreadyInitialized);
            }
            // This should realistically never panic, as we checked the existence of PMC earlier.
            let pmc = peripherals
                .pmc
                .as_ref()
                .expect("PMC is missing from system peripherals");

            // Configure watchdog
            match peripherals.watchdog.configure(WatchdogConfig {
                duration: config.watchdog_timeout,
                ..Default::default()
            }) {
                Ok(()) => {}
                Err(_) => return Err(HalError::HardwareAlreadyInitialized),
            };

            // Configure system timer
            let (ch0, ch1, ch2) = configure_timer(&mut peripherals.timer, pmc);

            peripherals.timer_ch0.replace(ch0);
            peripherals.timer_ch1.replace(ch1);
            peripherals.timer_ch2.replace(ch2);

            // Start system timer
            peripherals.timer.trigger_all_channels();

            Ok(())
        });

        if config.disable_interrupts_during_setup {
            Hal::enter_critical();
        }

        result
    }

    fn get_system_time() -> Instant {
        // SAFETY: This is safe, because this is a single-core system, and no other references to
        // system peripherals should exist during this call.
        let peripherals = unsafe {
            HAL_SYSTEM_PERIPHERALS
                .as_ref()
                .expect("HAL cannot be accessed before initialization")
        };

        let ch0 = peripherals
            .timer_ch0
            .as_ref()
            .expect("get_system_time called before HAL initialization");
        let ch1 = peripherals
            .timer_ch1
            .as_ref()
            .expect("get_system_time called before HAL initialization");
        let ch2 = peripherals
            .timer_ch2
            .as_ref()
            .expect("get_system_time called before HAL initialization");

        let time_ch2 = ch2.counter_value();
        let time_ch1 = ch1.counter_value();
        let time_ch0 = ch0.counter_value();

        // Timer's clock is 1MHz, so returned value is in microseconds.
        Instant::from_ticks(as_48bit_unsigned(time_ch0, time_ch1, time_ch2))
    }

    fn feed_watchdog() {
        // SAFETY: This is safe, because this is a single-core system, and no other references to
        // system peripherals should exist during this call.
        let peripherals = unsafe {
            HAL_SYSTEM_PERIPHERALS
                .as_mut()
                .expect("HAL cannot be accessed before initialization")
        };

        peripherals.watchdog.feed();
    }

    /// Enters critical section by disabling global interrupts.
    fn enter_critical() {
        cortex_m::interrupt::disable();
    }

    /// Exits critical section by enabling global interrupts.
    ///
    /// # Safety
    /// <div class="warning">This function should never be called from scope-bound critical sections (like the one created with <code>AerugoHal::execute_critical</code>)</div>
    fn exit_critical() {
        unsafe { cortex_m::interrupt::enable() };
    }

    fn execute_critical<F, R>(f: F) -> R
    where
        F: FnOnce(CriticalSection) -> R,
    {
        critical_section::with(f)
    }
}

/// Type representing all TC0 channels in Waveform mode.
type Tc0Channels = (
    Channel<TC0, Ch0, Waveform>,
    Channel<TC0, Ch1, Waveform>,
    Channel<TC0, Ch2, Waveform>,
);

/// Configures a timer for HAL usage.
///
/// This function configures Timer (using hardware TC0 instance) in Waveform mode with proper
/// input clocks (configured via PMC), and chains it's channels to achieve high-resolution
/// time source for the system.
///
/// Timer's source clock first goes into channel 0, which generates RC compare events that
/// toggle it's TIOA0 output, effectively dividing the input frequency by the value of RC register.
/// TIOA0 is connected via XC1 to channel 1, which does the same thing for TIOA1 output, which is
/// connected via XC2 to channel 2.
///
/// # Parameters
/// * `timer` - HAL Timer instance
/// * `pmc` - PAC PMC instance
fn configure_timer(timer: &mut Timer<TC0>, pmc: &PMC) -> Tc0Channels {
    configure_timer_pmc(pmc);

    // If any of the configurations is not available, user cannot do anything about it and it
    // certainly should not pass any tests, so just hard fault it.
    timer
        .configure_external_clock_source(ExternalClock::XC1, ExternalClockSource::TIOA0)
        .expect("Cannot connect TIOA0 to XC1");
    timer
        .configure_external_clock_source(ExternalClock::XC2, ExternalClockSource::TIOA1)
        .expect("Cannot connect TIOA1 to XC2");

    // If any of the channels is not available, it's a hard fault as it's an internal bug in Aerugo
    let ch0 = timer.channel_0.take().expect("TC0 CH0 already taken");
    let ch1 = timer.channel_1.take().expect("TC0 CH1 already taken");
    let ch2 = timer.channel_2.take().expect("TC0 CH2 already taken");

    let waveform_config = WaveformModeConfig {
        tioa_effects: OutputSignalEffects {
            software_trigger: ComparisonEffect::Clear,
            rc_comparison: ComparisonEffect::Toggle,
            ..Default::default()
        },
        ..Default::default()
    };

    let ch0 = ch0.into_waveform_channel(waveform_config);
    let ch1 = ch1.into_waveform_channel(waveform_config);
    let ch2 = ch2.into_waveform_channel(waveform_config);

    // Set RC values for all channels to max, so we can achieve full 48-bit resolution
    ch0.set_rc(u16::MAX);
    ch1.set_rc(u16::MAX);
    ch2.set_rc(u16::MAX);

    ch0.set_clock_source(ChannelClock::PmcPeripheralClock);
    ch1.set_clock_source(ChannelClock::XC1);
    ch2.set_clock_source(ChannelClock::XC2);

    ch0.enable();
    ch1.enable();
    ch2.enable();

    (ch0, ch1, ch2)
}

/// Configures PMC for TC0 operation with 3 chained channels
///
/// Enables TC0 CH0, CH1 and CH2 peripheral clocks, and configures PCK6
/// to generate proper clock for the timers.
///
/// PCK6 uses MAINCK clock source (which is 12MHz by default), and divides it by 12 to get
/// 1MHz input clock, used by the timer to achieve 1ns resolution.
///
fn configure_timer_pmc(pmc: &PMC) {
    // Configure PCK6 for 1MHz TC0 output
    // Source: MAINCK (12MHz by default)
    // Divider: /6 (TODO: is there a hidden /2 prescaler somewhere?)
    pmc.pck[6].write(|w| w.css().main_clk().pres().variant(5));

    // Enable TC0 CH0, CH1 and CH2 peripheral clocks
    pmc.pcer0
        .write(|w| w.pid23().set_bit().pid24().set_bit().pid25().set_bit());

    // Enable PCK6
    pmc.scer.write(|w| w.pck6().set_bit());

    // Wait until PCK6 is ready
    while pmc.sr.read().pckrdy6().bit_is_clear() {
        cortex_m::asm::nop();
    }
}

/// Converts three 16-bit values into single 48-bit value.
///
/// Returns it as u64, shifted to left.
///
/// # Parameters
/// * `lsb` - Least significant bytes
/// * `mid` - Middle bytes
/// * `msb` - Most significant bytes
fn as_48bit_unsigned(lsb: u16, mid: u16, msb: u16) -> u64 {
    ((msb as u64) << 32) | ((mid as u64) << 16) | (lsb as u64)
}
