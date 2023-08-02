//! Module containing channel configuration and status structures.

/// Structure representing available channel interrupts.
pub struct ChannelInterrupts {
    pub counter_overflow: bool,
    /// RA or RB have been loaded at least twice without any read since last time the status was read)
    pub load_overrun: bool,
    pub ra_compare: bool,
    pub rb_compare: bool,
    pub rc_compare: bool,
    pub ra_load: bool,
    pub rb_load: bool,
    pub external_trigger: bool,
}

/// Structure representing channel status register content.
pub struct ChannelStatus {
    /// Status of interrupts. Note that these flags are cleared when status register is read.
    pub interrupts: ChannelInterrupts,
    pub clock_enabled: bool,
    /// TIOA state - depending on config, it'll be either an internal signal, or a pin.
    pub tioa_state: bool,
    /// TIOB state - depending on config, it'll be either an internal signal, or a pin.
    pub tiob_state: bool,
}
