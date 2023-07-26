//! System HAL configuration structures.

use fugit::MillisDurationU32;

/// System hardware configuration.
pub struct SystemHardwareConfig {
    /// Timeout for the watchdog.
    pub watchdog_timeout: MillisDurationU32,
}

impl Default for SystemHardwareConfig {
    fn default() -> Self {
        SystemHardwareConfig {
            watchdog_timeout: MillisDurationU32::secs(1),
        }
    }
}
