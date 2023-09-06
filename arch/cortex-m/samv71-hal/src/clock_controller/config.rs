//! This module contains configuration-related PMC/ClockController structures.

use crate::time;
use crate::time::RateExtU32;

/// Main RC oscillator frequency.
pub enum MainRcOscillatorFrequency {
    /// 4MHz.
    MainRc4MHz,
    /// 8MHz.
    MainRc8MHz,
    /// 12MHz.
    MainRc12MHz,
}

impl TryFrom<time::MegahertzU32> for MainRcOscillatorFrequency {
    type Error = ();

    /// Converts time rate in megahertz to Main RC oscillator frequency.
    ///
    /// Can only be used to convert 4, 8 and 12MHz values to corresponding
    /// enum values. Any other time rate will return empty result.
    fn try_from(value: time::MegahertzU32) -> Result<Self, Self::Error> {
        match value.to_MHz() {
            4 => Ok(MainRcOscillatorFrequency::MainRc4MHz),
            8 => Ok(MainRcOscillatorFrequency::MainRc8MHz),
            12 => Ok(MainRcOscillatorFrequency::MainRc12MHz),
            _ => Err(()),
        }
    }
}

impl From<MainRcOscillatorFrequency> for time::MegahertzU32 {
    fn from(value: MainRcOscillatorFrequency) -> Self {
        match value {
            MainRcOscillatorFrequency::MainRc4MHz => 4.MHz(),
            MainRcOscillatorFrequency::MainRc8MHz => 8.MHz(),
            MainRcOscillatorFrequency::MainRc12MHz => 12.MHz(),
        }
    }
}
