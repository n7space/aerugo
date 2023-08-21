//! System HAL implementation for Cortex-M SAMV71 target.

use aerugo_cortex_m::Mutex;
use aerugo_hal::system_hal::{SystemHal, SystemHardwareConfig};
use bare_metal::CriticalSection;

use cortex_m::asm;

use crate::drivers::timer::channel_config::ChannelClock;
use crate::drivers::timer::timer_config::{ExternalClock, ExternalClockSource};
use crate::drivers::timer::waveform_config::{
    ComparisonEffect, OutputSignalEffects, WaveformModeConfig,
};
use crate::drivers::timer::{Ch0, Ch1, Ch2, Channel, Timer, Waveform};
use crate::drivers::watchdog::watchdog_config::WatchdogConfig;
use crate::drivers::watchdog::Watchdog;
use crate::error::HalError;
use crate::system_peripherals::SystemPeripherals;
use crate::user_peripherals::UserPeripherals;
use internal_cell::InternalCell;
use pac::{self, PMC, TC0};

/// This lock will prevent from creating HAL instance twice in the system.
/// Since HAL manages the access to peripherals, creating and using multiple
/// instances of it could be unsafe.
static HAL_CREATION_LOCK: Mutex<bool> = Mutex::new(false);

/// HAL implementation for Cortex-M based SAMV71 MCU.
pub struct Hal {
    /// User-accessible peripherals.
    user_peripherals: Option<UserPeripherals>,
    /// System peripherals.
    system_peripherals: InternalCell<SystemPeripherals>,
}

impl Hal {
    /// Frequency for the time types (TODO)
    const TIMER_FREQ: u32 = 1_000_000;

    /// Create new HAL instance from PAC peripherals.
    ///
    /// # Safety
    /// This function is safe to call only once.
    /// Subsequent calls will return an error, indicating that HAL instance has already been created.
    ///
    /// # Return
    /// `Hal` if it's first call during the program lifetime, [`HalError::HalAlreadyCreated`] otherwise.
    pub fn new() -> Result<Self, HalError> {
        HAL_CREATION_LOCK.lock(|lock| {
            if *lock {
                return Err(HalError::HalAlreadyCreated);
            }

            *lock = true;
            Ok(())
        })?;

        let (user_peripherals, system_peripherals) = Hal::create_peripherals();
        Ok(Hal {
            user_peripherals: Some(user_peripherals),
            system_peripherals: InternalCell::new(system_peripherals),
        })
    }

    /// Create peripherals for HAL
    ///
    /// # Safety
    /// This function should be only called once inside `new`.
    /// Subsequent calls will return valid peripherals, but it's not possible to
    /// guarantee safety if multiple instances of peripherals are used in the system.
    fn create_peripherals() -> (UserPeripherals, SystemPeripherals) {
        let mcu_peripherals = unsafe { pac::Peripherals::steal() };
        let core_peripherals = unsafe { pac::CorePeripherals::steal() };

        let system_peripherals = SystemPeripherals {
            watchdog: Watchdog::new(mcu_peripherals.WDT),
            timer: Timer::new(mcu_peripherals.TC0),
            timer_ch0: None,
            timer_ch1: None,
            timer_ch2: None,
            pmc: Some(mcu_peripherals.PMC),
        };

        let user_peripherals = UserPeripherals {
            chip_id: Some(mcu_peripherals.CHIPID),
            timer_counter1: Some(mcu_peripherals.TC1),
            timer_counter2: Some(mcu_peripherals.TC2),
            timer_counter3: Some(mcu_peripherals.TC3),
            pmc: None,
            nvic: Some(core_peripherals.NVIC),
        };

        (user_peripherals, system_peripherals)
    }

    /// Returns PAC peripherals for the user
    ///
    /// # Safety
    /// Can be called successfully only once. Subsequent calls will return None.
    ///
    /// # Return
    /// [`UserPeripherals`] on first call, `None` on subsequent calls.
    pub fn user_peripherals(&mut self) -> Option<UserPeripherals> {
        self.user_peripherals.take()
    }
}

impl SystemHal for Hal {
    type Instant = crate::time::TimerInstantU64<{ Hal::TIMER_FREQ }>;
    type Duration = crate::time::TimerDurationU64<{ Hal::TIMER_FREQ }>;
    type Error = HalError;

    fn configure_hardware(&mut self, config: SystemHardwareConfig) -> Result<(), HalError> {
        // SAFETY: This is safe, because this is a single-core system,
        // and no other references to system peripherals should exist.
        let peripherals = unsafe { self.system_peripherals.as_mut_ref() };

        match peripherals.watchdog.configure(WatchdogConfig {
            duration: config.watchdog_timeout,
            ..Default::default()
        }) {
            Ok(()) => {}
            Err(_) => return Err(HalError::HalAlreadyConfigured),
        };

        if let Some(pmc) = peripherals.pmc.take() {
            let (ch0, ch1, ch2) = configure_timer_for_hal(&mut peripherals.timer, &pmc);

            peripherals.timer_ch0.replace(ch0);
            peripherals.timer_ch1.replace(ch1);
            peripherals.timer_ch2.replace(ch2);

            if let Some(user_peripherals) = self.user_peripherals.as_mut() {
                user_peripherals.pmc.replace(pmc);
            } else {
                // That should never happen, as both system and user peripherals are created at
                // the same time, but to prevent hard-to-detect issues in the future, this will
                // throw an error anyway.
                return Err(HalError::HalNotInitializedYet);
            }
        } else {
            // If PMC is not there, it means that the system has already been initialized.
            return Err(HalError::HalAlreadyConfigured);
        }

        // Start system timer
        peripherals.timer.trigger_all_channels();

        Ok(())
    }

    fn get_system_time(&self) -> Self::Instant {
        // SAFETY: This is safe, because this is a single-core system,
        // and no other references to system peripherals should exist.
        let peripherals = unsafe { self.system_peripherals.as_ref() };

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

        let time_ch0 = ch0.counter_value();
        let time_ch1 = ch1.counter_value();
        let time_ch2 = ch2.counter_value();

        // Timer's clock is 1MHz, so returned value is in microseconds.
        crate::time::TimerInstantU64::from_ticks(as_48bit_unsigned(time_ch0, time_ch1, time_ch2))
    }

    fn feed_watchdog(&mut self) {
        // SAFETY: This is safe, because this is a single-core system,
        // and no other references to system peripherals should exist.
        let peripherals = unsafe { self.system_peripherals.as_mut_ref() };

        peripherals.watchdog.feed();
    }

    fn enter_critical() {
        cortex_m::interrupt::disable();
    }

    fn exit_critical() {
        unsafe { cortex_m::interrupt::enable() };
    }

    fn execute_critical<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        cortex_m::interrupt::free(f)
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
fn configure_timer_for_hal(timer: &mut Timer<TC0>, pmc: &PMC) -> Tc0Channels {
    configure_pmc_for_timer(pmc);

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
fn configure_pmc_for_timer(pmc: &PMC) {
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
        asm::nop();
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
