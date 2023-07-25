//! Implementation of HAL Watchdog driver.
//!
//! Watchdog can be used to unconditionally reset or interrupt the MCU when program halts.
//! To indicate that program is running, watchdog needs to be "fed" periodically.
//! Minimal frequency of feeding can be adjusted by setting watchdog counter reset value (duration).
//! Behavior of watchdog, it's state, and duration can be set using provided API.
//!
//! # Implementation notes
//! On SAMV71, Watchdog is enabled by default with duration of 16 seconds,
//! and can only be configured ONCE. Consequent configurations will have no effect,
//! until the MCU performs a hard reset (via reset controller or power cycle).

pub mod watchdog_config;
pub mod watchdog_error;

use crate::pac::WDT;
use fugit::MillisDurationU32;
pub use watchdog_config::WatchdogConfig;
pub use watchdog_error::WatchdogError;

use self::watchdog_config::MAXIMUM_WATCHDOG_DURATION;

/// Structure representing a watchdog.
pub struct Watchdog {
    /// Watchdog instance.
    wdt: WDT,
    /// Indicates whether the watchdog has already been configured (or disabled).
    configured: bool,
}

/// # Safety
/// Watchdog does not auto-implement Sync due to WDT structure containing a pointer.
/// Since it owns WDT, and it's running in single-core environment, it's safe to share.
unsafe impl Sync for Watchdog {}

impl Watchdog {
    /// Create a watchdog instance from PAC peripheral.
    ///
    /// # Parameters
    /// * `wdt` - PAC Watchdog peripheral.
    pub const fn new(wdt: WDT) -> Self {
        Self {
            wdt,
            configured: false,
        }
    }

    /// Set watchdog configuration
    ///
    /// Note that watchdog can be configured only once.
    /// After that, configuration is locked until MCU performs a hard reset.
    ///
    /// # Parameters
    /// * `config` - Watchdog configuration.
    ///
    /// # Return
    /// [`WatchdogError::WatchdogAlreadyConfigured`] if watchdog instance was
    /// configured earlier, `Ok(())` otherwise.
    pub fn configure(&mut self, config: WatchdogConfig) -> Result<(), WatchdogError> {
        if self.configured {
            return Err(WatchdogError::WatchdogAlreadyConfigured);
        }

        // It's unsafe per SAMV71 documentation to modify configuration and enable/disable
        // watchdog at the same time, therefore disabling is handled separately.
        if !config.enabled {
            self.disable()?;
            return Ok(());
        }

        let raw_duration = Watchdog::clamp_and_convert_duration(config.duration);

        // SAFETY: WDV is 12-bit field, value from configuration is clamped to 0xFFF
        self.wdt.mr.modify(|_, w| unsafe {
            w.wdidlehlt()
                .variant(!config.run_in_idle)
                .wddbghlt()
                .variant(!config.run_in_debug)
                .wdd()
                .bits(raw_duration)
                .wdrsten()
                .bit(config.reset_enabled)
                .wdfien()
                .variant(config.interrupt_enabled)
                .wdv()
                .bits(raw_duration)
        });

        self.configured = true;
        Ok(())
    }

    /// Check if watchdog was configured.
    pub fn was_configured(&self) -> bool {
        self.configured
    }

    /// Refresh the watchdog counter.
    pub fn feed(&mut self) {
        self.wdt.cr.write(|w| w.key().passwd().wdrstt().set_bit());
    }

    /// Disable the watchdog.
    ///
    /// Note that watchdog can be configured or disabled only once.
    /// Once disabled, it's off until the MCU performs a hard reset.
    ///
    /// # Return
    /// [`WatchdogError::WatchdogAlreadyConfigured`] if watchdog instance was
    /// configured earlier, `Ok(())` otherwise.
    pub fn disable(&mut self) -> Result<(), WatchdogError> {
        if self.configured {
            return Err(WatchdogError::WatchdogAlreadyConfigured);
        }

        self.wdt.mr.modify(|_, w| w.wddis().set_bit());
        self.configured = true;

        Ok(())
    }

    /// Convert duration to watchdog counter value.
    ///
    /// # Parameters
    /// * `duration` - Watchdog duration.
    ///
    /// # Returns
    /// Watchdog counter value representing passed duration.
    ///
    /// # Notes
    /// `duration` must be in inclusive range [0, [`MAXIMUM_WATCHDOG_DURATION`]].
    /// Since it's internal, private function, it does not perform any checks.
    /// To safely convert any duration into watchdog counter value, use [`clamp_and_convert_duration`](Watchdog::clamp_and_convert_duration).
    fn convert_duration_to_counter_value(duration: MillisDurationU32) -> u16 {
        let duration_ratio: f32 =
            (duration.to_secs() as f32) / (MAXIMUM_WATCHDOG_DURATION.to_secs() as f32);

        (duration_ratio * (0xFFF as f32)) as u16
    }

    /// Clamp duration to inclusive [0, [`MAXIMUM_WATCHDOG_DURATION`]] range,
    /// and convert it to unsigned value that can be put in watchdog's register
    ///
    /// # Parameters
    /// * `duration` - Watchdog duration.
    ///
    /// # Returns
    /// Watchdog counter value representing passed duration.
    fn clamp_and_convert_duration(duration: MillisDurationU32) -> u16 {
        let clamped_duration =
            duration.clamp(MillisDurationU32::secs(0), MillisDurationU32::secs(16));

        Watchdog::convert_duration_to_counter_value(clamped_duration)
    }
}
