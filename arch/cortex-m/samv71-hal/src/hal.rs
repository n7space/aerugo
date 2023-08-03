//! System HAL implementation for Cortex-M SAMV71 target.

use aerugo_cortex_m::Mutex;
use aerugo_hal::system_hal::{SystemHal, SystemHardwareConfig};
use bare_metal::CriticalSection;

use crate::drivers::watchdog::watchdog_config::WatchdogConfig;
use crate::drivers::watchdog::Watchdog;
use crate::error::HalError;
use crate::system_peripherals::SystemPeripherals;
use crate::user_peripherals::UserPeripherals;
use internal_cell::InternalCell;
use pac;

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

        let system_peripherals = SystemPeripherals {
            watchdog: Watchdog::new(mcu_peripherals.WDT),
        };

        let user_peripherals = UserPeripherals {
            chip_id: Some(mcu_peripherals.CHIPID),
            timer_counter1: Some(mcu_peripherals.TC1),
            timer_counter2: Some(mcu_peripherals.TC2),
            timer_counter3: Some(mcu_peripherals.TC3),
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

        Ok(())
    }

    fn get_system_time(&self) -> Self::Instant {
        crate::time::TimerInstantU64::from_ticks(0) // TODO: replace this stub with correct implementation
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
