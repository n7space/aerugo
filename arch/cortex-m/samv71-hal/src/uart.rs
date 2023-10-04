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

pub use super::time::HertzU32 as Frequency;

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

/// Constant representing oversampling ratio, which is used in baudrate calculations.
const OVERSAMPLING_RATIO: u32 = 16;

/// Enumeration representing an invalid baudrate.
/// This error indicates that the baudrate passed to [`UART::set_baudrate`]
/// cannot be represented, e.g. it would cause the clock divisor to be
/// zero, disabling the baudrate clock, or it would cause it to be larger
/// than it's maximum possible value.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InvalidBaudrate {
    /// Specified baudrate is too low, and it would result in clock divisor
    /// larger than maximum possible value.
    TooLow,
    /// Specified baudrate is too high, and it would cause the clock divisor
    /// to be zero, effectively disabling baudrate clock.
    TooHigh,
}

/// Enumeration representing UART I/O errors.
/// These errors might happen either when reception or transmission fails.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IOError {
    /// Timeout was reached.
    Timeout,
}

impl<Instance: UartMetadata> UART<Instance> {
    /// Create UART instance. Consumes PAC UART instance to prevent creating multiple instances
    /// of UART driver for the same UART peripheral.
    pub fn new(_uart: Instance) -> Self {
        Self { _meta: PhantomData }
    }

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
    /// If a byte is being processed, and a byte has been written to UART holding
    /// register, both bytes are transmitted before the transmitter is stopped.
    pub fn disable_transmitter(&mut self) {
        self.registers_ref().cr.write(|w| w.txdis().set_bit());
    }

    /// Resets UART transmitter.
    ///
    /// Any pending byte transmission is aborted when the transmitter is reset.
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
    /// If a byte is being processed, reception is completed before receiver is stopped.
    pub fn disable_receiver(&mut self) {
        self.registers_ref().cr.write(|w| w.rxdis().set_bit());
    }

    /// Resets UART receiver.
    ///
    /// Any pending byte reception is aborted when the receiver is reset.
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

    /// Returns current clock divisor.
    ///
    /// Clock divisor is used for calculating baudrate.
    /// If you intend to set specific baudrate, instead of calculating it
    /// manually, use [insert a function when it's done here].
    ///
    /// Clock divisor is defined as source clock frequency divided by
    /// 16*baudrate.
    ///
    /// Clock source can be changed with [`UART::set_clock_source`]
    /// and [`UART::set_config`].
    ///
    /// If the divisor is equal to 0, baud rate clock is disabled.
    pub fn clock_divisor(&self) -> u16 {
        self.registers_ref().brgr.read().cd().bits()
    }

    /// Sets the clock divisor.
    ///
    /// Clock divisor is used for calculating baudrate.
    /// If you intend to set specific baudrate, instead of calculating it
    /// manually, use [insert a function when it's done here].
    ///
    /// Clock divisor is defined as source clock frequency divided by
    /// 16*baudrate.
    ///
    /// Clock source can be changed with [`UART::set_clock_source`]
    /// and [`UART::set_config`].
    ///
    /// # Safety
    /// If the divisor is equal to 0, baud rate clock is disabled.
    /// Therefore, this function is unsafe, as it has potential, unwanted
    /// side-effect.
    pub unsafe fn set_clock_divisor(&mut self, divisor: u16) {
        self.registers_ref().brgr.write(|w| w.cd().variant(divisor));
    }

    /// Returns current UART baudrate (in bits per second).
    ///
    /// # Parameters
    /// * `source_clock_frequency` - Frequency of the UART source clock.
    ///                              Required for calculations.
    ///
    /// Clock source can be changed with [`UART::set_clock_source`]
    /// and [`UART::set_config`].
    pub fn baudrate(&self, source_clock_frequency: Frequency) -> u32 {
        let divisor = self.clock_divisor();
        source_clock_frequency.to_Hz() / (OVERSAMPLING_RATIO * (divisor as u32))
    }

