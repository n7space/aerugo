//! Implementation of Universal Asynchronous Receiver/Transmitter (UART) HAL driver.
//!
//! Before using UART, make sure to
//! - Enable UART peripheral clock using PMC driver
//! - Set appropriate pins mode to peripheral mode using PIO driver
//! Consult the SAMV71 datasheet and [Safety](#safety) section for details.
//!
//! This driver allows you to configure and use any available UART driver in safe way.
//! Currently, the driver supports:
//! - Baudrate and source clock configuration
//! - Transmitter and receiver status checks and configuration
//! - Mode configuration (normal/echo/loopback)
//! - Parity configuration
//! - Parity/overrun/framing error detection
//! - Interrupt configuration
//! - Digital filter configuration
//!
//! Currently, it does NOT support:
//! - Comparison configuration
//! - Sleepwalking mode
//!
//! # Safety
//! UART can be driven by either peripheral or programmable clock.
//! If programmable clock (PCK4) is selected, the baud rate is independent of the
//! processor/bus clock, thus the processor clock can be changed while UART is enabled.
//! However, only setting of processor clock that can be changed while UART is enabled is
//! Main Clock (MCK) prescaler. Other methods to modify the processor/bus clock frequency
//! (PLL multiplier, etc.) are forbidden when UART is enabled.
//!
//! The peripheral clock frequency must be at least three times higher than PCK.

use core::marker::PhantomData;

use self::metadata::{RegisterBlock, UartMetadata};

pub use self::status::Status;

pub mod interrupt;
pub mod metadata;
pub mod mode;
pub mod status;

/// Structure representing UART driver
pub struct UART<Metadata: UartMetadata> {
    /// PAC UART instance metadata.
    _meta: PhantomData<Metadata>,
}

impl<Instance: UartMetadata> UART<Instance> {
    /// Returns current UART status.
    ///
    /// Status flags must be manually cleared by calling [`UART::reset_status`](Self::reset_status)
    pub fn status(&self) -> Status {
        todo!()
    }

    /// Resets UART status by clearing status flags.
    /// **This function should usually be called immediately after reading the status.**
    pub fn reset_status(&mut self) {
        self.registers_ref().cr.write(|w| w.rststa().set_bit());
    }

    /// Enables UART transmitter.
    ///
    /// By default, the transmitter is disabled and it must be enabled before trying to
    /// transmit the data.
    pub fn enable_transmitter(&mut self) {
        self.registers_ref().cr.write(|w| w.txen().set_bit());
    }

    /// Disables UART transmitter.
    ///
    /// If a character is being processed, and a character has been written to UART holding
    /// register, both characters are transmitted before the transmitter is stopped.
    pub fn disable_transmitter(&mut self) {
        self.registers_ref().cr.write(|w| w.txdis().set_bit());
    }

    /// Resets UART transmitter.
    ///
    /// Any pending character transmission is aborted when the transmitter is reset.
    pub fn reset_transmitter(&mut self) {
        self.registers_ref().cr.write(|w| w.rsttx().set_bit());
    }

    /// Enables UART receiver.
    ///
    /// By default, the receiver is disabled and it must be enabled before trying to
    /// receive the data.
    pub fn enable_receiver(&mut self) {
        self.registers_ref().cr.write(|w| w.rxen().set_bit());
    }

    /// Disables UART receiver.
    ///
    /// If a character is being processed, reception is completed before receiver is stopped.
    pub fn disable_receiver(&mut self) {
        self.registers_ref().cr.write(|w| w.rxdis().set_bit());
    }

    /// Resets UART receiver.
    ///
    /// Any pending character reception is aborted when the receiver is reset.
    pub fn reset_receiver(&mut self) {
        self.registers_ref().cr.write(|w| w.rstrx().set_bit());
    }

    /// Returns reference to UART registers.
    ///
    /// # Safety
    /// This function dereferences a raw pointer.
    /// It's safe to use, as long as there aren't multiple instances
    /// of UART sharing the same register.
    fn registers_ref(&self) -> &RegisterBlock {
        unsafe { &*Instance::REGISTERS }
    }
}
