//! SPI Status structure and helpers.

use crate::pac::spi0::sr::R as StatusReader;

/// SPI Status structure.
pub struct SpiStatus {
    /// When true, then the data has been received and can be read from RX data register.
    pub rx_data_register_full: bool,
    /// When true, data has been moved from TX data register to internal shift register and new data
    /// can be copied to TX data register.
    pub tx_data_register_empty: bool,
    /// If true, a mode fault error has happened since last status read.
    pub mode_fault_error: bool,
    /// If true, an overrun error has happened since last status read.
    pub overrun_error: bool,
    /// If true, a rising edge occurred on NSS pin since last status read
    pub nss_rising: bool,
    /// When true, both TX data register and internal TX shift register are empty. If transfer delay
    /// has been defined, this flag is set after the end of this delay.
    pub tx_registers_empty: bool,
    /// If true, a transfer started while no data was loaded in TX data register since last status
    /// read.
    pub underrun_error: bool,
    /// When true, SPI is enabled.
    pub is_enabled: bool,
}

impl From<StatusReader> for SpiStatus {
    fn from(reg: StatusReader) -> Self {
        SpiStatus {
            rx_data_register_full: reg.rdrf().bit_is_set(),
            tx_data_register_empty: reg.tdre().bit_is_set(),
            mode_fault_error: reg.modf().bit_is_set(),
            overrun_error: reg.ovres().bit_is_set(),
            nss_rising: reg.nssr().bit_is_set(),
            tx_registers_empty: reg.txempty().bit_is_set(),
            underrun_error: reg.undes().bit_is_set(),
            is_enabled: reg.spiens().bit(),
        }
    }
}
