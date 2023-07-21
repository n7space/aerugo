//! System HAL implementation for Cortex-M SAMV71 target.

use aerugo_hal::system_hal::{SystemHal, SystemHardwareConfig};
use bare_metal::CriticalSection;

use crate::peripherals::Peripherals;
use internal_cell::InternalCell;

/// HAL implementation for Cortex-M SAMV71.
pub struct Hal {
    /// Hardware peripherals.
    peripherals: InternalCell<Option<Peripherals>>,
}

impl Hal {
    /// Frequency for the time types (TODO)
    const TIMER_FREQ: u32 = 1_000_000;

    /// Create new HAL instance.
    pub fn new(peripherals: Peripherals) -> Self {
        Hal {
            peripherals: InternalCell::new(Some(peripherals)),
        }
    }

    /// Returns PAC peripherals for the user. Can be called successfully only once.
    pub fn peripherals(&self) -> Option<Peripherals> {
        // SAFETY: This is safe, because HAL design guarantees that no other
        // references to `self.peripherals` exist when this function is called.
        unsafe { self.peripherals.as_mut_ref().take() }
    }
}

impl SystemHal for Hal {
    type Instant = crate::time::TimerInstantU64<{ Hal::TIMER_FREQ }>;
    type Duration = crate::time::TimerDurationU64<{ Hal::TIMER_FREQ }>;

    fn configure_hardware(&mut self, _config: SystemHardwareConfig) {
        todo!()
    }

    fn get_system_time(&self) -> Self::Instant {
        crate::time::TimerInstantU64::from_ticks(0) // TODO: replace this stub with correct implementation
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
