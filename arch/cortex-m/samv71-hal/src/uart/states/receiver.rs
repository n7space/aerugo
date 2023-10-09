//! Module with implementation of UART in receiver mode.

use crate::uart::{
    config::{bool_to_rx_filter_config, rx_filter_config_to_bool},
    metadata::UartMetadata,
    Error, Receiver, UART,
};

impl<Instance: UartMetadata> UART<Instance, Receiver> {
    /// Resets UART receiver.
    ///
    /// Any pending byte reception is aborted when the receiver is reset.
    pub fn reset_receiver(&mut self) {
        self.registers_ref().cr.write(|w| w.rstrx().set_bit());
    }

    /// Returns true if receive line is filtered.
    ///
    /// Filtering is done using a three-sample filter (16x-bit clock, 2 over 3 majority).
    pub fn is_rx_filter_enabled(&self) -> bool {
        rx_filter_config_to_bool(self.registers_ref().mr.read().filter().variant())
    }

    /// Sets receive line filtering state.
    ///
    /// Filtering is done using a three-sample filter (16x-bit clock, 2 over 3 majority).
    pub fn set_rx_filter_state(&mut self, enabled: bool) {
        self.registers_ref()
            .mr
            .modify(|_, w| w.filter().variant(bool_to_rx_filter_config(enabled)));
    }

    /// Receives a single byte. Blocks until a byte is received, or timeout is hit.
    ///
    /// # Parameters
    /// * `timeout_cpu_cycles` - Maximum amount of CPU cycles to wait for the character.
    ///
    /// # Returns
    /// `Ok(u8)` if reception was successful, the value is the received byte.
    /// `Err(())` on timeout.
    pub fn receive_byte(&self, timeout_cpu_cycles: u32) -> Result<u8, Error> {
        match self.wait_for_byte_reception(timeout_cpu_cycles) {
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
        self.registers_ref().rhr.read().rxchr().bits()
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