//! Module with chip select-related configuration items.

use samv71q21_pac::spi0::csr::{BITSSELECT_A, CPOLSELECT_A, NCPHASELECT_A};

use crate::utils::BoundedU8;

/// Chip configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ChipConfig {
    /// Clock polarity configuration.
    pub clock_polarity: ClockPolarity,
    /// Clock phase configuration.
    pub clock_phase: ClockPhase,
    /// Chip select behavior.
    pub chip_select_behavior: ChipSelectBehavior,
    /// Amount of bits per transfer.
    pub bits_per_transfer: BitsPerTransfer,
    /// Serial clock divider.
    pub clock_divider: SerialClockDivider,
    /// Delay between chip select activation and clock signal in peripheral clock cycles.
    /// If set to 0, the half of clock signal period will be used.
    pub delay_before_first_clock: u8,
    /// Delay between consecutive transfers with the same peripheral in peripheral clock cycles / 32.
    /// This delay is always inserted after each transfer, and before removing chip select after
    /// last transfer, depending on chosen chip select behavior
    pub delay_between_consecutive_transfers: u8,
}

/// Enumeration representing clock polarity configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockPolarity {
    /// Clock line is low when inactive.
    LowWhenInactive,
    /// Clock line is high when inactive.
    HighWhenInactive,
}

/// Enumeration representing clock phase configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockPhase {
    /// Data is changed on the leading edge of SPCK and captured on following one.
    DataChangedOnLeadingEdge,
    /// Data is captured on the leading edge of SPCK and changed on following one.
    /// That implies the client must prepare the data the moment the NSS signal is asserted.
    DataCapturedOnLeadingEdge,
}

/// Enumeration representing chip select behavior after transmission.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ChipSelectBehavior {
    /// Chip select line is deactivated as soon as the last transfer is finished.
    #[default]
    DeactivateAfterLastTransfer,
    /// Chip select line is kept active until a new transfer is requested on a different chip select.
    KeepActive,
    /// Chip select line is deactivated after every data transfer. Duration of the inactivity period
    /// can be controlled with `delay_between_transfers` field of [`ChipSelectConfig`].
    DeactivateAfterEveryTransfer,
}

/// Enumeration representing bits per transfer.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BitsPerTransfer {
    /// 8 bits per transfer.
    Bits8,
    /// 9 bits per transfer.
    Bits9,
    /// 10 bits per transfer.
    Bits10,
    /// 11 bits per transfer.
    Bits11,
    /// 12 bits per transfer.
    Bits12,
    /// 13 bits per transfer.
    Bits13,
    /// 14 bits per transfer.
    Bits14,
    /// 15 bits per transfer.
    Bits15,
    /// 16 bits per transfer.
    Bits16,
}

/// Type alias representing serial clock bit rate.
/// To calculate this value for specific SPI clock frequency, divide the SPI peripheral clock by
/// desired SPI clock frequency.
pub type SerialClockDivider = BoundedU8<1, 255>;

impl From<ClockPolarity> for CPOLSELECT_A {
    fn from(value: ClockPolarity) -> Self {
        match value {
            ClockPolarity::LowWhenInactive => CPOLSELECT_A::IDLE_LOW,
            ClockPolarity::HighWhenInactive => CPOLSELECT_A::IDLE_HIGH,
        }
    }
}

impl From<CPOLSELECT_A> for ClockPolarity {
    fn from(value: CPOLSELECT_A) -> Self {
        match value {
            CPOLSELECT_A::IDLE_LOW => ClockPolarity::LowWhenInactive,
            CPOLSELECT_A::IDLE_HIGH => ClockPolarity::HighWhenInactive,
        }
    }
}

impl From<ClockPhase> for NCPHASELECT_A {
    fn from(value: ClockPhase) -> Self {
        match value {
            ClockPhase::DataChangedOnLeadingEdge => NCPHASELECT_A::VALID_TRAILING_EDGE,
            ClockPhase::DataCapturedOnLeadingEdge => NCPHASELECT_A::VALID_LEADING_EDGE,
        }
    }
}

impl From<NCPHASELECT_A> for ClockPhase {
    fn from(value: NCPHASELECT_A) -> Self {
        match value {
            NCPHASELECT_A::VALID_LEADING_EDGE => ClockPhase::DataCapturedOnLeadingEdge,
            NCPHASELECT_A::VALID_TRAILING_EDGE => ClockPhase::DataChangedOnLeadingEdge,
        }
    }
}

impl From<BitsPerTransfer> for BITSSELECT_A {
    fn from(value: BitsPerTransfer) -> Self {
        match value {
            BitsPerTransfer::Bits8 => BITSSELECT_A::_8_BIT,
            BitsPerTransfer::Bits9 => BITSSELECT_A::_9_BIT,
            BitsPerTransfer::Bits10 => BITSSELECT_A::_10_BIT,
            BitsPerTransfer::Bits11 => BITSSELECT_A::_11_BIT,
            BitsPerTransfer::Bits12 => BITSSELECT_A::_12_BIT,
            BitsPerTransfer::Bits13 => BITSSELECT_A::_13_BIT,
            BitsPerTransfer::Bits14 => BITSSELECT_A::_14_BIT,
            BitsPerTransfer::Bits15 => BITSSELECT_A::_15_BIT,
            BitsPerTransfer::Bits16 => BITSSELECT_A::_16_BIT,
        }
    }
}

impl From<BITSSELECT_A> for BitsPerTransfer {
    fn from(value: BITSSELECT_A) -> Self {
        match value {
            BITSSELECT_A::_8_BIT => BitsPerTransfer::Bits8,
            BITSSELECT_A::_9_BIT => BitsPerTransfer::Bits9,
            BITSSELECT_A::_10_BIT => BitsPerTransfer::Bits10,
            BITSSELECT_A::_11_BIT => BitsPerTransfer::Bits11,
            BITSSELECT_A::_12_BIT => BitsPerTransfer::Bits12,
            BITSSELECT_A::_13_BIT => BitsPerTransfer::Bits13,
            BITSSELECT_A::_14_BIT => BitsPerTransfer::Bits14,
            BITSSELECT_A::_15_BIT => BitsPerTransfer::Bits15,
            BITSSELECT_A::_16_BIT => BitsPerTransfer::Bits16,
        }
    }
}
