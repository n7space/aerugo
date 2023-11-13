//! Module with SPI configuration related items.

use samv71q21_pac::spi0::mr::PCSSELECT_A;

use crate::utils::BoundedU8;

/// Structure with Master mode configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MasterConfig {
    /// Delay between the inactivation and activation of chip select signal.
    pub chip_selection_delay: ChipSelectionDelay,
    /// Peripheral chip select. This can later be changed using [`Spi::change_chip`].
    pub selected_chip: SelectedChip,
    /// If true, overrun detection will be enabled and an error will occur if data transmission will
    /// try to be started while there's unread data in RX data register.
    pub enable_overrun_detection: bool,
}

/// Type representing Chip Select Delay. It's measured in peripheral clock periods and it cannot
/// be lower than 6.
pub type ChipSelectionDelay = BoundedU8<6, { u8::MAX }>;

/// Chip select signal selection.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SelectedChip {
    /// Peripheral connected to CS0
    Chip0,
    /// Peripheral connected to CS1
    Chip1,
    /// Peripheral connected to CS2
    Chip2,
    /// Peripheral connected to CS3
    Chip3,
}

impl MasterConfig {
    /// Creates new MasterConfig with provided chip following defaults:
    ///
    /// * Minimum chip selection delay (6 peripheral clock cycles)
    /// * Overrun detection disabled
    pub fn new(selected_chip: SelectedChip) -> Self {
        Self {
            chip_selection_delay: ChipSelectionDelay::new(ChipSelectionDelay::LOW).unwrap(),
            selected_chip,
            enable_overrun_detection: false,
        }
    }
}

impl From<SelectedChip> for PCSSELECT_A {
    fn from(value: SelectedChip) -> Self {
        match value {
            SelectedChip::Chip0 => PCSSELECT_A::NPCS0,
            SelectedChip::Chip1 => PCSSELECT_A::NPCS1,
            SelectedChip::Chip2 => PCSSELECT_A::NPCS2,
            SelectedChip::Chip3 => PCSSELECT_A::NPCS3,
        }
    }
}

impl From<PCSSELECT_A> for SelectedChip {
    fn from(value: PCSSELECT_A) -> Self {
        match value {
            PCSSELECT_A::NPCS0 => SelectedChip::Chip0,
            PCSSELECT_A::NPCS1 => SelectedChip::Chip1,
            PCSSELECT_A::NPCS2 => SelectedChip::Chip2,
            PCSSELECT_A::NPCS3 => SelectedChip::Chip3,
        }
    }
}
