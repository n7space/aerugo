//! Module with implementation of UART for all states implementing [`Configured`] typestate trait

use crate::uart::{
    config::{calculate_clock_divider, ConfigurationError, LoopbackMode},
    metadata::UartMetadata,
    ClockSource, Config, Configured, Frequency, Interrupt, ParityBit, Status, UART,
};

impl<Instance: UartMetadata, State: Configured> UART<Instance, State> {
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
    #[inline(always)]
    pub fn reset_status(&mut self) {
        self.internal_reset_status()
    }

    /// Returns current loopback mode of UART.
    ///
    /// By default, loopback is disabled (and therefore, this function
    /// should return [`LoopbackMode::None`]). Loopback modes can be
    /// enabled with dedicated functions, which are implemented for compatible
    /// UART states.
    ///
    /// See
    /// * [`UART::switch_to_normal_mode`]
    /// * [`UART::switch_to_automatic_echo_mode`]
    /// * [`UART::switch_to_local_loopback_mode`]
    /// * [`UART::switch_to_remote_loopback_mode`]
    /// for details.
    pub fn loopback_mode(&self) -> LoopbackMode {
        self.registers_ref().mr.read().chmode().variant().into()
    }

    /// Switches UART into normal mode. This is the default mode of operation.
    ///
    /// In this mode, transmitter is connected to TX line and receiver to RX line.
    #[inline(always)]
    pub fn switch_to_normal_mode(&mut self) {
        self.internal_switch_to_normal_mode()
    }

    /// Switches UART into remote loopback mode.
    ///
    /// In this mode, RX line is internally connected to TX line.
    /// **Transmitter and receiver are disconnected from TX and RX lines.**
    /// Receiver is pulled to Vdd.
    ///
    /// Communication is impossible in this mode.
    pub fn switch_to_remote_loopback_mode(&mut self) {
        self.registers_ref()
            .mr
            .modify(|_, w| w.chmode().remote_loopback());
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

    /// Returns current UART [`Config`].
    pub fn config(&self) -> Config {
        let reg = self.registers_ref().mr.read();

        // All following `unwraps` are deliberate. If creating Config fails
        // here, then the internal state of UART (or the driver) is invalid,
        // which should never happen in Configured state.
        Config::new(self.baudrate(), self.clock_source_frequency.expect("invalid UART state - clock source frequency not set while UART is in configured state"))
            .unwrap()
            .with_clock_source(reg.brsrcck().variant().into())
            .with_parity_bit(reg.par().variant().expect("invalid UART state - unexpected value in parity bit register").into())
    }

    /// Returns current UART baudrate (in bits per second).
    #[inline(always)]
    pub fn baudrate(&self) -> u32 {
        // Safety: `internal_get_baudrate` panics when it's called while
        // source clock is not configured. This UART state (`Configured`)
        // guarantees that's not the case.
        unsafe { self.internal_get_baudrate() }
    }

    /// Calculates the clock divider for provided baudrate and writes it to UART registers.
    ///
    /// # Parameters
    /// * `baudrate` - New baudrate to be set
    ///
    /// # Returns
    /// [Ok(())] if baudrate was set successfully, [Err(ConfigurationError)] if baudrate
    /// is invalid for current clock configuration.
    pub fn set_baudrate(&mut self, baudrate: u32) -> Result<(), ConfigurationError> {
        // Safety: In `Configured` state, clock source frequency must be set.
        let divider = calculate_clock_divider(baudrate, self.clock_source_frequency.unwrap())?;

        // Safety: divider has been validated above.
        unsafe {
            self.internal_set_clock_divider(divider);
        }
        Ok(())
    }

    /// Returns current clock source used to drive UART baudrate.
    ///
    /// Clock source (and it's frequency) can only be changed by re-initializing UART driver.
    pub fn clock_source(&self) -> ClockSource {
        self.registers_ref().mr.read().brsrcck().variant().into()
    }

    /// Returns currently configured clock source frequency.
    ///
    /// Clock source (and it's frequency) can only be changed by re-initializing UART driver.
    pub fn clock_source_frequency(&self) -> Frequency {
        self.clock_source_frequency.unwrap()
    }

    /// Returns current parity bit configuration.
    pub fn parity_bit(&self) -> ParityBit {
        // If the register holds invalid parity bit configuration, it's reasonable to panic here.
        self.registers_ref()
            .mr
            .read()
            .par()
            .variant()
            .expect("invalid UART state - unexpected value in parity bit register")
            .into()
    }

    /// Sets parity bit configuration.
    ///
    /// # Parameters
    /// * `config` - New parity bit config
    pub fn set_parity_bit(&mut self, config: ParityBit) {
        self.registers_ref()
            .mr
            .modify(|_, w| w.par().variant(config.into()));
    }

    /// Returns current clock divider.
    ///
    /// Clock divider is used for calculating baudrate.
    /// If you intend to set specific baudrate, instead of calculating it
    /// manually, use [insert a function when it's done here].
    ///
    /// Clock divider is defined as clock source frequency divided by
    /// (16*baudrate).
    ///
    /// Clock source can only be changed by state transition.
    ///
    /// If the divider is equal to 0, baud rate clock is disabled.
    #[inline(always)]
    pub fn clock_divider(&self) -> u16 {
        self.internal_get_clock_divider()
    }

    /// Sets the clock divider.
    ///
    /// Clock divider is used for calculating baudrate.
    /// If you intend to get/set specific baudrate, instead of calculating it
    /// manually, use [`UART::baudrate`]/[`UART::set_baudrate`].
    ///
    /// Clock divider is defined as clock source frequency divided by
    /// (16*baudrate).
    ///
    /// # Safety
    /// If the divider is equal to 0, baud rate clock is disabled.
    /// Therefore, this function is unsafe, as it has potential, unwanted
    /// side-effect.
    #[inline(always)]
    pub unsafe fn set_clock_divider(&mut self, divider: u16) {
        self.internal_set_clock_divider(divider)
    }

    /// Waits until provided functor returns `true`. Functor receives UART status and should return
    /// the specific flag (or their combination).
    ///
    /// # Parameters
    /// * `status_checker` - Functor, returning flag or combination of status flags to wait for
    /// * `timeout_cycles` - Maximum amount of arbitrary "cycles" to spend on waiting for the flag.
    ///                      This is basically an amount of loop iterations with status checks.
    ///
    /// # Returns
    /// `Ok(u32)` if functor returned `true` before timeout, `Err(())` if timeout has been reached.
    /// The value returned on success indicates how much "cycles" are left for next timeout.
    pub(super) fn wait_for_status_flag<F>(
        &self,
        status_checker: F,
        timeout_cycles: u32,
    ) -> Result<u32, ()>
    where
        F: Fn(Status) -> bool,
    {
        for wasted_cycles in 0..timeout_cycles {
            if status_checker(self.status()) {
                return Ok(timeout_cycles - wasted_cycles);
            }
        }

        Err(())
    }
}
