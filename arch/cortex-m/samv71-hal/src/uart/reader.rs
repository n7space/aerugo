//! UART Reader implementation.
//!
//! Reader can be used to receive data via UART.

use core::marker::PhantomData;

use crate::utils::wait_until;

use super::Error;
use super::Status;
use super::UARTMetadata;

/// This structure can be used to receive data via UART.
///
/// Reader instance is created by [`UART`](super::UART) and can be taken from it using
/// [`UART::take_reader`](super::UART::take_reader) method.
/// Once taken, it can be put inside UART again using [`UART::put_reader`](super::UART::put_reader)
/// for storage.
///
/// # Safety
/// If Reader is used while UART receiver is disabled, it will always return [`Error::TimedOut`] on
/// blocking operations.
///
/// Reader is thread-safe, as it doesn't share any (mutable) state with UART or Writer, and
/// there can be only a single instance of Reader per UART.
pub struct Reader<Instance: UARTMetadata> {
    /// UART instance marker.
    _uart: PhantomData<Instance>,
}

impl<Instance: UARTMetadata> Reader<Instance> {
    /// Receives a single byte. Blocks until a byte is received, or timeout is hit.
    ///
    /// If you check "receiver ready" flag manually (for example, in IRQ handler), you could use
    /// [`Reader::get_received_byte`] instead, as it doesn't perform the additional status check.
    /// However, this function will also work fine in that context, it'll just double-check that.
    ///
    /// This function requires mutable access to Reader, as reading the character from RX holding
    /// register while "receiver ready" flag is set will reset it's state and clear this flag.
    ///
    /// # Parameters
    /// * `timeout` - Maximum amount of UART status checks before declaring timeout.
    ///
    /// # Returns
    /// `Ok(u8)` if reception was successful, with the value of received byte.
    /// `Err(())` on timeout.
    pub fn receive_byte(&mut self, timeout: u32) -> Result<u8, Error> {
        self.wait_for_byte_reception(timeout)
            // This is safe, as we just verified that receiver is ready and RX holding register
            // contains a received byte.
            .map_or(Err(Error::TimedOut), |_| unsafe {
                Ok(self.get_received_byte())
            })
    }

    /// Returns the byte currently stored in received character register.
    ///
    /// This function is meant to be used primarily in interrupt handlers, as a slightly faster
    /// version of [`Reader::receive_byte`] that avoids double-checking the status register.
    ///
    /// This function requires mutable access to Reader, as reading the character from RX holding
    /// register while "receiver ready" flag is set will reset it's state and clear this flag.
    ///
    /// # Safety
    /// This function doesn't wait for UART to indicate that there's data in RX register, and will
    /// return `0` if there's no received data there, instead of an error.
    /// Therefore, it's reasonable to use only if you manually checked if there's new data in UART
    /// RX register (by checking "receiver ready" status flag). If you do that, then this function
    /// becomes safe to use.
    ///
    /// # Returns
    /// Received byte, if UART status flag indicates that there's one in RX register.
    /// `0`` otherwise.
    #[inline(always)]
    pub unsafe fn get_received_byte(&mut self) -> u8 {
        Instance::registers().rhr.read().rxchr().bits()
    }

    /// Returns current UART status.
    ///
    /// Error flags **must** be cleared manually by calling [`Reader::reset_status`].
    pub fn status(&self) -> Status {
        Instance::registers().sr.read().into()
    }

    /// Resets UART status by clearing status flags.
    /// **This function should usually be called immediately after reading the status.**
    #[inline(always)]
    pub fn reset_status(&mut self) {
        Instance::registers().cr.write(|w| w.rststa().set_bit());
    }

    /// Creates new Reader instance.
    ///
    /// This function should be called only once for each UART instance.
    pub(super) fn new() -> Self {
        Self { _uart: PhantomData }
    }

    /// Blocks the CPU until either a byte is received, or timeout is hit.
    ///
    /// # Parameters
    /// * `timeout` - Maximum amount of UART status checks before declaring timeout.
    ///
    /// # Returns
    /// `Some(u32)`, with amount of checks left before "timeout" is hit, or `None` if maximum
    /// checks amount has been reached.
    fn wait_for_byte_reception(&self, timeout: u32) -> Option<u32> {
        wait_until(
            || Instance::registers().sr.read().rxrdy().bit_is_set(),
            timeout,
        )
    }
}
