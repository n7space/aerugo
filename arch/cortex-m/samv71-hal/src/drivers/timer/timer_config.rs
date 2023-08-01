//! Module containing configuration structures for Timer

use super::timer_error::TimerConfigurationError;
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
    /// To prevent accidental typos, returned values are taken directly from PAC.
    /// This allows for easy type erasure, while also retaining value safety.
    ///
    /// # Parameters
    /// * `clock` - External clock source to apply for current clock.
    ///
    /// # Returns
    /// `Ok(u8)` if conversion was successful, `Err(TimerConfigurationError::InvalidClockSource)` if
    /// selected clock source cannot be connected to the external clock.
    pub(super) fn source_id(
        self,
        clock: ExternalClockSource,
    ) -> Result<u8, TimerConfigurationError> {
        match self {
            ExternalClock::XC0 => match clock {
                ExternalClockSource::TCLKx => Ok(TC0XC0SSELECT_A::TCLK0 as u8),
                ExternalClockSource::TIOA0 => Err(TimerConfigurationError::InvalidClockSource),
                ExternalClockSource::TIOA1 => Ok(TC0XC0SSELECT_A::TIOA1 as u8),
                ExternalClockSource::TIOA2 => Ok(TC0XC0SSELECT_A::TIOA2 as u8),
            },
            ExternalClock::XC1 => match clock {
                ExternalClockSource::TCLKx => Ok(TC1XC1SSELECT_A::TCLK1 as u8),
                ExternalClockSource::TIOA0 => Ok(TC1XC1SSELECT_A::TIOA0 as u8),
                ExternalClockSource::TIOA1 => Err(TimerConfigurationError::InvalidClockSource),
                ExternalClockSource::TIOA2 => Ok(TC1XC1SSELECT_A::TIOA2 as u8),
            },
            ExternalClock::XC2 => match clock {
                ExternalClockSource::TCLKx => Ok(TC2XC2SSELECT_A::TCLK2 as u8),
                ExternalClockSource::TIOA0 => Ok(TC2XC2SSELECT_A::TIOA0 as u8),
                ExternalClockSource::TIOA1 => Ok(TC2XC2SSELECT_A::TIOA1 as u8),
                ExternalClockSource::TIOA2 => Err(TimerConfigurationError::InvalidClockSource),
            },
        }
    }
}
