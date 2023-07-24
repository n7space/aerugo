//! Module containing watchdog configuration types.

/// Structure representing Watchdog configuration.
///
/// Note that watchdog can be configured only once.
/// Subsequent configuration is locked until MCU performs a hard reset.
pub struct WatchdogConfig {
    /// If true, watchdog stays enabled.
    pub enabled: bool,
    /// If true, watchdog will reset the MCU on timeout.
    pub reset_enabled: bool,
    /// Defines the reset value for watchdog's counter in watchdog clock cycles.
    pub duration: u16,
    /// If true, watchdog will run in idle state.
    pub run_in_idle: bool,
    /// If true, watchdog will run in debug state.
    pub run_in_debug: bool,
    /// If true, watchdog underflow or error will trigger interrupt.
    pub interrupt_enabled: bool,
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        WatchdogConfig {
            enabled: true,
            reset_enabled: true,
            duration: 0xFFF,
            run_in_idle: false,
            run_in_debug: false,
            interrupt_enabled: false,
        }
    }
}
