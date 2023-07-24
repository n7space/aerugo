//! Module representing peripherals internally used by Aerugo.

use crate::drivers::watchdog::Watchdog;

/// System peripherals structure. These peripherals are represented as HAL drivers.
/// They are initialized on system init, and used directly by HAL to provide core functionality.
pub struct SystemPeripherals {
    /// Watchdog instance.
    pub watchdog: Watchdog,
}
