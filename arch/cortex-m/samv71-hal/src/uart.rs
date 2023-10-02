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

use samv71q21_pac::uart0::mr::FILTERSELECT_A;

use self::metadata::{RegisterBlock, UartMetadata};

pub use self::config::{ClockSource, Config, Mode, ParityBit};
pub use self::interrupt::Interrupt;
pub use self::status::Status;

pub mod config;
pub mod interrupt;
pub mod metadata;
pub mod status;

/// Structure representing UART driver
pub struct UART<Metadata: UartMetadata> {
    /// PAC UART instance metadata.
    _meta: PhantomData<Metadata>,
}

impl<Instance: UartMetadata> UART<Instance> {
    /// Returns current UART status.
    ///
    /// Error flags **must** be cleared manually by calling [`UART::reset_status`].
    pub fn status(&self) -> Status {
        let reg = self.registers_ref().sr.read();

        Status {
            comparison_matched: reg.cmp().bit_is_set(),
            transmitter_empty: reg.txempty().bit_is_set(),
            parity_error: reg.pare().bit_is_set(),
            framing_error: reg.frame().bit_is_set(),
            overrun_error: reg.ovre().bit_is_set(),
            transmitter_ready: reg.txrdy().bit_is_set(),
            receiver_ready: reg.rxrdy().bit_is_set(),
        }
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

    /// Returns current UART configuration (mode, clock source, parity bit config and RX filtering state)
    ///
    /// This is a single-read operation, and it should be preferred instead of concrete functions
    /// (mode, clock_source, etc.) when multiple config values are needed.
    pub fn config(&self) -> Config {
        let reg = self.registers_ref().mr.read();

        // If the register holds invalid parity bit configuration, it's reasonable to panic here.
        Config {
            mode: reg.chmode().variant().into(),
            clock_source: reg.brsrcck().variant().into(),
            parity: reg.par().variant().unwrap().into(),
            rx_filter_enabled: reg.filter().is_enabled(),
        }
    }

    /// Sets UART configuration.
    ///
    /// This is a single-write operation, and it should be preferred instead of concrete functions
    /// (set_mode, set_clock_source, etc.) if you intend to change multiple configuration settings.
    pub fn set_config(&mut self, config: Config) {
        self.registers_ref().mr.write(|w| {
            w.chmode()
                .variant(config.mode.into())
                .brsrcck()
                .variant(config.clock_source.into())
                .par()
                .variant(config.parity.into())
                .filter()
                .variant(Self::bool_to_rx_filter_config(config.rx_filter_enabled))
        });
    }

    /// Returns current UART mode (normal/loopback)
    pub fn mode(&self) -> Mode {
        self.registers_ref().mr.read().chmode().variant().into()
    }

    /// Sets UART mode (normal/loopback)
    pub fn set_mode(&mut self, mode: Mode) {
        self.registers_ref()
            .mr
            .modify(|_, w| w.chmode().variant(mode.into()));
    }

    /// Returns current clock source used to drive UART baudrate.
    pub fn clock_source(&self) -> ClockSource {
        self.registers_ref().mr.read().brsrcck().variant().into()
    }

    /// Sets clock source used to drive UART baudrate.
    pub fn set_clock_source(&mut self, source: ClockSource) {
        self.registers_ref()
            .mr
            .modify(|_, w| w.brsrcck().variant(source.into()));
    }

    /// Returns current parity bit configuration.
    pub fn parity_bit_config(&self) -> ParityBit {
        // If the register holds invalid parity bit configuration, it's reasonable to panic here.
        self.registers_ref()
            .mr
            .read()
            .par()
            .variant()
            .unwrap()
            .into()
    }

    /// Sets parity bit configuration.
    pub fn set_parity_bit_config(&mut self, config: ParityBit) {
        self.registers_ref()
            .mr
            .modify(|_, w| w.par().variant(config.into()));
    }

    /// Returns true if receive line is filtered.
    ///
    /// Filtering is done using a three-sample filter (16x-bit clock, 2 over 3 majority).
    pub fn rx_filter_enabled(&self) -> bool {
        Self::rx_filter_config_to_bool(self.registers_ref().mr.read().filter().variant())
    }

    /// Sets receive line filtering state.
    ///
    /// Filtering is done using a three-sample filter (16x-bit clock, 2 over 3 majority).
    pub fn set_rx_filter_state(&mut self, enabled: bool) {
        self.registers_ref()
            .mr
            .modify(|_, w| w.filter().variant(Self::bool_to_rx_filter_config(enabled)));
    }

    /// Returns `true` if specified interrupt is currently enabled.
    pub fn is_interrupt_enabled(&self, interrupt: Interrupt) -> bool {
        let reg = self.registers_ref().imr.read();

        match interrupt {
            Interrupt::Comparison => reg.cmp().bit_is_set(),
            Interrupt::TxEmpty => reg.txempty().bit_is_set(),
            Interrupt::ParityError => reg.pare().bit_is_set(),
            Interrupt::FramingError => reg.frame().bit_is_set(),
            Interrupt::OverrunError => reg.ovre().bit_is_set(),
            Interrupt::TxReady => reg.txrdy().bit_is_set(),
            Interrupt::RxReady => reg.rxrdy().bit_is_set(),
        }
    }

    /// Enables specified interrupt.
    pub fn enable_interrupt(&mut self, interrupt: Interrupt) {
        self.registers_ref().ier.write(|w| match interrupt {
            Interrupt::Comparison => w.cmp().set_bit(),
            Interrupt::TxEmpty => w.txempty().set_bit(),
            Interrupt::ParityError => w.pare().set_bit(),
            Interrupt::FramingError => w.frame().set_bit(),
            Interrupt::OverrunError => w.ovre().set_bit(),
            Interrupt::TxReady => w.txrdy().set_bit(),
            Interrupt::RxReady => w.rxrdy().set_bit(),
        });
    }

    /// Disables specified interrupt.
    pub fn disable_interrupt(&mut self, interrupt: Interrupt) {
        self.registers_ref().idr.write(|w| match interrupt {
            Interrupt::Comparison => w.cmp().set_bit(),
            Interrupt::TxEmpty => w.txempty().set_bit(),
            Interrupt::ParityError => w.pare().set_bit(),
            Interrupt::FramingError => w.frame().set_bit(),
            Interrupt::OverrunError => w.ovre().set_bit(),
            Interrupt::TxReady => w.txrdy().set_bit(),
            Interrupt::RxReady => w.rxrdy().set_bit(),
        });
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

    /// Converts RX filter selection to boolean.
    ///
    /// `From` cannot be implemented for `bool`, as it's an external type.
    #[inline(always)]
    const fn rx_filter_config_to_bool(config: FILTERSELECT_A) -> bool {
        match config {
            FILTERSELECT_A::DISABLED => false,
            FILTERSELECT_A::ENABLED => true,
        }
    }

    /// Converts boolean into RX filter selection.
    ///
    /// `From` cannot be implemented for `bool`, as it's an external type.
    #[inline(always)]
    const fn bool_to_rx_filter_config(filter_enabled: bool) -> FILTERSELECT_A {
        match filter_enabled {
            true => FILTERSELECT_A::ENABLED,
            false => FILTERSELECT_A::DISABLED,
        }
    }
}
