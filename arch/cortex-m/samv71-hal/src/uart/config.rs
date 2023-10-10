//! Module with structures and enumerations representing UART configuration.

use samv71q21_pac::uart0::mr::{BRSRCCKSELECT_A, CHMODESELECT_A, FILTERSELECT_A, PARSELECT_A};

use super::{Frequency, OVERSAMPLING_RATIO};

/// Structure representing UART configuration.
///
/// Public members of this structure can be changed directly, or
/// via chained functions `with_X`, where `X` is the member name..
///
/// Private members can be accessed and modified only via dedicated
/// functions, as their values may depend on each other.
///
/// This structure makes sure that provided UART configuration is
/// always correct. It should not be possible to create [Config]
/// instance that contains invalid configuration. Linked fields are
/// always updated when one of them is being changed (for example,
/// clock divider is always updated when changing baudrate or source
/// clock frequency).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Config {
    /// Baudrate.
    baudrate: u32,
    /// clock source.
    clock_source: ClockSource,
    /// clock source frequency.
    clock_source_frequency: Frequency,
    /// Parity bit configuration.
    parity_bit: ParityBit,
    /// Baudrate clock divider.
    clock_divider: u16,
}

/// Structure representing additional UART settings, applicable only
/// for receiver or bidirectional mode.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ReceiverConfig {
    /// If `true`, UART will filter the receive line using a three-sample
    /// filter (16x-bit clock, 2 over 3 majority).
    pub rx_filter_enabled: bool,
}

/// Enumeration representing configuration error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ConfigurationError {
    /// Specified baudrate is too low, and it would result in clock divider
    /// larger than maximum possible value.
    BaudrateTooLow,
    /// Specified baudrate is too high, and it would cause the clock divider
    /// to be zero, effectively disabling baudrate clock.
    BaudrateTooHigh,
}

/// Enumeration representing UART clock source.
///
/// # Safety
/// Consult UART module documentation, or SAMV71 datasheet (section 46.5.1 "Baud Rate Generator")
/// if you intend to change processor's clocks frequency while UART is enabled.
///
/// The peripheral clock frequency must be at least three times higher than PCK frequency.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ClockSource {
    /// UART is driven using standard peripheral clock.
    /// You must enable this clock via PMC before using UART.
    #[default]
    PeripheralClock,
    /// UART is driven using programmable clock (PCK4).
    /// You must enable and configure this clock via PMC before using UART.
    ProgrammableClock,
}

/// Enumeration representing UART parity bit configuration.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ParityBit {
    /// Even parity
    Even,
    /// Odd parity
    Odd,
    /// Parity forced to 0
    Space,
    /// Parity forced to 1
    Mark,
    /// No parity bit
    #[default]
    None,
}

/// Enumeration representing available loopback modes.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LoopbackMode {
    /// Loopback disabled, UART operates normally
    None,
    /// RX line is internally connected to TX line.
    /// Transmitter is disconnected from TX line.
    AutomaticEcho,
    /// Transmitter is internally connected to receiver.
    /// RX and TX lines are disconnected from receiver and transmitter.
    /// TX line is pulled to Vdd.
    LocalLoopback,
    /// RX line is internally connected to TX line.
    /// Transmitter and receiver are disconnected from TX and RX lines.
    /// Receiver is pulled to Vdd.
    RemoteLoopback,
}

impl Config {
    /// Creates [`Config`] instance with provided baudrate and following defaults:
    /// * Clock source is peripheral clock (must be enabled via PMC)
    /// * No parity bit
    ///
    /// This function should be used to create new instance of [`Config`].
    ///
    /// To change other settings, use appropriate chained methods of [`Config`],
    /// or change their fields directly.
    ///
    /// Provided baudrate will be internally re-calculated after calculating clock
    /// divider for it, because clock divider might not be able to precisely
    /// scale the clock for it. If you need to know the precise baudrate, you can
    /// check it with [`Config::baudrate`] method.
    ///
    /// # Parameters
    /// * `baudrate` - UART Baudrate in bits per second.
    /// * `peripheral_clock_frequency` - Frequency of peripheral clock.
    ///
    /// # Returns
    /// Ok([Config]), or Err([ConfigurationError]) if provided configuration is invalid.
    pub fn new(
        baudrate: u32,
        peripheral_clock_frequency: Frequency,
    ) -> Result<Self, ConfigurationError> {
        let clock_divider = calculate_clock_divider(baudrate, peripheral_clock_frequency)?;
        // Clock divider changed, baudrate must be recalculated
        let baudrate = calculate_baudrate(clock_divider, peripheral_clock_frequency);

        Ok(Self {
            baudrate,
            clock_source: ClockSource::PeripheralClock,
            clock_source_frequency: peripheral_clock_frequency,
            parity_bit: ParityBit::None,
            clock_divider,
        })
    }

    /// Returns configured baudrate.
    pub fn baudrate(&self) -> u32 {
        self.baudrate
    }

