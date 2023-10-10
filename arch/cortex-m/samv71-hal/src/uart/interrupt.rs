//! Module with structures and enumerations representing UART interrupts.

/// Enumeration representing available UART interrupts.
pub enum Interrupt {
    /// Comparison interrupt, triggered on successful character comparison.
    Comparison,
    /// TX Empty interrupt, triggered when both TX holding register and internal
    /// shift registers become empty, effectively indicating end of transmission.
    TxEmpty,
    /// Parity error interrupt, triggered on invalid parity bit(s) detection.
    ParityError,
    /// Framing error interrupt, triggered when stop bit was not successfully detected.
    FramingError,
    /// Overrun error interrupt, triggered when a character is received while RX holding
    /// register is not empty.
    OverrunError,
    /// TX Ready interrupt, triggered when TX holding register becomes empty and ready for
    /// next character. **It does not mean that transmission has finished, which is indicated
    /// by [`TxEmpty`](Interrupt::TxEmpty) interrupt.**
    TxReady,
    /// RX Ready interrupt, triggered when a valid UART frame is received and it's value
    /// is moved to RX holding register.
    RxReady,
}
