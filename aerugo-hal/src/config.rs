//! System HAL configuration structures.

use crate::time;

/// System hardware configuration.
pub struct SystemHardwareConfig {
    /// Timeout for the watchdog.
    pub watchdog_timeout: time::MillisDurationU32,
}

impl Default for SystemHardwareConfig {
    fn default() -> Self {
        SystemHardwareConfig {
            watchdog_timeout: time::MillisDurationU32::secs(3),
        }
    }
}
