//! Module with implementation of UART in transmitter mode.
//!
use crate::uart::{metadata::UartMetadata, Error, Transmitter, UART};

impl<Instance: UartMetadata> UART<Instance, Transmitter> {
    /// Resets UART transmitter.
    ///
    /// Any pending byte transmission is aborted when the transmitter is reset.
    pub fn reset_transmitter(&mut self) {
        self.registers_ref().cr.write(|w| w.rsttx().set_bit());
    }

    /// Transmits a single byte. Blocks until the transmission is completed, or timeout
    /// is hit.
    ///
    /// UART has no direct PMC dependency, and PMC doesn't support clock frequency calculations
    /// yet, therefore the timeout is specified as amount of CPU cycles to wait until UART
    /// finishes the transmission.
    ///
    /// # Parameters
    /// * `byte` - Byte to transmit
    /// * `timeout_cpu_cycles` - Maximum amount of CPU cycles the transmission should take.
    ///
    /// # Returns
    /// `Ok(())` on successful transmission, `Err(())` if timeout has been reached.                         
    pub fn transmit_byte(&mut self, byte: u8, timeout_cpu_cycles: u32) -> Result<(), Error> {
        if let Ok(timeout_cpu_cycles) = self.wait_for_transmitter_ready(timeout_cpu_cycles) {
            self.set_transmitted_byte(byte);
            return match self.wait_for_transmission_to_complete(timeout_cpu_cycles) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::TimedOut),
            };
        }

        Err(Error::TimedOut)
    }

    /// Transmits multiple bytes. Blocks until the transmission is completed, or timeout
    /// is hit.
    ///
    /// This function is more optimal than calling [`UART::transmit_byte`] in a loop, as
    /// it will feed the holding register as soon as possible, instead of waiting for transmission
    /// to finish. It should be preferred for this kind of operations.
    ///
    /// # Parameters
    /// * `bytes` - Bytes to transmit.
    /// * `timeout_cpu_cycles` - Maximum amount of CPU cycles the transmission of whole buffer should take.
    ///
    /// # Returns
    /// `Ok(())` on successful transmission, `Err(())` if timeout has been reached.
    pub fn transmit_bytes(&mut self, bytes: &[u8], timeout_cpu_cycles: u32) -> Result<(), Error> {
        if let Ok(mut timeout_cpu_cycles) = self.wait_for_transmitter_ready(timeout_cpu_cycles) {
            for &byte in bytes {
                self.set_transmitted_byte(byte);
                match self.wait_for_transmitter_ready(timeout_cpu_cycles) {
                    Ok(remaining_timeout) => timeout_cpu_cycles = remaining_timeout,
                    Err(_) => return Err(Error::TimedOut),
                }
            }

            return match self.wait_for_transmission_to_complete(timeout_cpu_cycles) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::TimedOut),
            };
        }

        Err(Error::TimedOut)
    }

    /// Writes a byte to be transmitted next into TX holding register.
    ///
    /// Doesn't perform any checks. This is simply a wrapper for register write.
    #[inline(always)]
    fn set_transmitted_byte(&mut self, byte: u8) {
        self.registers_ref().thr.write(|w| w.txchr().variant(byte));
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
