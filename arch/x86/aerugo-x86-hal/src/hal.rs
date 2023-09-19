//! System HAL implementation for x86 target.

use std::convert::TryInto;
use std::time::SystemTime;

use aerugo_hal::{AerugoHal, Duration, Instant, SystemHardwareConfig};
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

    fn get_system_time() -> Instant {
        let duration = Duration::nanos(
            TIME_START
                .elapsed()
                .expect("{}")
                .as_nanos()
                .try_into()
                .unwrap(),
        );

        Instant::from_ticks(duration.ticks())
    }

    fn feed_watchdog() {
        // There is no watchdog for x86 target.
    }
}
