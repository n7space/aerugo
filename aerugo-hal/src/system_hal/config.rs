//! System HAL configuration structures.

/// System hardware configuration.
pub struct SystemHardwareConfig {
    /// Timeout for the watchdog.
    pub watchdog_timeout: crate::Duration,
    /// If true, all interrupts will be disabled until `AERUGO.start()` is called.
    pub disable_interrupts_during_setup: bool,
}

impl Default for SystemHardwareConfig {
    fn default() -> Self {
        SystemHardwareConfig {
            watchdog_timeout: crate::Duration::secs(1),
            disable_interrupts_during_setup: true,
        }
    }
}
