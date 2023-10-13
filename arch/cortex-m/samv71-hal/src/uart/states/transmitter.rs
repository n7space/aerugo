//! Module with implementation of UART in transmitter mode.
//!
use crate::uart::{metadata::UARTMetadata, Error, Transmit, UART};

impl<Instance: UARTMetadata, State: Transmit> UART<Instance, State> {
    /// Resets UART transmitter.
    ///
    /// Any pending byte transmission is aborted when the transmitter is reset.
    pub fn reset_transmitter(&mut self) {
        Instance::registers().cr.write(|w| w.rsttx().set_bit());
    }

    /// Transmits a single byte.
    /// Does not wait until transmission is completed, use [`UART::flush`] for that.
    ///
    /// # Parameters
    /// * `byte` - Byte to transmit
    /// * `timeout_cycles` - Maximum amount of arbitrary "cycles" to wait until transmitter is ready.
    ///                      This is basically an amount of loop iterations with status checks.
    ///
    /// # Returns
    /// `Ok(())` on successful transmission, `Err(Error::TimedOut)` if timeout has been reached.                         
    pub fn transmit_byte(&mut self, byte: u8, timeout_cycles: u32) -> Result<(), Error> {
        match self.wait_for_transmitter_ready(timeout_cycles) {
            Ok(_) => {
                self.set_transmitted_byte(byte);
                Ok(())
            }
            Err(_) => Err(Error::TimedOut),
        }
    }

    /// Transmits multiple bytes. Blocks until the transmission is completed, or timeout
    /// is hit. Flushes the UART after transmitting last byte.
    ///
    /// # Parameters
    /// * `bytes` - Bytes to transmit.
    /// * `timeout_cycles` - Maximum amount of arbitrary "cycles" to wait until transmission is complete.
    ///                      **Timeout is defined for the whole transaction**, not for single byte's transmission.
    ///                      This is basically an amount of loop iterations with status checks.
    ///
    /// # Returns
    /// `Ok(())` on successful transmission, `Err(Error::TimedOut)` if timeout has been reached.
    pub fn transmit_bytes(&mut self, bytes: &[u8], timeout_cycles: u32) -> Result<(), Error> {
        if let Ok(mut timeout_cycles) = self.wait_for_transmitter_ready(timeout_cycles) {
            for &byte in bytes {
                self.set_transmitted_byte(byte);
                match self.wait_for_transmitter_ready(timeout_cycles) {
                    Ok(remaining_timeout) => timeout_cycles = remaining_timeout,
                    Err(_) => return Err(Error::TimedOut),
                }
            }

            return self.flush(timeout_cycles);
        }

        Err(Error::TimedOut)
    }

    /// Flushes the UART by waiting until currently transmitted character is processed.
    ///
    /// # Parameters
    /// * `timeout_cycles` - Maximum amount of arbitrary "cycles" to wait until a single byte is transmitted.
    ///                      This is basically an amount of loop iterations with status checks.
    ///
    /// # Returns
    /// `Ok(())` on successful flush, `Err(Error::TimedOut)` if timeout has been reached.
    pub fn flush(&mut self, timeout_cycles: u32) -> Result<(), Error> {
        self.wait_for_transmission_to_complete(timeout_cycles)
            .map(|_| ())
            .map_err(|_| Error::TimedOut)
    }

    /// Writes a byte to be transmitted next into TX holding register.
    ///
    /// Doesn't perform any checks. This is simply a wrapper for register write.
    #[inline(always)]
    fn set_transmitted_byte(&mut self, byte: u8) {
        Instance::registers().thr.write(|w| w.txchr().variant(byte));
    }

    /// Blocks the CPU until either the transmission is complete, or timeout is hit.
    ///
    /// # Parameters
    /// * `timeout_cycles` - Maximum amount of arbitrary "cycles" the transmission should take.
    ///                      This is basically an amount of loop iterations with status checks.
    ///
    /// # Returns
    /// `Ok(u32)` if transmission was completed before timeout, `Err(())` if timeout has been reached.
    /// The value returned on success indicates how much CPU cycles are left for next timeout.
    fn wait_for_transmission_to_complete(&self, timeout_cycles: u32) -> Result<u32, ()> {
        self.wait_for_status_flag(|status| status.transmitter_empty, timeout_cycles)
    }

    /// Blocks the CPU until transmit holding register is empty and ready for next byte.
    ///
    /// # Parameters
    /// * `timeout_cycles` - Maximum amount arbitrary "cycles" to wait for the transmitter.
    ///                      This is basically an amount of loop iterations with status checks.
    ///
    /// # Returns
    /// `Ok(u32)` if transmitter became ready before timeout, `Err(())` if timeout has been reached.
    /// The value returned on success indicates how much CPU cycles are left for next timeout.
    fn wait_for_transmitter_ready(&self, timeout_cycles: u32) -> Result<u32, ()> {
        self.wait_for_status_flag(|status| status.transmitter_ready, timeout_cycles)
    }
}
