//! Module containing channel configuration and status structures.

use crate::pac::tc0::tc_channel::cmr_waveform_mode::TCCLKSSELECT_A as PacClockId;

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

/// Enumeration listing available channel's clock sources.
#[derive(Debug, Default, Eq, PartialEq)]
pub enum ChannelClock {
    /// PCK6 (or PCK7 for TC0 Ch0, if configured in PMC) clock signal from PMC.
    /// Default per datasheet.
    #[default]
    PmcPeripheralClock,
    /// Host clock divided by 8
    MckDividedBy8,
    /// Host clock divided by 32
    MckDividedBy32,
    /// Host clock divided by 128
    MckDividedBy128,
    /// Slow clock (SLCK)
    SlowClock,
    /// External clock 0
    XC0,
    /// External clock 1
    XC1,
    /// External clock 2
    XC2,
    /// Timer peripheral clock (see PMC_PCR register in MCU manual)
    TimerPeripheralClock,
}

impl From<PacClockId> for ChannelClock {
    fn from(value: PacClockId) -> Self {
        match value {
            PacClockId::TIMER_CLOCK1 => ChannelClock::PmcPeripheralClock,
            PacClockId::TIMER_CLOCK2 => ChannelClock::MckDividedBy8,
            PacClockId::TIMER_CLOCK3 => ChannelClock::MckDividedBy32,
            PacClockId::TIMER_CLOCK4 => ChannelClock::MckDividedBy128,
            PacClockId::TIMER_CLOCK5 => ChannelClock::SlowClock,
            PacClockId::XC0 => ChannelClock::XC0,
            PacClockId::XC1 => ChannelClock::XC1,
            PacClockId::XC2 => ChannelClock::XC2,
        }
    }
}

/// Not all values from ChannelClock can be represented as PAC Clock ID.
impl TryFrom<ChannelClock> for PacClockId {
    type Error = ();

    fn try_from(value: ChannelClock) -> Result<Self, Self::Error> {
        match value {
            ChannelClock::PmcPeripheralClock => Ok(PacClockId::TIMER_CLOCK1),
            ChannelClock::MckDividedBy8 => Ok(PacClockId::TIMER_CLOCK2),
            ChannelClock::MckDividedBy32 => Ok(PacClockId::TIMER_CLOCK3),
            ChannelClock::MckDividedBy128 => Ok(PacClockId::TIMER_CLOCK4),
            ChannelClock::SlowClock => Ok(PacClockId::TIMER_CLOCK5),
            ChannelClock::XC0 => Ok(PacClockId::XC0),
            ChannelClock::XC1 => Ok(PacClockId::XC1),
            ChannelClock::XC2 => Ok(PacClockId::XC2),
            ChannelClock::TimerPeripheralClock => Err(()),
        }
    }
}
