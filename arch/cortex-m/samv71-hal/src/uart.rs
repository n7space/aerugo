//! Implementation of Universal Asynchronous Receiver/Transmitter (UART) HAL driver.
//!
//! Before using UART, make sure to
//! - Enable UART peripheral clock using PMC driver
//! - Set appropriate pins mode to peripheral mode using PIO driver
//!
//! Consult the SAMV71 datasheet and [Safety](#safety) section for details.
//!
//! This driver allows you to configure and use any available UART peripheral in safe way.
//! Currently, the driver supports:
//! - Baudrate and clock source configuration
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
//! UART driver is implemented using typestate pattern. This approach prevents the
//! user from using UART in invalid configuration, while having no runtime cost
//! associated with error checking.
//! For details about usage, see [`Uart`] struct documentation.
//!
//! # Safety
//! **Make sure to read and understand this safety remark before switching UART to PCK
//! clock and/or trying to change MCU clocks while using UART**.
//!
//! UART can be driven by either peripheral or programmable clock.
//! If programmable clock (PCK4) is selected, the baud rate is independent of the
//! processor/bus clock, thus the processor clock can be changed while UART is enabled.
//! However, **only the baud rate clock is driven using PCK in that case, parts of UART
//! peripheral are still driven by peripheral clock, so UART is never fully independent
//! from peripheral clock**.
//! Only setting of processor clock that can be changed while UART is enabled is,
//! per reference manual, Main Clock (MCK) prescaler. Other methods to modify the
//! processor/bus clock frequency (PLL multiplier, etc.) are forbidden when UART is enabled.
//!
//! Therefore, UART must be de-initialized before changing the processor clock in any other
//! way, and re-initialized afterwards.
//!
//! **The peripheral clock frequency must be at least three times higher than PCK.**
//! In other words, PCK frequency can be equal to at most 1/3rd peripheral clock frequency.

extern crate embedded_io;

use core::marker::PhantomData;

use self::config::{bool_to_rx_filter_config, calculate_baudrate};
use self::reader::Reader;
use self::writer::Writer;

pub use embedded_io::ErrorKind as Error;
pub use embedded_io::{Read, ReadReady, Write, WriteReady};

pub use super::time::HertzU32 as Frequency;

pub mod config;
pub mod interrupt;
pub mod metadata;
pub mod reader;
pub mod states;
pub mod status;
pub mod writer;

pub use self::config::{ClockSource, Config, ParityBit, ReceiverConfig};
pub use self::interrupt::Interrupt;
pub use self::metadata::UARTMetadata;
pub use self::status::Status;

/// Constant representing oversampling ratio, which is used in baudrate and
/// clock divider calculations.
pub(super) const OVERSAMPLING_RATIO: u32 = 16;

/// Typestate trait representing generic UART state.
///
/// This is a super-trait for all UART states.
pub trait State {}

/// Typestate trait representing a configured UART
///
/// This is a super-trait for all UART states that allows free configuration of settings
/// independent from concrete state, for example baudrate, or clock source.
///
/// State-dependent configuration should be kept in more specific implementations.
pub trait Configured: State {}

/// Typestate trait representing UART with enabled receiver.
///
/// Concrete states in which UART can function as a receiver should implement it.
pub trait Receive: Configured {}

/// Typestate trait representing UART with enabled transmitter.
///
/// Concrete states in which UART can function as a transmitter should implement it.
pub trait Transmit: Configured {}

/// Typestate trait representing UART with enabled transmitter and receiver.
///
/// This is an utility trait, which is automatically implemented for all state
/// types implementing [`Receive`] and [`Transmit`].
pub trait ReceiveTransmit: Receive + Transmit {}

/// Typestate struct representing UART in not configured, usually post-reset state.
pub struct NotConfigured;

/// Typestate struct representing UART with enabled received
pub struct Receiver;

/// Typestate struct representing UART with enabled transmitter
pub struct Transmitter;

/// Typestate struct representing UART with enabled receiver and transmitter.
pub struct Bidirectional;

impl State for NotConfigured {}
impl State for Receiver {}
impl State for Transmitter {}
impl State for Bidirectional {}

impl Configured for Receiver {}
impl Configured for Transmitter {}
impl Configured for Bidirectional {}

impl Receive for Receiver {}
impl Receive for Bidirectional {}

impl Transmit for Transmitter {}
impl Transmit for Bidirectional {}

impl<T> ReceiveTransmit for T where T: Receive + Transmit {}

