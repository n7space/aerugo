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
    peripherals: Option<Peripherals>,
}

impl Hal {
    /// Frequency for the time types (TODO)
    const TIMER_FREQ: u32 = 1_000_000;

    /// Create new HAL instance.
    pub fn new(peripherals: Peripherals) -> Self {
        Hal {
            peripherals: Some(peripherals),
        }
    }

    /// Returns PAC peripherals for the user. Can be called successfully only once.
    pub fn peripherals(&mut self) -> Option<Peripherals> {
        self.peripherals.take()
    }
}

impl SystemHal for Hal {
    type Instant = crate::time::TimerInstantU64<{ Hal::TIMER_FREQ }>;
    type Duration = crate::time::TimerDurationU64<{ Hal::TIMER_FREQ }>;

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
