//! This module contains interrupt-related PMC/ClockController structures.

/// Structure representing PMC interrupts.
///
/// Since it contains a lot of booleans that won't be packed, it's not automatically
/// deriving `Copy`. Clone it manually instead.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Interrupts {
    /// Main crystal oscillator status interrupt.
    pub mco_stabilized: bool,
    /// PLL A status interrupt.
    pub plla_locked: bool,
    /// Master clock status interrupt.
    pub master_clock_ready: bool,
    /// UTMI PLL lock status interrupt.
    pub utmi_pll_locked: bool,
    /// Programmable clock (PCK) readiness interrupt.
    pub pck_ready: [bool; 8],
    /// Main clock source oscillator selection status interrupt.
    pub main_clock_selected: bool,
    /// Main RC oscillator status interrupt.
    pub main_rc_stabilized: bool,
    /// Main crystal oscillator clock failure event detection interrupt.
    pub mco_failure_event_detected: bool,
    /// Slow crystal oscillator error interrupt.
    pub slow_oscillator_error: bool,
}
