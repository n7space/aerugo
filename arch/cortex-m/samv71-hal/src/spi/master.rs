//! Implementation of SPI driver functionality in Master mode.

use super::{interrupts::Interrupts, metadata::SPIMetadata, Master, Spi};

impl<Instance: SPIMetadata> Spi<Instance, Master> {
    /// Sets interrupts state.
    /// Interrupts set to `true` will be enabled, and set to `false` - disabled.
    pub fn set_interrupts_state(&mut self, interrupts: Interrupts) {
        Instance::registers().ier.write(|w| {
            w.rdrf()
                .variant(interrupts.rx_data_register_full)
                .tdre()
                .variant(interrupts.tx_data_register_empty)
                .modf()
                .variant(interrupts.mode_fault_error)
                .ovres()
                .variant(interrupts.overrun_error)
                .nssr()
                .variant(interrupts.nss_rising)
                .txempty()
                .variant(interrupts.tx_registers_empty)
                .undes()
                .variant(interrupts.underrun_error)
        });

        Instance::registers().idr.write(|w| {
            w.rdrf()
                .variant(!interrupts.rx_data_register_full)
                .tdre()
                .variant(!interrupts.tx_data_register_empty)
                .modf()
                .variant(!interrupts.mode_fault_error)
                .ovres()
                .variant(!interrupts.overrun_error)
                .nssr()
                .variant(!interrupts.nss_rising)
                .txempty()
                .variant(!interrupts.tx_registers_empty)
                .undes()
                .variant(!interrupts.underrun_error)
        });
    }

    /// Returns current interrupts state (enabled/disabled).
    ///
    /// To check the state of events the IRQs indicate, use
    /// [`StatusReader`](super::status_reader::StatusReader).
    pub fn interrupts_state(&self) -> Interrupts {
        Instance::registers().imr.read().into()
    }
}
