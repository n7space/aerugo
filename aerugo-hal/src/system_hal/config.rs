//! System HAL configuration structures.

use fugit::MillisDurationU32;

/// System hardware configuration.
pub struct SystemHardwareConfig {
    /// Timeout for the watchdog.
    _watchdog_timeout: MillisDurationU32,
}
