//! System HAL implementation for x86 target.

use std::convert::TryInto;
use std::time::SystemTime;

use aerugo_hal::system_hal::{SystemHal, SystemHardwareConfig};
use bare_metal::CriticalSection;
use once_cell::sync::Lazy;

use crate::peripherals::Peripherals;

/// Time when system was started
static TIME_START: Lazy<SystemTime> = Lazy::new(SystemTime::now);

/// HAL implementation for x86.
pub struct Hal {
    /// Hardware peripherals.
    _peripherals: Peripherals,
}

impl Hal {
    /// Create new HAL instance.
    pub const fn new(peripherals: Peripherals) -> Self {
        Hal {
            _peripherals: peripherals,
        }
    }
}

impl SystemHal for Hal {
    type Instant = crate::time::TimerInstantU64<1_000_000>;
    type Duration = crate::time::TimerDurationU64<1_000_000>;

    fn configure_hardware(&mut self, _config: SystemHardwareConfig) {
        todo!()
    }

    fn get_system_time(&self) -> Self::Instant {
        Self::Instant::from_ticks(
            TIME_START
                .elapsed()
                .expect("{}")
                .as_nanos()
                .try_into()
                .unwrap(),
        )
    }

    fn feed_watchdog(&mut self) {
        todo!()
    }

    fn enter_critical()
    where
        Self: Sized,
    {
        // There is no critical section implementation for x86 target.
    }

    fn exit_critical()
    where
        Self: Sized,
    {
        // There is no critical section implementation for x86 target.
    }

    fn execute_critical<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
        Self: Sized,
    {
        // There is no critical section implementation for x86 target.
        f(unsafe { &CriticalSection::new() })
    }
}
