//! This module contains status reader implementation, a structure that allows you to read XDMAC's
//! status register.
//!
//! This structure is a helper that's supposed to be given to XDMAC interrupt handler, or whatever
//! piece of code you're using for checking the XDMAC status.

use super::Xdmac;
use samv71q21_pac::{xdmac::RegisterBlock, XDMAC};

use bitvec::prelude::*;
use heapless::Vec;

/// Helper structure, use it to read XDMAC status.
///
/// After getting it's instance from [`Xdmac`], you can use it to check which channels have pending
/// interrupts.
///
/// **Reading the status register clears the flags inside it, so you should always handle pending
/// interrupts afterwards, as soon as the register is read.**
pub struct StatusReader;

/// Array type returned from StatusReader that contains the status of each channel's interrupt.
/// If flag on index `i` is set to true, then the channel with id `i` has pending interrupt
/// that should be handled immediately.
pub type PendingChannels = Vec<bool, { Xdmac::SUPPORTED_CHANNELS }>;

impl StatusReader {
    /// Returns an array of boolean values, where every n'th value indicates whether n'th channel
    /// currently has pending interrupt.
    ///
    /// For example, to check if 5th, 7th and 10th XDMAC channels have an interrupt pending to be
    /// serviced, do this:
    ///
    /// ```no_run
    /// let interrupts_state = reader.get_pending_interrupts();
    ///
    /// if interrupts_state[5] {
    ///     // Channel 5 has pending IRQ...
    /// }
    ///
    /// if interrupts_state[7] {
    ///     // Channel 7 has pending IRQ...
    /// }
    ///
    /// if interrupts_state[10] {
    ///     // Channel 10 has pending IRQ...
    /// }
    /// ```
    ///
    /// **Reading the status register clears the flags inside it, so you should read it once, and
    /// immediately handle all channels with pending IRQs.
    ///
    /// [`Channel`](super::Channel) provides similar structure for handling specific channel IRQs.
    pub fn get_pending_channels(&mut self) -> PendingChannels {
        let status = self.xdmac_registers_ref().gis.read();
        let status_bits = status.bits();
        status_bits.view_bits::<Lsb0>()[0..Xdmac::SUPPORTED_CHANNELS]
            .iter()
            .map(|channel_bit| channel_bit == true)
            .collect()
    }

    /// Returns a reference to XDMAC's registers.
    fn xdmac_registers_ref(&self) -> &RegisterBlock {
        // Safety: This is safe, as the address of XDMAC register is guaranteed to be valid by Xdmac.
        unsafe { &*Self::XDMAC_REGISTERS }
    }

    /// Pointer to XDMAC's registers.
    const XDMAC_REGISTERS: *const RegisterBlock = XDMAC::PTR;
}