    /// Sets UART baudrate.
    ///
    /// # Parameters
    /// * `baudrate` - Desired baudrate, in bits per second.
    /// * `source_clock_frequency` - Frequency of the UART source clock.
    ///                              Required for calculations.
    ///
    /// # Remarks
    /// Because of the fact that clock divisor is 16-bit, specified baudrate
    /// might not be possible to be precisely configured. This function will calculate
    /// the approximate clock divisor for specified baudrate using rough integer
    /// division. True baudrate is returned from this function.
    ///
    /// Clock source can be changed with [`UART::set_clock_source`]
    /// and [`UART::set_config`].
    ///
    /// # Returns
    /// `Ok(u32)` if baudrate was set successfully. Returned `u32` is the true baudrate.
    /// `Err([InvalidBaudrate])` if specified baudrate cannot be set.
    pub fn set_baudrate(
        &mut self,
        baudrate: u32,
        source_clock_frequency: Frequency,
    ) -> Result<u32, InvalidBaudrate> {
        let divisor = source_clock_frequency.to_Hz() / (OVERSAMPLING_RATIO * baudrate);

        // If provided baudrate is larger than source clock frequency / 16, it will
        // cause the divisor to be truncated to 0, which will disable the baudrate clock.
        // If you intend to disable the baudrate clock that way, set the divisor to 0
        // directly, using `UART::set_clock_divisor`.
        if divisor == 0 {
            return Err(InvalidBaudrate::TooHigh);
        }

        // If provided baudrate is small enough, it will cause the divisor to be
        // larger than (2^16) - 1, which would cause an integer overflow if tried
        // to be converted to it's type.
        if divisor > (u16::MAX as u32) {
            return Err(InvalidBaudrate::TooLow);
        }

        // Safety: This is safe, because the divisor is validated.
        unsafe {
            self.set_clock_divisor(divisor as u16);
        }

        Ok(source_clock_frequency.to_Hz() / (OVERSAMPLING_RATIO * divisor))
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
    pub fn transmit_byte(&mut self, byte: u8, timeout_cpu_cycles: u32) -> Result<(), IOError> {
        if let Ok(timeout_cpu_cycles) = self.wait_for_transmitter_ready(timeout_cpu_cycles) {
            self.set_transmitted_byte(byte);
            return match self.wait_for_transmission_to_complete(timeout_cpu_cycles) {
                Ok(_) => Ok(()),
                Err(_) => Err(IOError::Timeout),
            };
        }

        Err(IOError::Timeout)
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
    pub fn transmit_bytes(&mut self, bytes: &[u8], timeout_cpu_cycles: u32) -> Result<(), IOError> {
        if let Ok(mut timeout_cpu_cycles) = self.wait_for_transmitter_ready(timeout_cpu_cycles) {
            for &byte in bytes {
                self.set_transmitted_byte(byte);
                match self.wait_for_transmitter_ready(timeout_cpu_cycles) {
                    Ok(remaining_timeout) => timeout_cpu_cycles = remaining_timeout,
                    Err(_) => return Err(IOError::Timeout),
                }
            }

            return match self.wait_for_transmission_to_complete(timeout_cpu_cycles) {
                Ok(_) => Ok(()),
                Err(_) => Err(IOError::Timeout),
            };
        }

        Err(IOError::Timeout)
    }

    /// Receives a single byte. Blocks until a byte is received, or timeout is hit.
    ///
    /// # Parameters
    /// * `timeout_cpu_cycles` - Maximum amount of CPU cycles to wait for the character.
    ///
    /// # Returns
    /// `Ok(u8)` if reception was successful, the value is the received byte.
    /// `Err(())` on timeout.
    pub fn receive_byte(&self, timeout_cpu_cycles: u32) -> Result<u8, IOError> {
        match self.wait_for_byte_reception(timeout_cpu_cycles) {
            Ok(_) => Ok(self.get_received_byte()),
            Err(_) => Err(IOError::Timeout),
        }
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

    /// Writes a byte to be transmitted next into TX holding register.
    ///
    /// Doesn't perform any checks. This is simply a wrapper for register write.
    #[inline(always)]
    fn set_transmitted_byte(&mut self, byte: u8) {
        self.registers_ref().thr.write(|w| w.txchr().variant(byte));
    }

    /// Returns the byte currently stored in received character register.
    ///
    /// Doesn't perform any checks. This is simply a wrapper for register read.
    /// Will return `0` if no data has been received yet.
    #[inline(always)]
    fn get_received_byte(&self) -> u8 {
        self.registers_ref().rhr.read().rxchr().bits()
    }

    /// Waits until provided functor returns `true`. Functor receives UART status and should return
    /// the specific flag (or their combination).
    ///
    /// # Parameters
    /// * `status_checker` - Functor, returning flag or combination of status flags to wait for
    /// * `timeout_cpu_cycles` - Maximum amount of CPU cycles to spend on waiting for the flag.
    ///
    /// # Returns
    /// `Ok(u32)` if functor returned `true` before timeout, `Err(())` if timeout has been reached.
    /// The value returned on success indicates how much CPU cycles are left for next timeout.
    fn wait_for_status_flag<F>(&self, status_checker: F, timeout_cpu_cycles: u32) -> Result<u32, ()>
    where
        F: Fn(Status) -> bool,
    {
        for wasted_cycles in 0..timeout_cpu_cycles {
            if status_checker(self.status()) {
                return Ok(timeout_cpu_cycles - wasted_cycles);
            }
        }

        Err(())
    }

    /// Blocks the CPU until either the transmission is complete, or timeout is hit.
    ///
    /// # Parameters
    /// * `timeout_cpu_cycles` - Maximum amount of CPU cycles the transmission should take.
    ///
    /// # Returns
    /// `Ok(u32)` if transmission was completed before timeout, `Err(())` if timeout has been reached.
    /// The value returned on success indicates how much CPU cycles are left for next timeout.
    fn wait_for_transmission_to_complete(&self, timeout_cpu_cycles: u32) -> Result<u32, ()> {
        self.wait_for_status_flag(|status| status.transmitter_empty, timeout_cpu_cycles)
    }

    /// Blocks the CPU until transmit holding register is empty and ready for next byte.
    ///
    /// # Parameters
    /// * `timeout_cpu_cycles` - Maximum amount of CPU cycles to wait for the transmitter.
    ///
    /// # Returns
    /// `Ok(u32)` if transmitter became ready before timeout, `Err(())` if timeout has been reached.
    /// The value returned on success indicates how much CPU cycles are left for next timeout.
    fn wait_for_transmitter_ready(&self, timeout_cpu_cycles: u32) -> Result<u32, ()> {
        self.wait_for_status_flag(|status| status.transmitter_ready, timeout_cpu_cycles)
    }

    /// Blocks the CPU until a byte is received.
    ///
    /// # Parameters
    /// * `timeout_cpu_cycles` - Maximum amount of CPU cycles to wait for reception.
    ///
    /// # Returns
    /// `Ok(u32)` if byte was received before timeout, `Err(())` if timeout has been reached.
    /// The value returned on success indicates how much CPU cycles are left for next timeout.
    fn wait_for_byte_reception(&self, timeout_cpu_cycles: u32) -> Result<u32, ()> {
        self.wait_for_status_flag(|status| status.receiver_ready, timeout_cpu_cycles)
    }
}
