//! This module contains status-related PMC/ClockController structures.

/// Structure representing PMC status.
///
/// Since it contains a lot of booleans that won't be packed, it's not automatically
/// deriving `Copy`. Clone it manually instead.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Status {
    /// Main crystal oscillator status.
    pub mco_stabilized: bool,
    /// PLL A status.
    pub plla_locked: bool,
    /// Master clock status.
    pub master_clock_ready: bool,
    /// UTMI PLL lock status.
    pub utmi_pll_locked: bool,
    /// Slow clock source.
    pub slow_clock_source: SlowClockSource,
    /// Programmable clock (PCK) readiness.
    pub pck_ready: [bool; 8],
    /// Main clock source oscillator selection status.
    pub main_clock_selected: bool,
    /// Main RC oscillator status.
    pub main_rc_stabilized: bool,
    /// Main crystal oscillator clock status.
    pub mco_status: MCOStatus,
    /// Slow crystal oscillator error.
    /// If slow oscillator monitoring is disabled, will stay `false`.
    /// If slow oscillator monitoring is enabled, will be set on clock failure.
    pub slow_oscillator_error: bool,
}

/// Slow clock (SLCK) source
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SlowClockSource {
    /// Internal RC oscillator.
    InternalRCOscillator = 0,
    /// External 32.768kHz crystal oscillator.
    External32kCrystal = 1,
}

impl From<bool> for SlowClockSource {
    fn from(value: bool) -> Self {
        match value {
            true => SlowClockSource::External32kCrystal,
            false => SlowClockSource::InternalRCOscillator,
        }
    }
}

impl From<SlowClockSource> for bool {
    fn from(value: SlowClockSource) -> Self {
        match value {
            SlowClockSource::InternalRCOscillator => false,
            SlowClockSource::External32kCrystal => true,
        }
    }
}

/// Main crystal oscillator clock status.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MCOStatus {
    /// Indicates if at least one clock failure event has been detected.
    /// This is cleared when status register is read.
    pub clock_failure_event_detected: bool,
    /// Indicates if MCO has failed in last slow clock cycle.
    /// Is automatically cleared on next slow clock cycle that does not
    /// detect MCO failure.
    pub clock_failure_currently_detected: bool,
    /// Indicates if fault output of clock failure detector is active.
    pub clock_failure_fault_active: bool,
}
