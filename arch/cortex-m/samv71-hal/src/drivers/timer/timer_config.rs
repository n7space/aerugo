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

impl ExternalClockSource {
    /// Converts external clock source to numeric ID representing it's value
    /// in Timer's Block Mode configuration register.
    ///
    /// # Parameters
    /// * `channel` - Channel
    pub(super) fn to_clock_source_id(
        self,
        clock: ExternalClock,
    ) -> Result<u8, TimerConfigurationError> {
        match clock {
            ExternalClock::XC0 => match self {
                ExternalClockSource::TCLKx => Ok(TC0XC0SSELECT_A::TCLK0 as u8),
                ExternalClockSource::TIOA0 => {
                    Err(TimerConfigurationError::InvalidClockSourceForExternalClock)
                }
                ExternalClockSource::TIOA1 => Ok(TC0XC0SSELECT_A::TIOA1 as u8),
                ExternalClockSource::TIOA2 => Ok(TC0XC0SSELECT_A::TIOA2 as u8),
            },
            ExternalClock::XC1 => match self {
                ExternalClockSource::TCLKx => Ok(TC1XC1SSELECT_A::TCLK1 as u8),
                ExternalClockSource::TIOA0 => Ok(TC1XC1SSELECT_A::TIOA0 as u8),
                ExternalClockSource::TIOA1 => {
                    Err(TimerConfigurationError::InvalidClockSourceForExternalClock)
                }
                ExternalClockSource::TIOA2 => Ok(TC1XC1SSELECT_A::TIOA2 as u8),
            },
            ExternalClock::XC2 => match self {
                ExternalClockSource::TCLKx => Ok(TC2XC2SSELECT_A::TCLK2 as u8),
                ExternalClockSource::TIOA0 => Ok(TC2XC2SSELECT_A::TIOA0 as u8),
                ExternalClockSource::TIOA1 => Ok(TC2XC2SSELECT_A::TIOA1 as u8),
                ExternalClockSource::TIOA2 => {
                    Err(TimerConfigurationError::InvalidClockSourceForExternalClock)
                }
            },
        }
    }
}

/// External clock signal selection configuration structure.
/// This can be used to configure external clock inputs for all channels at once,
/// to reduce amount of writes performed to timer registers.
pub type ExternalClockSignalSelectionConfig = [ExternalClockSource; 3];
