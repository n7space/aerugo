//! Module containing channel configuration and status structures.

use pac::tc0::tc_channel::cmr_capture_mode::TCCLKSSELECT_A;

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
    /// PCK6 (or PCK7 for TC0 Ch0) clock signal from PMC
    #[default]
    PmcPeripheralClock,
    /// MCK divided by 8
    MckDividedBy8,
    /// MCK divided by 32
    MckDividedBy32,
    /// MCK divided by 128
    MckDividedBy128,
    /// Slow clock (SLCK)
    SlowClock,
    /// External clock 0
    XC0,
    /// External clock 1
    XC1,
    /// External clock 2
    XC2,
    /// Timer peripheral clock
    TimerPeripheralClock,
}

impl ChannelClock {
    /// Converts channel clock source to numeric ID representing it's value
    /// in Channel's mode configuration register.
    ///
    /// To prevent accidental typos, returned values are taken directly from PAC.
    /// This allows easy type erasure, while also retaining value safety.
    ///
    /// Not all values from this enumeration are meant to be written into CMR,
    /// hence an Option is returned.
    ///
    /// # Returns
    /// `Some(u8)` if current clock ID can be represented as value in CMR, otherwise None.
    pub(super) fn clock_id(self) -> Option<u8> {
        match self {
            ChannelClock::PmcPeripheralClock => Some(TCCLKSSELECT_A::TIMER_CLOCK1 as u8),
            ChannelClock::MckDividedBy8 => Some(TCCLKSSELECT_A::TIMER_CLOCK2 as u8),
            ChannelClock::MckDividedBy32 => Some(TCCLKSSELECT_A::TIMER_CLOCK3 as u8),
            ChannelClock::MckDividedBy128 => Some(TCCLKSSELECT_A::TIMER_CLOCK4 as u8),
            ChannelClock::SlowClock => Some(TCCLKSSELECT_A::TIMER_CLOCK5 as u8),
            ChannelClock::XC0 => Some(TCCLKSSELECT_A::XC0 as u8),
            ChannelClock::XC1 => Some(TCCLKSSELECT_A::XC1 as u8),
            ChannelClock::XC2 => Some(TCCLKSSELECT_A::XC2 as u8),
            ChannelClock::TimerPeripheralClock => None,
        }
    }
}
