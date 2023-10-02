//! Module with structures and enumerations representing UART status.

/// Structure representing UART status.
pub struct Status {
    /// `true` if received character matches the programmed comparison criteria.
    pub comparison_matched: bool,
    /// `true` if both internal shift register and TX holding registers are empty.
    /// In other words, this indicates that transmitter is idle.
    pub transmitter_empty: bool,
    /// `true` if invalid parity bit(s) was/were detected.
    pub parity_error: bool,
    /// `true` if stop bit was not successfully detected.
    pub framing_error: bool,
    /// `true` if a character is received while RX holding register isn't empty.
    pub overrun_error: bool,
    /// `true` if TX holding register is empty. It does not indicate that transmitter
    /// is idle, as it may still process data from it's internal shift register,
    /// see [`Status::transmitter_empty`].
    pub transmitter_ready: bool,
    /// `true` if RX holding register contains a complete character, ready to be read.
    pub receiver_ready: bool,
}
