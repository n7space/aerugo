//! Implementation of HAL for Watchdog
//!
//! Watchdog can be used to unconditionally reset or interrupt the MCU when program halts.
//! To indicate that program is running, watchdog needs to be "fed" periodically.
//! Minimal frequency of feeding can be adjusted by setting watchdog counter reset value (duration).
//! Behavior of watchdog, it's state and duration can be set using provided API.
//!
//! # Implementation remarks
//! On SAMV71, Watchdog is enabled by default with duration of 16 seconds,
//! and can only be configured ONCE. Consequent configurations will have no effect,
//! until the MCU performs a hard reset (via reset controller or power cycle).

use crate::pac::WDT;

/// Possible watchdog errors
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WatchdogError {
    /// Tried to configure watchdog more than once.
    WatchdogAlreadyConfigured,
}

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

/// Structure representing a watchdog
pub struct Watchdog {
    /// Watchdog instance
    wdt: WDT,
    /// Indicates whether the watchdog has already been configured (or disabled).
    configured: bool,
}

/// SAFETY: Watchdog does not auto-implement Sync due to WDT structure.
/// Since it owns WDT, and it's running in single-core environment, it's safe to share.
unsafe impl Sync for Watchdog {}

impl Watchdog {
    /// Create a watchdog instance
    pub const fn new(wdt: WDT) -> Self {
        Self {
            wdt,
            configured: false,
        }
    }

    /// Set watchdog configuration
    ///
    /// Note that watchdog can be configured only once.
    /// Configuration is locked until MCU performs a hard reset.
    pub fn configure(&mut self, configuration: WatchdogConfig) -> Result<(), WatchdogError> {
        if self.configured {
            return Err(WatchdogError::WatchdogAlreadyConfigured);
        }

        // It's unsafe per SAMV71 documentation to modify configuration and enable/disable
        // watchdog at the same time, therefore disabling is handled separately.
        if !configuration.enabled {
            self.disable()?;
            return Ok(());
        }

        let clamped_counter_value = configuration.duration.clamp(0, 2u16.pow(12) - 1);

        // SAFETY: WDV is 12-bit field, value from configuration is clamped to (2^12)-1
        self.wdt.mr.modify(|_, w| unsafe {
            w.wdidlehlt()
                .variant(!configuration.run_in_idle)
                .wddbghlt()
                .variant(!configuration.run_in_debug)
                .wdd()
                .bits(clamped_counter_value)
                .wdrsten()
                .bit(configuration.reset_enabled)
                .wdfien()
                .variant(configuration.interrupt_enabled)
                .wdv()
                .bits(clamped_counter_value)
        });

        self.configured = true;
        Ok(())
    }

    /// Returns true if watchdog has been already configured, false otherwise.
    pub fn was_configured(&self) -> bool {
        self.configured
    }

    /// Refresh the watchdog counter.
    pub fn feed(&mut self) {
        self.wdt.cr.write(|w| w.key().passwd().wdrstt().set_bit());
    }

    /// Disables the watchdog.
    ///
    /// Note that watchdog can be configured or disabled only once.
    /// Once disabled, it's off until the MCU performs a hard reset.
    pub fn disable(&mut self) -> Result<(), WatchdogError> {
        if self.configured {
            return Err(WatchdogError::WatchdogAlreadyConfigured);
        }

        self.wdt.mr.modify(|_, w| w.wddis().set_bit());
        self.configured = true;

        Ok(())
    }
}