/// Structure representing UART driver.
///
/// This structure is implemented using typestate pattern.
/// In order to use it, you must first create it's instance with [`Uart::new`] method.
/// This method consumes PAC UART instance, which prevents from creating multiple UART
/// driver instances for the same UART peripheral (which would invalidate `Send` implementation
/// for this structure, so it should never be allowed).
///
/// [`Uart::new`] will return `Uart<_, NotConfigured>`, which you have to initialize by converting
/// it to one of three valid states: `Uart<_, Receiver>`, `Uart<_, Transmitter>` or
/// `Uart<_, Bidirectional>`. To do that, use `Uart::into_X` function, where `X` is the desired state.
/// See
/// * [`Uart::into_transmitter`],
/// * [`Uart::into_receiver`],
/// * [`Uart::into_bidirectional`], and
/// * [`Uart::disable`]
///
/// for details.
///
/// These functions expect some kind of config struct(s). [`Config`] is the generic configuration
/// for UART in any state, however, some state transitions may require additional configuration,
/// and if that's the case, additional configuration structure must be provided. RX filter configuration
/// is applicable only to UART in `Receiver` or `Bidirectional` state, so it's not present in [`Config`]
/// but rather in [`ReceiverConfig`] structure.
///
/// # Safety
/// **UART driver must be manually notified about source clock frequency changes.**
/// As described in module documentation, unless you drive UART using programmable clock,
/// **and** you're changing main clock frequency using it's prescaler, **you must de-initialize
/// UART using [`Uart::disable`] before changing clock settings**, and re-initialize it by
/// converting it into desired state after the clock is configured.
pub struct Uart<Instance: UARTMetadata, CurrentState: State> {
    /// Frequency of the clock driving UART baudrate.
    /// Required for baudrate calculations. Must be
    /// manually updated by the user after changing
    /// the clock source or it's frequency, otherwise
    /// UART will not work correctly.
    clock_source_frequency: Option<Frequency>,
    /// UART Reader instance.
    /// Can be taken using [`Uart::take_reader`] in Receiver mode.
    /// Can be put here after taking it using [`Uart::put_reader`] in Receiver mode.
    reader: Option<Reader<Instance>>,
    /// UART Writer instance.
    /// Can be taken using [`Uart::take_writer`] in Transmitter mode.
    /// Can be put here after taking it using [`Uart::put_writer`] in Transmitter mode.
    writer: Option<Writer<Instance>>,
    /// PAC UART instance metadata.
    _meta: PhantomData<Instance>,
    /// State metadata.
    _state: PhantomData<CurrentState>,
}

