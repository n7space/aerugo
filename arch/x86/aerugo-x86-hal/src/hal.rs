//! System HAL implementation for x86 target.

use std::convert::TryInto;
use std::time::SystemTime;

use aerugo_hal::SystemInstant;
use aerugo_hal::{AerugoHal, SystemHardwareConfig};
use bare_metal::CriticalSection;
use once_cell::sync::Lazy;

use crate::error::HalError;
use crate::user_peripherals::UserPeripherals;

/// Time when system was started
static TIME_START: Lazy<SystemTime> = Lazy::new(SystemTime::now);

/// HAL implementation for x86.
pub struct Hal {}

impl Hal {
    /// Returns empty UserPeripherals struct, as x86 does not provide any.
    pub fn create_user_peripherals() -> Option<UserPeripherals> {
        Some(UserPeripherals {})
    }
}

impl AerugoHal for Hal {
    type Error = HalError;

    fn configure_hardware(_config: SystemHardwareConfig) -> Result<(), HalError> {
        // There is no hardware to configure on x86
        Ok(())
    }

    fn get_system_time() -> SystemInstant {
        SystemInstant::from_ticks(
            TIME_START
                .elapsed()
                .expect("{}")
                .as_nanos()
                .try_into()
                .unwrap(),
        )
    }

    fn feed_watchdog() {
        // There is no watchdog for x86 target.
    }

    fn enter_critical() {
        // There is no critical section implementation for x86 target.
    }

    fn exit_critical() {
        // There is no critical section implementation for x86 target.
    }

    fn execute_critical<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        // There is no critical section implementation for x86 target.
        f(unsafe { &CriticalSection::new() })
    }
}
