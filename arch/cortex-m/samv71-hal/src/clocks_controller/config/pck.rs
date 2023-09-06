//! This module contains structures related to Programmable Clocks (PCK) configuration.

/// Constant indicating the amount of PCKs supported by SAMV71 MCU.
pub const PROGRAMMABLE_CLOCKS_SUPPORTED: usize = 8;

/// Structure representing Programmable Clock (PCK) configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PCKConfig {}

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
