//! Module with implementation of UART in receiver mode.

use crate::uart::{config::rx_filter_config_to_bool, metadata::UARTMetadata, Error, Receive, UART};

impl<Instance: UARTMetadata, State: Receive> UART<Instance, State> {
    /// Receives a single byte. Blocks until a byte is received, or timeout is hit.
    ///
    /// # Parameters
    /// * `timeout_cycles` - Maximum amount of arbitrary "cycles" to spend on waiting for the flag.
    ///                      This is basically an amount of loop iterations with status checks.
    ///
    /// # Returns
    /// `Ok(u8)` if reception was successful, the value is the received byte.
    /// `Err(())` on timeout.
    pub fn receive_byte(&self, timeout_cycles: u32) -> Result<u8, Error> {
        match self.wait_for_byte_reception(timeout_cycles) {
            Ok(_) => Ok(self.get_received_byte()),
            Err(_) => Err(Error::TimedOut),
        }
    }

    /// Returns the byte currently stored in received character register.
    ///
    /// Doesn't perform any checks. This is simply a wrapper for register read.
    /// Will return `0` if no data has been received yet.
    #[inline(always)]
    fn get_received_byte(&self) -> u8 {
        Instance::registers().rhr.read().rxchr().bits()
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

    /// Blocks the CPU until a byte is received.
    ///
    /// # Parameters
    /// * `timeout_cycles` - Maximum amount of arbitrary "cycles" to wait for byte reception.
    ///                      This is basically an amount of loop iterations with status checks.
    ///
    /// # Returns
    /// `Ok(u32)` if byte was received before timeout, `Err(())` if timeout has been reached.
    /// The value returned on success indicates how much CPU cycles are left for next timeout.
    fn wait_for_byte_reception(&self, timeout_cycles: u32) -> Result<u32, ()> {
        self.wait_for_status_flag(|status| status.receiver_ready, timeout_cycles)
    }
}
