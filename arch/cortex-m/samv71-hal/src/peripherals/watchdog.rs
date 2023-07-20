//! Implementation of HAL for Watchdog

use embedded_hal::watchdog::WatchdogDisable;

use crate::embedded_hal::watchdog;
use crate::pac::WDT;

/// Structure representing a watchdog
pub struct Watchdog {
    /// Watchdog instance
    wdt: WDT,
    /// Indicates whether the watchdog has been configured (or disabled).
    ///
    /// Note that watchdog can be configured or disabled only once.
    /// Configuration is locked until MCU performs a hard reset.
    /// This flag prevents it from being configured twice.
    configured: bool,
}

/// SAFETY: Watchdog does not auto-implement Sync due to WDT structure.
/// Since it owns WDT, and it's running in single-core environment, it's safe to share.
unsafe impl Sync for Watchdog {}

/// Structure representing Watchdog configuration.
///
/// Note that watchdog can be configured only once.
/// Configuration is locked until MCU performs a hard reset.
pub struct WatchdogConfiguration {
    /// If true, watchdog stays enabled.
    pub enabled: bool,
    /// If true, watchdog will reset the MCU on timeout.
    pub reset_enabled: bool,
    /// Defines the counter value for watchdog.
    pub counter: u16,
    /// If true, watchdog will run in idle state.
    pub run_in_idle: bool,
    /// If true, watchdog will run in debug state.
    pub run_in_debug: bool,
    /// If true, watchdog underflow or error will trigger interrupt.
    pub interrupt_enabled: bool,
}

impl Default for WatchdogConfiguration {
    fn default() -> Self {
        WatchdogConfiguration {
            enabled: true,
            reset_enabled: true,
            counter: 0xFFF,
            run_in_idle: false,
            run_in_debug: false,
            interrupt_enabled: false,
        }
    }
}

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
    pub fn configure(&mut self, configuration: WatchdogConfiguration) {
        if self.configured {
            return;
        }

        // It's unsafe per SAMV71 documentation to modify configuration and enable/disable
        // watchdog at the same time, therefore disabling is handled separately.
        if !configuration.enabled {
            self.disable();
            return;
        }

        let clamped_counter_value = configuration.counter.clamp(0, 2u16.pow(12) - 1);

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
    }
}

impl watchdog::Watchdog for Watchdog {
    fn feed(&mut self) {
        self.wdt.cr.write(|w| w.key().passwd().wdrstt().set_bit());
    }
}

impl watchdog::WatchdogDisable for Watchdog {
    /// Disables the watchdog.
    ///
    /// Note that watchdog can be configured or disabled only once.
    /// Once disabled, it's off until the MCU performs a hard reset.
    fn disable(&mut self) {
        if self.configured {
            return;
        }

        self.wdt.mr.modify(|_, w| w.wddis().set_bit());
        self.configured = true;
    }
}
