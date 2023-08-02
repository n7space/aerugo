//! Module containing configuration structures for Timer

use pac::tc0::bmr::{TC0XC0SSELECT_A, TC1XC1SSELECT_A, TC2XC2SSELECT_A};

/// External clock signal source.
///
/// This represents available source clock signals for channel's external clock inputs.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ExternalClockSource {
    /// Timer clock (default source)
    TCLKx,
    /// Channel 0 output A
    TIOA0,
    /// Channel 1 output A
    TIOA1,
    /// Channel 2 output A
    TIOA2,
}

/// External clock signal.
///
/// This represents available external clock signals that can be used as timer channel's input clocks.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ExternalClock {
    /// External Clock 0.
    XC0,
    /// External Clock 1.
    XC1,
    /// External Clock 2.
    XC2,
}

impl ExternalClock {
    /// Converts external clock source to numeric ID representing it's value
    /// in Timer's Block Mode configuration register.
    ///
    /// Normally, [`From`] trait implementation would be used, but this enum values
    /// are used in multiple registers fields, while also having different types
    /// in PAC, so it's much less boilerplate to erase their type into u8.
    ///
    /// To prevent accidental typos, values are taken directly from PAC, and
    /// converted to u8. This allows easy type erasure, while also retaining
    /// value safety.
    ///
    /// # Parameters
    /// * `clock` - External clock source to apply for current clock.
    ///
    /// # Returns
    /// `Some(u8)` if conversion was successful, `None` if selected clock source
    /// cannot be connected to the external clock.
    pub(super) fn id(self, clock: ExternalClockSource) -> Option<u8> {
        match self {
            ExternalClock::XC0 => match clock {
                ExternalClockSource::TCLKx => Some(TC0XC0SSELECT_A::TCLK0 as u8),
                ExternalClockSource::TIOA0 => None,
                ExternalClockSource::TIOA1 => Some(TC0XC0SSELECT_A::TIOA1 as u8),
                ExternalClockSource::TIOA2 => Some(TC0XC0SSELECT_A::TIOA2 as u8),
            },
            ExternalClock::XC1 => match clock {
                ExternalClockSource::TCLKx => Some(TC1XC1SSELECT_A::TCLK1 as u8),
                ExternalClockSource::TIOA0 => Some(TC1XC1SSELECT_A::TIOA0 as u8),
                ExternalClockSource::TIOA1 => None,
                ExternalClockSource::TIOA2 => Some(TC1XC1SSELECT_A::TIOA2 as u8),
            },
            ExternalClock::XC2 => match clock {
                ExternalClockSource::TCLKx => Some(TC2XC2SSELECT_A::TCLK2 as u8),
                ExternalClockSource::TIOA0 => Some(TC2XC2SSELECT_A::TIOA0 as u8),
                ExternalClockSource::TIOA1 => Some(TC2XC2SSELECT_A::TIOA1 as u8),
                ExternalClockSource::TIOA2 => None,
            },
        }
    }
}
