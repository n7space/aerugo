//! Module with structures and enumerations representing UART modes of operation.

/// Enumeration representing UART channel modes.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ChannelMode {
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

/// Enumeration representing UART source clock.
///
/// # Safety
/// Consult UART module documentation, or SAMV71 datasheet (section 46.5.1 "Baud Rate Generator")
/// if you intend to change processor's clocks frequency while UART is enabled.
///
/// The peripheral clock frequency must be at least three times higher than PCK frequency.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum SourceClock {
    /// UART is driven using standard peripheral clock.
    /// You must enable this clock via PMC before using UART.
    #[default]
    PeripheralClock,
    /// UART is driven using programmable clock (PCK4).
    /// You must enable and configure this clock via PMC before using UART.
    ProgrammableClock,
}
