//! Module with implementation of UART for all states implementing [`Configured`] typestate trait

use crate::uart::{
    config::{calculate_clock_divider, ConfigurationError, LoopbackMode},
    metadata::UARTMetadata,
    ClockSource, Config, Configured, Frequency, Interrupt, ParityBit, Status, Uart,
};

impl<Instance: UARTMetadata, State: Configured> Uart<Instance, State> {
    /// Returns current UART status.
    ///
    /// In order to clear reception error flags, you must use [`Reader`](crate::uart::Reader)
    /// object.
    pub fn status(&self) -> Status {
        Instance::registers().sr.read().into()
    }

    /// Returns current loopback mode of UART.
    ///
    /// By default, loopback is disabled (and therefore, this function
    /// should return [`LoopbackMode::None`]). Loopback modes can be
    /// enabled with dedicated functions, which are implemented for compatible
    /// UART states.
    ///
    /// See
    /// * [`Uart::switch_to_normal_mode`],
    /// * [`Uart::switch_to_automatic_echo_mode`],
    /// * [`Uart::switch_to_local_loopback_mode`], and
    /// * [`Uart::switch_to_remote_loopback_mode`]
    ///
    /// for details.
    pub fn loopback_mode(&self) -> LoopbackMode {
        Instance::registers().mr.read().chmode().variant().into()
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
        Instance::registers()
            .mr
            .modify(|_, w| w.chmode().remote_loopback());
    }

    /// Returns `true` if specified interrupt is currently enabled.
    pub fn is_interrupt_enabled(&self, interrupt: Interrupt) -> bool {
        let reg = Instance::registers().imr.read();

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
        Instance::registers().ier.write(|w| match interrupt {
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
        Instance::registers().idr.write(|w| match interrupt {
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
        let reg = Instance::registers().mr.read();

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
    /// Formula for baudrate is `(source clock frequency) / (16 * clock divider)`.
    /// **Calculated clock divider may not always precisely represent desired baudrate.**
    /// Make sure to choose source clock and baudrate values according to this formula
    /// to prevent issues with data transmission or reception. You can calculate the divider
    /// for any baudrate and clock using [`calculate_clock_divider`], and the baudrate for
    /// any clock and it's divider using [`calculate_baudrate`](crate::uart::config::calculate_baudrate). You can also check the
    /// real baudrate with [`Uart::baudrate`].
    ///
    /// # Parameters
    /// * `baudrate` - New baudrate to be set
    ///
    /// # Returns
    /// `Ok(())` if baudrate was set successfully, `Err(ConfigurationError)` if baudrate
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
        Instance::registers().mr.read().brsrcck().variant().into()
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
        Instance::registers()
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
        Instance::registers()
            .mr
            .modify(|_, w| w.par().variant(config.into()));
    }

    /// Returns current clock divider.
    ///
    /// Clock divider is used for calculating baudrate.
    /// If you intend to get/set specific baudrate, instead of calculating it
    /// manually, use [`Uart::baudrate`]/[`Uart::set_baudrate`].
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
    /// manually, use [`Uart::baudrate`]/[`Uart::set_baudrate`].
    ///
    /// Clock divider is defined as clock source frequency divided by
    /// (16*baudrate).
    ///
    /// Clock source can only be changed by state transition.
    ///
    /// # Safety
    /// If the divider is equal to 0, baud rate clock is disabled.
    /// Therefore, this function is unsafe, as it has potential, unwanted
    /// side-effect.
    #[inline(always)]
    pub unsafe fn set_clock_divider(&mut self, divider: u16) {
        self.internal_set_clock_divider(divider)
    }
}
