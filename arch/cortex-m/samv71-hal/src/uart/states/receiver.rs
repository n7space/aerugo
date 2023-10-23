//! Module with implementation of UART in receiver mode.

use crate::uart::{
    config::rx_filter_config_to_bool, metadata::UARTMetadata, reader::Reader, Receive, UART,
};

impl<Instance: UARTMetadata, State: Receive> UART<Instance, State> {
    /// Takes [`Reader`] instance out of UART.
    /// There can only be a single instance of Reader per UART, and this is the only way to get it.
    /// You can check whether UART stores Reader instance using [`UART::has_reader`].
    /// Reader can be put back into UART using [`UART::put_reader`].
    ///
    /// # Returns
    /// `Some(Reader)` if instance of a reader is currently stored in UART.
    /// `None` if it was already taken.
    pub fn take_reader(&mut self) -> Option<Reader<Instance>> {
        self.reader.take()
    }

    /// Resets UART receiver.
    ///
    /// Any pending byte reception is aborted when the receiver is reset.
    pub fn reset_receiver(&mut self) {
        Instance::registers().cr.write(|w| w.rstrx().set_bit());
    }

    /// Returns true if receive line is filtered.
    ///
    /// Filtering is done using a three-sample filter (16x-bit clock, 2 over 3 majority).
    pub fn is_rx_filter_enabled(&self) -> bool {
        rx_filter_config_to_bool(Instance::registers().mr.read().filter().variant())
    }

    /// Sets receive line filtering state.
    ///
    /// Filtering is done using a three-sample filter (16x-bit clock, 2 over 3 majority).
    #[inline(always)]
    pub fn set_rx_filter_state(&mut self, enabled: bool) {
        self.internal_set_rx_filter_state(enabled)
    }

    /// Enables automatic echo mode.
    ///
    /// In this mode, RX line is internally connected to TX line.
    /// **Transmitter is disconnected from TX line, and therefore unusable in this mode**.
    /// Receiver operates normally in this mode.
    pub fn switch_to_automatic_echo_mode(&mut self) {
        Instance::registers()
            .mr
            .modify(|_, w| w.chmode().automatic());
    }
}
