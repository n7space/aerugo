//! System HAL implementation for x86 target.

use aerugo_hal::system_hal::{SystemHal, SystemHardwareConfig};
use bare_metal::CriticalSection;

use crate::peripherals::Peripherals;

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
        todo!()
    }

    fn feed_watchdog(&mut self) {
        todo!()
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
