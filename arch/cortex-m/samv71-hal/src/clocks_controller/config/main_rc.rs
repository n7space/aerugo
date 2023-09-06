//! This module contains configuration structures related to Main RC oscillator.

use samv71q21_pac::pmc::ckgr_mor::MOSCRCFSELECT_A;

use crate::time::{self, RateExtU32};

/// Main RC oscillator frequency.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MainRcFrequency {
    /// 4MHz.
    MainRc4MHz = 0,
    /// 8MHz.
    MainRc8MHz = 1,
    /// 12MHz.
    MainRc12MHz = 2,
}

impl TryFrom<time::MegahertzU32> for MainRcFrequency {
    type Error = ();

    /// Converts time rate in megahertz to Main RC oscillator frequency.
    ///
    /// Can only be used to convert 4, 8 and 12MHz values to corresponding
    /// enum values. Any other time rate will return empty result.
    fn try_from(value: time::MegahertzU32) -> Result<Self, Self::Error> {
        match value.to_MHz() {
            4 => Ok(MainRcFrequency::MainRc4MHz),
            8 => Ok(MainRcFrequency::MainRc8MHz),
            12 => Ok(MainRcFrequency::MainRc12MHz),
            _ => Err(()),
        }
    }
}

impl From<MainRcFrequency> for time::MegahertzU32 {
    fn from(value: MainRcFrequency) -> Self {
        match value {
            MainRcFrequency::MainRc4MHz => 4.MHz(),
            MainRcFrequency::MainRc8MHz => 8.MHz(),
            MainRcFrequency::MainRc12MHz => 12.MHz(),
        }
    }
}

impl From<MainRcFrequency> for MOSCRCFSELECT_A {
    fn from(value: MainRcFrequency) -> Self {
        match value {
            MainRcFrequency::MainRc4MHz => MOSCRCFSELECT_A::_4_MHZ,
            MainRcFrequency::MainRc8MHz => MOSCRCFSELECT_A::_8_MHZ,
            MainRcFrequency::MainRc12MHz => MOSCRCFSELECT_A::_12_MHZ,
        }
    }
}

impl From<MOSCRCFSELECT_A> for MainRcFrequency {
    fn from(value: MOSCRCFSELECT_A) -> Self {
        match value {
            MOSCRCFSELECT_A::_4_MHZ => MainRcFrequency::MainRc4MHz,
            MOSCRCFSELECT_A::_8_MHZ => MainRcFrequency::MainRc8MHz,
            MOSCRCFSELECT_A::_12_MHZ => MainRcFrequency::MainRc12MHz,
        }
    }
}
