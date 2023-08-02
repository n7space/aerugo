//! Module containing channel configuration and status structures.

/// Structure representing available channel interrupts.
#[derive(Debug, Eq, PartialEq)]
pub struct ChannelInterrupts {
    /// Counter overflow
    pub counter_overflow: bool,
    /// RA or RB have been loaded at least twice without any read since last time the status was read)
    pub load_overrun: bool,
    /// RA Compare
    pub ra_compare: bool,
    /// RB Compare
    pub rb_compare: bool,
    /// RC Compare
    pub rc_compare: bool,
    /// RA Load
    pub ra_load: bool,
    /// RB Load
    pub rb_load: bool,
    /// External trigger
    pub external_trigger: bool,
}

impl ChannelInterrupts {
    /// Create a `ChannelInterrupts` structure with all interrupts set to `true`
    pub fn all() -> Self {
        Self {
            counter_overflow: true,
            load_overrun: true,
            ra_compare: true,
            rb_compare: true,
            rc_compare: true,
            ra_load: true,
            rb_load: true,
            external_trigger: true,
        }
    }

    /// Create a `ChannelInterrupts` structure with all interrupts set to `false`
    pub fn none() -> Self {
        Self {
            counter_overflow: false,
            load_overrun: false,
            ra_compare: false,
            rb_compare: false,
            rc_compare: false,
            ra_load: false,
            rb_load: false,
            external_trigger: false,
        }
    }
}

/// Explicit default, to explicitly indicate that default state is "all IRQs disabled"
impl Default for ChannelInterrupts {
    fn default() -> Self {
        ChannelInterrupts::none()
    }
}

/// Structure representing channel status register content.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ChannelStatus {
    /// Status of interrupts. Note that these flags are cleared when status register is read.
    pub interrupts: ChannelInterrupts,
    /// Clock is enabled
    pub clock_enabled: bool,
    /// TIOA state - depending on config, it'll be either an internal signal, or a pin.
    pub tioa_state: bool,
    /// TIOB state - depending on config, it'll be either an internal signal, or a pin.
    pub tiob_state: bool,
}