impl<Instance: UARTMetadata> Uart<Instance, NotConfigured> {
    /// Creates new UART driver instance, consuming PAC UART instance to prevent creating
    /// duplicate drivers.
    ///
    /// # Parameters
    /// * `uart` - PAC UARTx instance, where `x` is the number of UART peripheral.
    ///
    /// # Returns
    /// UART driver instance in `NotConfigured` state. It must be converted to valid
    /// state using `into_X` method to be usable.
    /// See
    /// * [`Uart::into_transmitter`],
    /// * [`Uart::into_receiver`],
    /// * [`Uart::into_bidirectional`], and
    /// * [`Uart::disable`]
    ///
    /// for details.
    pub fn new(_uart: Instance) -> Self {
        Self {
            clock_source_frequency: None,
            reader: Some(Reader::new()),
            writer: Some(Writer::new()),
            _meta: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Instance: UARTMetadata, AnyState: State> Uart<Instance, AnyState> {
    /// Transforms UART into `Transmitter` state. Resets UART status before
    /// changing the state. Disables loopback and RX filtering.
    ///
    /// # Parameters
    /// * `config` - Generic UART configuration.
    ///
    /// # Returns
    /// UART in `Transmitter` state, with only the transmission-related functions available.
    pub fn into_transmitter(mut self, config: Config) -> Uart<Instance, Transmitter> {
        self.disable_receiver();
        self.disable_transmitter();

        self.internal_reset_status();
        self.internal_set_config(config);
        self.enable_transmitter();

        Uart::transform(self)
    }

    /// Transforms UART into `Receiver` state. Resets UART status before
    /// changing the state. Disables loopback.
    ///
    /// # Parameters
    /// * `config` - Generic UART configuration.
    /// * `receiver_config` - UART configuration specific to receiver.
    ///
    /// # Returns
    /// UART in `Receiver` state, with only the reception-related functions available.
    pub fn into_receiver(
        mut self,
        config: Config,
        receiver_config: ReceiverConfig,
    ) -> Uart<Instance, Receiver> {
        self.disable_receiver();
        self.disable_transmitter();

        self.internal_reset_status();
        self.internal_set_config(config);
        self.internal_set_rx_filter_state(receiver_config.rx_filter_enabled);
        self.enable_receiver();

        Uart::transform(self)
    }

    /// Transforms UART into `Bidirectional` state. Resets UART status before
    /// changing the state. Disables loopback.
    ///
    /// # Parameters
    /// * `config` - Generic UART configuration.
    /// * `receiver_config` - UART configuration specific to receiver.
    ///
    /// # Returns
    /// UART in `Bidirectional` state, with both reception- and transmission-related functions
    /// available.
    pub fn into_bidirectional(
        mut self,
        config: Config,
        receiver_config: ReceiverConfig,
    ) -> Uart<Instance, Bidirectional> {
        self.disable_receiver();
        self.disable_transmitter();

        self.internal_reset_status();
        self.internal_set_config(config);
        self.internal_set_rx_filter_state(receiver_config.rx_filter_enabled);
        self.enable_transmitter();
        self.enable_receiver();

        Uart::transform(self)
    }

    /// Disables UART by disabling both receiver and transmitter, and stopping baudrate clock.
    ///
    /// Does not disable any interrupts. If you want to do that, use [`Uart::disable_all_interrupts`].
    ///
    /// # Returns
    /// UART in `NotConfigured` state.
    pub fn disable(mut self) -> Uart<Instance, NotConfigured> {
        self.disable_receiver();
        self.disable_transmitter();
        // Safety: This is intentional, as it disables baudrate clock.
        unsafe {
            self.internal_set_clock_divider(0);
        }
        self.internal_switch_to_normal_mode();
        self.internal_reset_status();

        Uart::transform(self)
    }

    /// Disables all UART interrupts.
    pub fn disable_all_interrupts(&mut self) {
        Instance::registers().idr.write(|w| {
            w.cmp()
                .set_bit()
                .txempty()
                .set_bit()
                .pare()
                .set_bit()
                .frame()
                .set_bit()
                .ovre()
                .set_bit()
                .txrdy()
                .set_bit()
                .rxrdy()
                .set_bit()
        });
    }

    /// Stores [`Reader`] instance inside UART.
    pub fn put_reader(&mut self, reader: Reader<Instance>) {
        self.reader.replace(reader);
    }

    /// Returns `true` if UART currently has [`Reader`] instance.
    pub fn has_reader(&self) -> bool {
        self.reader.is_some()
    }

    /// Stores [`Writer`] instance inside UART.
    pub fn put_writer(&mut self, writer: Writer<Instance>) {
        self.writer.replace(writer);
    }

    /// Returns `true` if UART currently has [`Writer`] instance.
    pub fn has_writer(&self) -> bool {
        self.writer.is_some()
    }

    /// Transforms UART into a type with different state.
    ///
    /// This is a helper function that reduces state transition boilerplate.
    ///
    /// # Parameters
    /// * `uart` - UART instance to be consumed and transformed.
    ///
    /// # Returns
    /// Transformed UART instance.
    const fn transform<NewState: State>(uart: Uart<Instance, NewState>) -> Self {
        Self {
            clock_source_frequency: uart.clock_source_frequency,
            reader: uart.reader,
            writer: uart.writer,
            _meta: PhantomData,
            _state: PhantomData,
        }
    }

    /// Enables UART receiver.
    ///
    /// The receiver is automatically enabled on conversion into `Receiver` or `Bidirectional`
    /// state, so this function is useful only if you've disabled it manually.
    ///
    /// This function is private, as it should be used only in state transition or loopback
    /// configuration code.
    pub(super) fn enable_receiver(&mut self) {
        Instance::registers().cr.write(|w| w.rxen().set_bit());
    }

    /// Disables UART receiver.
    ///
    /// If a byte is being processed, reception is completed before receiver is stopped.
    ///
    /// This function is private, as it should be used only in state transition or loopback
    /// configuration code.
    pub(super) fn disable_receiver(&mut self) {
        Instance::registers().cr.write(|w| w.rxdis().set_bit());
    }

    /// Enables UART transmitter.
    ///
    /// The transmitter is automatically enabled on conversion into `Transmitter` or `Bidirectional`
    /// state, so this function is useful only if you've disabled it manually.
    ///
    /// This function is private, as it should be used only in state transition or loopback
    /// configuration code.
    pub(super) fn enable_transmitter(&mut self) {
        Instance::registers().cr.write(|w| w.txen().set_bit());
    }

    /// Disables UART transmitter.
    ///
    /// If a byte is being processed, and a byte has been written to UART holding
    /// register, both bytes are transmitted before the transmitter is stopped.
    ///
    /// This function is private, as it should be used only in state transition or loopback
    /// configuration code.
    pub(super) fn disable_transmitter(&mut self) {
        Instance::registers().cr.write(|w| w.txdis().set_bit());
    }

    /// Switches UART into normal mode. This is the default mode of operation.
    ///
    /// This function is private - it might be re-exported (defined in public scope
    /// without prefix) in concrete state implementation, where it's safe to use.
    ///
    /// In this mode, transmitter is connected to TX line and receiver to RX line.
    pub(super) fn internal_switch_to_normal_mode(&mut self) {
        Instance::registers().mr.modify(|_, w| w.chmode().normal());
    }

    /// Returns current UART baudrate (in bits per second).
    ///
    /// This function is private - it might be re-exported (defined in public scope
    /// without prefix) in concrete state implementation, where it's safe to use.
    ///
    /// # Safety
    /// This function unwraps clock source frequency. It must not be called until source
    /// clock is configured.
    ///
    /// # Returns
    /// Current UART baudrate, or panics if clock source is not configured.
    unsafe fn internal_get_baudrate(&self) -> u32 {
        let divider = self.internal_get_clock_divider();
        calculate_baudrate(
            divider,
            self.clock_source_frequency.expect(
                "invalid UART state - baudrate tried to be read before setting clock source",
            ),
        )
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
    /// This function is private - it might be re-exported (defined in public scope
    /// without prefix) in private state implementation, where it's safe to use.
    ///
    /// # Returns
    /// UART clock divider. If the divider is equal to 0, baud rate clock is disabled.
    fn internal_get_clock_divider(&self) -> u16 {
        Instance::registers().brgr.read().cd().bits()
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
    /// This function is private - it might be re-exported (defined in public scope
    /// without prefix) in concrete state implementation, where it's safe to use.
    ///
    /// # Safety
    /// If the divider is equal to 0, baud rate clock is disabled.
    /// Therefore, this function is unsafe, as it has potential, unwanted
    /// side-effect.
    ///
    unsafe fn internal_set_clock_divider(&mut self, divider: u16) {
        Instance::registers()
            .brgr
            .write(|w| w.cd().variant(divider));
    }

    /// Sets the RX filtering state.
    ///
    /// This function is private - it might be re-exported (defined in public scope
    /// without prefix) in concrete state implementation, where it's safe to use.
    ///
    /// # Parameters
    /// * `enabled` - If `true`, RX filtering will be enabled.
    ///               If `false, RX filtering will be disabled.
    fn internal_set_rx_filter_state(&mut self, enabled: bool) {
        Instance::registers()
            .mr
            .modify(|_, w| w.filter().variant(bool_to_rx_filter_config(enabled)));
    }

    /// Resets UART status by clearing status flags.
    /// **This function should usually be called immediately after reading the status.**
    ///
    /// This function is private - it might be re-exported (defined in public scope
    /// without prefix) in concrete state implementation, where it's safe to use.
    fn internal_reset_status(&mut self) {
        Instance::registers().cr.write(|w| w.rststa().set_bit());
    }

    /// Configures UART using provided settings.
    /// It also disables loopback mode and RX filtering. These should
    /// be set using a dedicated, state-dependent function.
    ///
    /// This function is private - it might be re-exported (defined in public scope
    /// without prefix) in concrete state implementation, where it's safe to use.
    fn internal_set_config(&mut self, config: Config) {
        self.clock_source_frequency
            .replace(config.clock_source_frequency());
        // Disable baudrate clock before changing the configuration.
        // Safety: This is intentional. Setting divider to 0 disabled baudrate clock.
        unsafe {
            self.internal_set_clock_divider(0u16);
        }

        // Set source clock and parity bit config, disable loopback and RX filtering.
        Instance::registers().mr.write(|w| {
            w.chmode()
                .normal()
                .brsrcck()
                .variant(config.clock_source().into())
                .par()
                .variant(config.parity_bit().into())
                .filter()
                .disabled()
        });

        // Set the desired baudrate.
        // Safety: Validity of clock divider is guaranteed by `Config`.
        unsafe {
            self.internal_set_clock_divider(config.clock_divider());
        }
    }
}
