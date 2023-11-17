//! SPI Status structure and helpers.

use crate::pac::spi0::sr::R as StatusReader;

use super::interrupts::Interrupts;

/// SPI Status structure.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpiStatus {
    /// Interrupt-related status
    pub interrupts: Interrupts,
    /// When true, SPI is enabled.
    pub is_enabled: bool,
}

impl From<StatusReader> for SpiStatus {
    fn from(reg: StatusReader) -> Self {
        let is_enabled = reg.spiens().bit();
        SpiStatus {
            interrupts: reg.into(),
            is_enabled,
        }
    }
}