    /// Consumes config and returns a new instance with specified baudrate.
    ///
    /// Provided baudrate will be internally re-calculated after calculating clock
    /// divider for it, because clock divider might not be able to precisely
    /// scale the clock for it. If you need to know the precise baudrate, you can
    /// check it with [`Config::baudrate`] method.
    ///
    /// Use this to chain-construct new config, or create modified instance of
    /// existing one, for example:
    /// ```
    /// # let old_config = Config::new(9600, 10_000_000.to_Hz());
    /// let config = old_config.with_baudrate(115200);
    /// ```
    ///
    /// You can chain multiple configuration methods.
    pub fn with_baudrate(self, baudrate: u32) -> Result<Self, ConfigurationError> {
        let clock_divider = calculate_clock_divider(baudrate, self.clock_source_frequency)?;
        // Clock divider changed, baudrate must be recalculated
        let baudrate = calculate_baudrate(clock_divider, self.clock_source_frequency);

        Ok(Self {
            baudrate,
            clock_divider,
            ..self
        })
    }

    /// Returns configured clock source.
    pub fn clock_source(&self) -> ClockSource {
        self.clock_source
    }

    /// Consumes config and returns a new instance with specified clock source.
    ///
    /// Use this to chain-construct new config, or create modified instance of
    /// existing one, for example:
    /// ```
    /// let config = Config::with_baudrate(9600, 12_000_000.to_Hz())
    ///              .with_clock_source(ClockSource::ProgrammableClock);
    /// ```
    ///
    /// You can chain multiple configuration methods.
    pub fn with_clock_source(self, clock_source: ClockSource) -> Self {
        Self {
            clock_source,
            ..self
        }
    }

    /// Returns configured frequency of clock source driving the baudrate.
    pub fn clock_source_frequency(&self) -> Frequency {
        self.clock_source_frequency
    }

    /// Consumes config and returns a new instance with specified clock source frequency.
    ///
    /// Original baudrate will be internally re-calculated after calculating clock
    /// divider for it, because clock divider might not be able to precisely
    /// scale the clock for it. If you need to know the precise baudrate, you can
    /// check it with [`Config::baudrate`] method.
    ///
    /// Use this to chain-construct new config, or create modified instance of
    /// existing one, for example:
    /// ```
    /// # let old_config = Config::new(9600, 10_000_000.to_Hz());
    /// let config = old_config.with_clock_source_frequency(12_000_000.to_Hz());
    /// ```
    ///
    /// You can chain multiple configuration methods.
    pub fn with_clock_source_frequency(
        self,
        clock_source_frequency: Frequency,
    ) -> Result<Self, ConfigurationError> {
        let clock_divider = calculate_clock_divider(self.baudrate, clock_source_frequency)?;
        // Clock divider changed, baudrate must be recalculated
        let baudrate = calculate_baudrate(clock_divider, clock_source_frequency);

        Ok(Self {
            baudrate,
            clock_source_frequency,
            clock_divider,
            ..self
        })
    }

    /// Returns configured parity bit
    pub fn parity_bit(&self) -> ParityBit {
        self.parity_bit
    }

    /// Consumes config and returns a new instance with specified parity bit.
    ///
    /// Use this to chain-construct new config, or create modified instance of
    /// existing one, for example:
    /// ```
    /// let config = Config::with_baudrate(9600, 12_000_000.to_Hz())
    ///              .with_parity_bit(ParityBit::Odd);
    /// ```
    ///
    /// You can chain multiple configuration methods.
    pub fn with_parity_bit(self, parity_bit: ParityBit) -> Self {
        Self { parity_bit, ..self }
    }

    /// Returns configured clock divider.
    ///
    /// Clock divider is automatically updated by [`Config::new`] and all the functions
    /// that modify baudrate or clock source frequency.
    pub fn clock_divider(&self) -> u16 {
        self.clock_divider
    }

    /// Consumes config and returns a new instance with specified clock source divider.
    ///
    /// Setting the clock divider automatically updates the baudrate stored in config.
    ///
    /// Use this to chain-construct new config, or create modified instance of
    /// existing one, for example:
    /// ```
    /// # let old_config = Config::new(9600, 10_000_000.to_Hz());
    /// let config = old_config.with_clock_divider(100);
    /// ```
    ///
    /// You can chain multiple configuration methods.
    ///
    /// # Safety
    /// Setting clock divider to `0` disables baudrate clock, which makes it a potentially
    /// unwanted side-effect.
    /// Therefore, this function is `unsafe`. Use it at your own peril. If you want to
    /// set or modify the baudrate, use baudrate-related functions which prevent UART from
    /// having it's baudrate clock disabled in this way.
    pub unsafe fn with_clock_divider(self, clock_divider: u16) -> Self {
        let baudrate = calculate_baudrate(clock_divider, self.clock_source_frequency);

        Self {
            baudrate,
            clock_divider,
            ..self
        }
    }
}

