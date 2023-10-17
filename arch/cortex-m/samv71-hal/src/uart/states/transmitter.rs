//! Module with implementation of UART in transmitter mode.
//!
use crate::uart::{metadata::UARTMetadata, writer::Writer, Transmit, UART};

impl<Instance: UARTMetadata, State: Transmit> UART<Instance, State> {
    /// Takes [`Writer`] instance out of UART.
    /// There can only be a single instance of Writer per UART, and this is the only way to get it.
    /// You can check whether UART stores Writer instance using [`UART::has_writer`].
    /// Writer can be put back into UART using [`UART::put_writer`].
    ///
    /// # Returns
    /// `Some(Writer)` if instance of a writer is currently stored in UART.
    /// `None` if it was already taken.
    pub fn take_writer(&mut self) -> Option<Writer<Instance>> {
        self.writer.take()
    }

    /// Resets UART transmitter.
    ///
    /// Any pending byte transmission is aborted when the transmitter is reset.
    pub fn reset_transmitter(&mut self) {
        Instance::registers().cr.write(|w| w.rsttx().set_bit());
    }
}
