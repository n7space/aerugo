//! Module with structures and enumerations representing UART configuration.

use samv71q21_pac::uart0::mr::{BRSRCCKSELECT_A, CHMODESELECT_A, PARSELECT_A};

/// Structure representing UART configuration.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Config {
    /// Mode (normal/loopback).
    pub mode: Mode,
    /// Clock source.
    pub clock_source: ClockSource,
    /// Parity bit configuration.
    pub parity: ParityBit,
    /// If `true`, UART will filter the receive line using a three-sample
    /// filter (16x-bit clock, 2 over 3 majority).
    pub rx_filter_enabled: bool,
}

/// Enumeration representing UART channel modes.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    /// Normal mode, UART communicates using I/O pins.
    #[default]
    Normal,
    /// Automatic echo mode, RX data pin is internally connected to TX data pin.
    /// Receiver works normally, transmitter is disabled.
    AutomaticEcho,
    /// Local loopback mode, transmitter is internally connected to receiver and pins are disconnected.
    /// TX data pin is pulled to VDD in this mode.
    LocalLoopback,
    /// Remote loopback mode, RX data pin is connected to TX data pin.
    /// Receiver nad transmitter are disabled in this mode, and receiver is pulled to VDD.
    RemoteLoopback,
}

impl From<CHMODESELECT_A> for Mode {
    fn from(value: CHMODESELECT_A) -> Self {
        match value {
            CHMODESELECT_A::NORMAL => Mode::Normal,
            CHMODESELECT_A::AUTOMATIC => Mode::AutomaticEcho,
            CHMODESELECT_A::LOCAL_LOOPBACK => Mode::LocalLoopback,
            CHMODESELECT_A::REMOTE_LOOPBACK => Mode::RemoteLoopback,
        }
    }
}

impl From<Mode> for CHMODESELECT_A {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Normal => CHMODESELECT_A::NORMAL,
            Mode::AutomaticEcho => CHMODESELECT_A::AUTOMATIC,
            Mode::LocalLoopback => CHMODESELECT_A::LOCAL_LOOPBACK,
            Mode::RemoteLoopback => CHMODESELECT_A::REMOTE_LOOPBACK,
        }
    }
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
