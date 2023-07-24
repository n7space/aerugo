//! Module containing watchdog configuration types.

use fugit::MillisDurationU32;

/// Maximum duration that watchdog can wait before triggering an event.
pub const MAXIMUM_WATCHDOG_DURATION: MillisDurationU32 = MillisDurationU32::secs(16);

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
    pub duration: MillisDurationU32,
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
            duration: MAXIMUM_WATCHDOG_DURATION,
            run_in_idle: false,
            run_in_debug: false,
            interrupt_enabled: false,
        }
    }
}
