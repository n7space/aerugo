//! Module with implementation of UART in bidirectional mode.
//!
use crate::uart::{metadata::UARTMetadata, Bidirectional, UART};

impl<Instance: UARTMetadata> UART<Instance, Bidirectional> {
    /// Switches UART into local loopback mode.
    ///
    /// In this mode, transmitter is internally connected to receiver.
    /// RX and TX lines are disconnected from receiver and transmitter.
    /// TX line is pulled to Vdd.
    ///
    /// Effectively, every sent byte should be automatically received by
    /// the same UART.
    pub fn switch_to_local_loopback_mode(&mut self) {
        Instance::registers()
            .mr
            .modify(|_, w| w.chmode().local_loopback());
    }
}
