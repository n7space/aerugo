//! This module contains structures related to Programmable Clocks (PCK) configuration.

use crate::pac::pmc::pck::CSSSELECT_A;

/// Constant indicating the amount of PCKs supported by SAMV71 MCU.
pub const PROGRAMMABLE_CLOCKS_SUPPORTED: usize = 8;

/// Structure representing Programmable Clock (PCK) configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PCKConfig {
    /// Programmable clock's source
    pub source: PCKSource,
    /// Programmable clock's prescaler. Source clock is divided by this value.
    /// Valid range for prescaler is [2; 256], inclusive.
    pub prescaler: PCKPrescaler,
}

/// Type alias for list of programmable clock statuses
pub type PCKList = [bool; PROGRAMMABLE_CLOCKS_SUPPORTED];

/// Enumeration listing available programmable clocks.
///
/// This enumeration is provided for type safety, and can be converted
/// to integer via `.into()`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PCK {
    /// Programmable clock 0
    PCK0 = 0,
    /// Programmable clock 1
    PCK1 = 1,
    /// Programmable clock 2
    PCK2 = 2,
    /// Programmable clock 3
    PCK3 = 3,
    /// Programmable clock 4
    PCK4 = 4,
    /// Programmable clock 5
    PCK5 = 5,
    /// Programmable clock 6
    PCK6 = 6,
    /// Programmable clock 7
    PCK7 = 7,
}

impl TryFrom<usize> for PCK {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PCK::PCK0),
            1 => Ok(PCK::PCK1),
            2 => Ok(PCK::PCK2),
            3 => Ok(PCK::PCK3),
            4 => Ok(PCK::PCK4),
            5 => Ok(PCK::PCK5),
            6 => Ok(PCK::PCK6),
            7 => Ok(PCK::PCK7),
            _ => Err(()),
        }
    }
}

impl From<PCK> for usize {
    fn from(value: PCK) -> Self {
        match value {
            PCK::PCK0 => 0,
            PCK::PCK1 => 1,
            PCK::PCK2 => 2,
            PCK::PCK3 => 3,
            PCK::PCK4 => 4,
            PCK::PCK5 => 5,
            PCK::PCK6 => 6,
            PCK::PCK7 => 7,
        }
    }
}

/// Enumeration listing available Programmable Clock sources.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PCKSource {
    /// Slow clock (SLCK).
    SlowClock,
    /// Main clock (MAINCK).
    MainClock,
    /// PLL A clock.
    PLLA,
    /// USB UTMI PLL clock.
    USBPLL,
    /// Master clock (MCK).
    MasterClock,
}

impl From<CSSSELECT_A> for PCKSource {
    fn from(value: CSSSELECT_A) -> Self {
        match value {
            CSSSELECT_A::SLOW_CLK => PCKSource::SlowClock,
            CSSSELECT_A::MAIN_CLK => PCKSource::MainClock,
            CSSSELECT_A::PLLA_CLK => PCKSource::PLLA,
            CSSSELECT_A::UPLL_CLK => PCKSource::USBPLL,
            CSSSELECT_A::MCK => PCKSource::MasterClock,
        }
    }
}

impl From<PCKSource> for CSSSELECT_A {
    fn from(value: PCKSource) -> Self {
        match value {
            PCKSource::SlowClock => CSSSELECT_A::SLOW_CLK,
            PCKSource::MainClock => CSSSELECT_A::MAIN_CLK,
            PCKSource::PLLA => CSSSELECT_A::PLLA_CLK,
            PCKSource::USBPLL => CSSSELECT_A::UPLL_CLK,
            PCKSource::MasterClock => CSSSELECT_A::MCK,
        }
    }
}

/// Structure representing PCK prescaler.
/// Valid range of PCK prescaler is (2..=256).
///
/// This is a convenience structure that makes it impossible to create invalid prescaler values,
/// as prescaler value in the register is shifted by 1, and it's not reasonable to set the prescaler
/// to 0, as it stops the timer (it should be disabled instead, or dedicated function should be provided
/// for that, if needed).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PCKPrescaler {
    /// "Hardware" value of the prescaler.
    /// This can be written directly into the register.
    value: u8,
}

/// Enumeration representing prescaler errors.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PrescalerError {
    /// Tried to create prescaler which value is out of range.
    /// Value is provided along error code.
    OutOfRange(u16),
}

impl PCKPrescaler {
    /// Create new instance of PCKPrescaler.
    ///
    /// # Parameters
    /// * `prescaler` - Value of the prescaler. Valid range is (2..=256).
    ///
    /// # Returns
    /// `Ok(PCKPrescaler)` if value is correct, `Err(())` otherwise.
    pub fn new(prescaler: u16) -> Result<Self, PrescalerError> {
        if !(2..=256).contains(&prescaler) {
            Err(PrescalerError::OutOfRange(prescaler))
        } else {
            Ok(PCKPrescaler {
                // We're converting (2..=256) range into (1..=255), which
                // always fits in 8-bit unsigned, so it's safe to cast.
                value: (prescaler - 1) as u8,
            })
        }
    }

    /// Returns user-provided value of the prescaler.
    pub fn value(&self) -> u16 {
        (self.value as u16) + 1
    }

    /// Returns "hardware" value of the prescaler, that can be written directly into the register.
    ///
    /// # Safety
    /// Usage of this function is safe as long as the value invariant (it must not be 0)
    /// is enforced.
    pub(crate) fn into_register_value(self) -> u8 {
        self.value
    }

    /// Converts value read from the register into `PCKPrescaler`.
    pub(crate) fn from_register_value(value: u8) -> PCKPrescaler {
        PCKPrescaler { value }
    }
}

impl TryFrom<u16> for PCKPrescaler {
    type Error = PrescalerError;

    fn try_from(prescaler: u16) -> Result<Self, Self::Error> {
        PCKPrescaler::new(prescaler)
    }
}

impl From<PCKPrescaler> for u16 {
    fn from(prescaler: PCKPrescaler) -> Self {
        prescaler.value()
    }
}
