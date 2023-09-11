//! This module contains interrupt-related PMC/ClockController structures.

/// Structure representing PMC interrupts.
///
/// Since it contains a lot of booleans that won't be packed, it's not automatically
/// deriving `Copy`. Clone it manually instead.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
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

impl Interrupts {
    /// Creates [`Interrupts`] instance with all interrupts flags set.
    pub const fn all() -> Self {
        Interrupts {
            mco_stabilized: true,
            plla_locked: true,
            master_clock_ready: true,
            utmi_pll_locked: true,
            pck_ready: [true; 8],
            main_clock_selected: true,
            main_rc_stabilized: true,
            mco_failure_event_detected: true,
            slow_oscillator_error: true,
        }
    }

    /// Creates [`Interrupts`] instance with all interrupts flags cleared.
    /// Same as [`Default::default`], but with `const` on top.
    pub const fn none() -> Self {
        Interrupts {
            mco_stabilized: false,
            plla_locked: false,
            master_clock_ready: false,
            utmi_pll_locked: false,
            pck_ready: [false; 8],
            main_clock_selected: false,
            main_rc_stabilized: false,
            mco_failure_event_detected: false,
            slow_oscillator_error: false,
        }
    }
}
