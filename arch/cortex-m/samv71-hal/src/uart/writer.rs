//! UART Writer implementation.
//!
//! Writer can be used to transmit data via UART.

use core::marker::PhantomData;

use crate::utils::wait_until;

use super::Error;
use super::Status;
use super::UARTMetadata;

/// This structure can be used to transmit data via UART.
///
/// Writer instance is created by [`UART`](super::UART) and can be taken from it using
/// [`UART::take_writer`](super::UART::take_writer) method.
/// Once taken, it can be put inside UART again using [`UART::put_writer`](super::UART::put_writer)
/// for storage.
///
/// # Safety
/// If Writer is used while UART transmitter is disabled, it will always return [`Error::TimedOut`]
/// on blocking operations.
///
/// Writer is thread-safe, as it doesn't share any (mutable) state with UART or Reader, and
/// there can be only a single instance of Writer per UART.
pub struct Writer<Instance: UARTMetadata> {
    /// UART instance marker.
    _uart: PhantomData<Instance>,
}

impl<Instance: UARTMetadata> Writer<Instance> {
    /// Transmits a single byte.
    ///
    /// Waits for UART TX register to be empty.
    /// Does not wait until transmission is completed, use [`Writer::flush`] if you want to make sure
    /// of that.
    ///
    /// If you check "transmitter ready" flag manually (for example, in IRQ handler), you could use
    /// [`Writer::set_transmitted_byte`] instead, as it doesn't perform the additional status check.
    /// However, this function will also work fine in that context, it'll just double-check that.
    ///
    /// # Parameters
    /// * `byte` - Byte to transmit
    /// * `timeout` - Maximum amount of UART status checks before declaring timeout.
    ///
    /// # Returns
    /// `Ok(())` on successful transmission, `Err(Error::TimedOut)` if timeout has been reached.                         
    pub fn transmit_byte(&mut self, byte: u8, timeout: u32) -> Result<(), Error> {
        self.wait_for_transmitter_ready(timeout)
            // Safety: this is safe, as we just verified that transmitter is ready.
            .map_or(Err(Error::TimedOut), |_| unsafe {
                self.set_transmitted_byte(byte);
                Ok(())
            })
    }

    /// Transmits multiple bytes. Blocks until the transmission is completed, or timeout
    /// is hit. Automatically flushes the UART after transmitting last byte.
    ///
    /// # Parameters
    /// * `bytes` - Bytes to transmit.
    /// * `timeout` - Maximum amount of UART status checks before declaring timeout.
    ///               Timeout is defined for **the whole transaction**, not single byte transmission.
    ///
    /// # Returns
    /// `Ok(())` on successful transmission, `Err(Error::TimedOut)` if timeout has been reached.
    pub fn transmit_bytes(&mut self, bytes: &[u8], timeout: u32) -> Result<(), Error> {
        if let Some(mut timeout_cycles) = self.wait_for_transmitter_ready(timeout) {
            for &byte in bytes {
                // Safety: this is safe, as we just verified that transmitter is ready.
                unsafe {
                    self.set_transmitted_byte(byte);
                }

                match self.wait_for_transmitter_ready(timeout_cycles) {
                    Some(remaining_timeout) => timeout_cycles = remaining_timeout,
                    None => return Err(Error::TimedOut),
                }
            }

            return self.flush(timeout_cycles);
        }

        Err(Error::TimedOut)
    }

    /// Flushes the UART by waiting until currently transmitted character is processed.
    ///
    /// # Parameters
    /// * `timeout` - Maximum amount of UART status checks before declaring timeout.
    ///
    /// # Returns
    /// `Ok(())` on successful flush, `Err(Error::TimedOut)` if timeout has been reached.
    pub fn flush(&mut self, timeout: u32) -> Result<(), Error> {
        self.wait_for_transmission_to_complete(timeout)
            .map_or(Err(Error::TimedOut), |_| Ok(()))
    }

    /// Writes a byte to be transmitted next into TX holding register.
    ///
    /// This function is meant to be used primarily in interrupt handlers, as a slightly faster
    /// version of [`Writer::transmit_byte`] that avoids double-checking the status register.
    ///
    /// # Safety
    /// This function doesn't wait for TX holding register to become empty, unlike
    /// [`Writer::transmit_byte`]. Therefore, it's safe to use only if you do that manually by
    /// checking the "transmitter ready" status flag.
    ///
    /// "Transmitter empty" flag can also be used for that purpose, but be aware that it's set when
    /// there's no data in TX holding register **and** transmitter is idle (flushed), so unless
    /// you have to make sure that each byte placed in TX holding register is transmitted before
    /// trying to transmit the next one, it will induce unnecessary performance penalty and may
    /// potentially cause communication issues, depending on receiver's code.
    ///
    /// If transmitted byte is set while there's already a byte in TX holding register, existing
    /// byte will be overwritten and not sent.
    #[inline(always)]
    pub unsafe fn set_transmitted_byte(&mut self, byte: u8) {
        Instance::registers().thr.write(|w| w.txchr().variant(byte));
    }

    /// Returns current UART status.
    pub fn status(&self) -> Status {
        Instance::registers().sr.read().into()
    }

    /// Creates new Reader instance.
    ///
    /// This function should be called only once for each UART instance.
    pub(super) fn new() -> Self {
        Self { _uart: PhantomData }
    }

    /// Blocks the CPU until either the transmission is complete, or timeout is hit.
    ///
    /// # Parameters
    /// * `timeout` - Maximum amount of UART status checks before declaring timeout.
    ///
    /// # Returns
    /// `Some(u32)`, with amount of checks left before "timeout" is hit, or `None` if maximum
    /// checks amount has been reached.
    fn wait_for_transmission_to_complete(&self, timeout: u32) -> Option<u32> {
        wait_until(
            || Instance::registers().sr.read().txrdy().bit_is_set(),
            timeout,
        )
    }

    /// Blocks the CPU until transmit holding register is empty and ready for next byte.
    ///
    /// # Parameters
    /// * `timeout` - Maximum amount of UART status checks before declaring timeout.
    ///
    /// # Returns
    /// `Some(u32)`, with amount of checks left before "timeout" is hit, or `None` if maximum
    /// checks amount has been reached.
    fn wait_for_transmitter_ready(&self, timeout: u32) -> Option<u32> {
        wait_until(
            || Instance::registers().sr.read().txempty().bit_is_set(),
            timeout,
        )
    }
}