/// Validates provided baudrate and calculates clock divider.
///
/// If you intend to configure the UART, you should use [`Config`] or one
/// of the [`UART`](super::UART) methods instead, as they perform baudrate validation too.
///
/// This function should be used only if you want to validate UART baudrate
/// manually, as there's plenty of methods for baudrate configuration
/// implemented by UART driver that use this function underneath.
///
/// # Parameters
/// * `baudrate` - Baudrate to be validated.
/// * `baudrate_clock_frequency` - Frequency of the clock driving the baudrate.
///
/// # Returns
/// `Ok(u16)` with clock divider if provided baudrate is valid for provided clock
/// frequency, `Err(ConfigurationError)` otherwise.
pub const fn calculate_clock_divider(
    baudrate: u32,
    baudrate_clock_frequency: Frequency,
) -> Result<u16, ConfigurationError> {
    let divider = baudrate_clock_frequency.to_Hz() / (OVERSAMPLING_RATIO * baudrate);

    // If provided baudrate is larger than clock source frequency / 16, it will
    // cause the divider to be truncated to 0, which will disable the baudrate clock.
    // If you intend to disable the baudrate clock that way, set the divider to 0
    // directly, using `UART::set_clock_divider`.
    if divider == 0 {
        return Err(ConfigurationError::BaudrateTooHigh);
    }

    // If provided baudrate is small enough, it will cause the divider to be
    // larger than (2^16) - 1, which would cause an integer overflow.
    if divider > (u16::MAX as u32) {
        return Err(ConfigurationError::BaudrateTooLow);
    }

    // This cast is safe, because we validated that `divider` can be represented
    // as 16-bit unsigned.
    Ok(divider as u16)
}

/// Calculates and returns UART baudrate.
///
/// # Parameters
/// * `clock_divider` - Baudrate clock divider. **Cannot be 0**, as it will cause this function to panic.
/// * `baudrate_clock_frequency` - Frequency of the clock driving the baudrate.
///
/// # Returns
/// Baudrate in bits per second.
pub const fn calculate_baudrate(clock_divider: u16, baudrate_clock_frequency: Frequency) -> u32 {
    baudrate_clock_frequency.to_Hz() / (OVERSAMPLING_RATIO * (clock_divider as u32))
}

/// Converts RX filter configuration from PAC to boolean value.
#[inline(always)]
pub(super) const fn rx_filter_config_to_bool(config: FILTERSELECT_A) -> bool {
    match config {
        FILTERSELECT_A::DISABLED => false,
        FILTERSELECT_A::ENABLED => true,
    }
}

/// Converts boolean value into RX filter configuration from PAC.
#[inline(always)]
pub(super) const fn bool_to_rx_filter_config(filter_enabled: bool) -> FILTERSELECT_A {
    match filter_enabled {
        true => FILTERSELECT_A::ENABLED,
        false => FILTERSELECT_A::DISABLED,
    }
}

impl From<BRSRCCKSELECT_A> for ClockSource {
    fn from(value: BRSRCCKSELECT_A) -> Self {
        match value {
            BRSRCCKSELECT_A::PERIPH_CLK => ClockSource::PeripheralClock,
            BRSRCCKSELECT_A::PMC_PCK => ClockSource::ProgrammableClock,
        }
    }
}

impl From<ClockSource> for BRSRCCKSELECT_A {
    fn from(value: ClockSource) -> Self {
        match value {
            ClockSource::PeripheralClock => Self::PERIPH_CLK,
            ClockSource::ProgrammableClock => BRSRCCKSELECT_A::PMC_PCK,
        }
    }
}

impl From<PARSELECT_A> for ParityBit {
    fn from(value: PARSELECT_A) -> Self {
        match value {
            PARSELECT_A::EVEN => ParityBit::Even,
            PARSELECT_A::ODD => ParityBit::Odd,
            PARSELECT_A::SPACE => ParityBit::Space,
            PARSELECT_A::MARK => ParityBit::Mark,
            PARSELECT_A::NO => ParityBit::None,
        }
    }
}

impl From<ParityBit> for PARSELECT_A {
    fn from(value: ParityBit) -> Self {
        match value {
            ParityBit::Even => PARSELECT_A::EVEN,
            ParityBit::Odd => PARSELECT_A::ODD,
            ParityBit::Space => PARSELECT_A::SPACE,
            ParityBit::Mark => PARSELECT_A::MARK,
            ParityBit::None => PARSELECT_A::NO,
        }
    }
}

impl From<CHMODESELECT_A> for LoopbackMode {
    fn from(value: CHMODESELECT_A) -> Self {
        match value {
            CHMODESELECT_A::NORMAL => LoopbackMode::None,
            CHMODESELECT_A::AUTOMATIC => LoopbackMode::AutomaticEcho,
            CHMODESELECT_A::LOCAL_LOOPBACK => LoopbackMode::LocalLoopback,
            CHMODESELECT_A::REMOTE_LOOPBACK => LoopbackMode::RemoteLoopback,
        }
    }
}

impl From<LoopbackMode> for CHMODESELECT_A {
    fn from(value: LoopbackMode) -> Self {
        match value {
            LoopbackMode::None => CHMODESELECT_A::NORMAL,
            LoopbackMode::AutomaticEcho => CHMODESELECT_A::AUTOMATIC,
            LoopbackMode::LocalLoopback => CHMODESELECT_A::LOCAL_LOOPBACK,
            LoopbackMode::RemoteLoopback => CHMODESELECT_A::REMOTE_LOOPBACK,
        }
    }
}
