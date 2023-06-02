//! System HAL implementation for x86 target.

mod peripherals;

pub use self::peripherals::Peripherals;

use aerugo_hal::system_hal::SystemHal;

/// HAL implementation for x86.
pub struct Hal {
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

    /// Gets current system time timestamp.
    fn get_system_time(&'static self) -> Self::Instant {
        todo!()
    }
}
