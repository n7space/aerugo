//! Waveform-mode related configuration structures.

use crate::pac::tc0::tc_channel::cmr_waveform_mode::{
    ACPASELECT_A, EEVTEDGSELECT_A, EEVTSELECT_A, WAVSELSELECT_A,
};

/// Structure representing waveform mode configuration.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct WaveformModeConfig {
    /// RC Compare event effect on timer's counter state.
    pub rc_compare_effect: RcCompareEffect,
    /// External event configuration
    pub external_event: ExternalEventConfig,
    /// Waveform mode selection.
    pub mode: CountMode,
    /// Event effects for output A.
    pub tioa_effects: OutputSignalEffects,
    /// Event effects for output B.
    pub tiob_effects: OutputSignalEffects,
}

/// Enumeration representing RC compare event effect on channel's counter state.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum RcCompareEffect {
    /// RC Compare event has no effect on channel's counter state.
    /// Default per datasheet.
    #[default]
    None,
    /// RC Compare event stops channel's counter.
    CounterStops,
    /// RC Compare event disables channel's counter.
    CounterDisables,
    /// RC Compare event stops and disables channel's counter.
    CounterStopsAndDisables,
}

/// Intermediate helper struct used for explicit conversions between enum value and it's flags in TC registers.
/// To be used internally by HAL, not by the end user.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) struct RcCompareEffectFlags {
    /// Indicates that RC Compare stops the counter.
    pub stops: bool,
    /// Indicates that RC Compare disables the counter.
    pub disables: bool,
}

/// Helper conversion trait, used to parse the enum value into TC register flags.
impl From<RcCompareEffect> for RcCompareEffectFlags {
    fn from(value: RcCompareEffect) -> Self {
        match value {
            RcCompareEffect::None => Self {
                stops: false,
                disables: false,
            },
            RcCompareEffect::CounterStops => Self {
                stops: true,
                disables: false,
            },
            RcCompareEffect::CounterDisables => Self {
                stops: false,
                disables: true,
            },
            RcCompareEffect::CounterStopsAndDisables => Self {
                stops: true,
                disables: true,
            },
        }
    }
}

/// Helper conversion trait, used to parse data from TC registers into enum value.
impl From<RcCompareEffectFlags> for RcCompareEffect {
    fn from(value: RcCompareEffectFlags) -> Self {
        match value {
            RcCompareEffectFlags {
                stops: false,
                disables: false,
            } => RcCompareEffect::None,
            RcCompareEffectFlags {
                stops: true,
                disables: false,
            } => RcCompareEffect::CounterStops,
            RcCompareEffectFlags {
                stops: false,
                disables: true,
            } => RcCompareEffect::CounterDisables,
            RcCompareEffectFlags {
                stops: true,
                disables: true,
            } => RcCompareEffect::CounterStopsAndDisables,
        }
    }
}

/// Structure representing external event configuration for waveform mode.
///
/// Note: The selected external event only controls the TIOAx output and TIOBx
/// if not used as input (trigger event input, or other input).
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ExternalEventConfig {
    /// External event signal edge.
    pub edge: EventEdge,
    /// External event signal source.
    pub signal: ExternalEventSignal,
    /// Do external event trigger resets the counter and starts the counter clock?
    pub enabled: bool,
}

impl ExternalEventConfig {
    /// Returns disabled external event configuration.
    pub fn disabled() -> Self {
        ExternalEventConfig {
            edge: EventEdge::None,
            signal: ExternalEventSignal::TIOB,
            enabled: false,
        }
    }
}

/// Structure representing event effects on channel's output signals (TIOA and TIOB)
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct OutputSignalEffects {
    /// RA/RB comparison effect (RA for TIOA, RB for TIOB).
    pub rx_comparison: ComparisonEffect,
    /// RC comparison effect.
    pub rc_comparison: ComparisonEffect,
    /// External event effect.
    pub external_event: ComparisonEffect,
    /// Software trigger effect.
    pub software_trigger: ComparisonEffect,
}

impl OutputSignalEffects {
    /// Returns an [`OutputSignalEffects`] instance with all fields set to specific type of [effect](ComparisonEffect).
    pub fn all(effect: ComparisonEffect) -> Self {
        Self {
            rx_comparison: effect,
            rc_comparison: effect,
            external_event: effect,
            software_trigger: effect,
        }
    }

    /// Returns an [`OutputSignalEffects`] instance with all fields set to [`ComparisonEffect::None`]
    pub fn none() -> Self {
        Self::all(ComparisonEffect::None)
    }
}

/// Enumeration listing event signal edges.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum EventEdge {
    /// None
    /// Default per datasheet for all scenarios.
    #[default]
    None,
    /// Rising edge
    Rising,
    /// Falling edge
    Falling,
    /// Each edge
    Both,
}

impl From<EEVTEDGSELECT_A> for EventEdge {
    fn from(value: EEVTEDGSELECT_A) -> Self {
        match value {
            EEVTEDGSELECT_A::NONE => EventEdge::None,
            EEVTEDGSELECT_A::RISING => EventEdge::Rising,
            EEVTEDGSELECT_A::FALLING => EventEdge::Falling,
            EEVTEDGSELECT_A::EDGE => EventEdge::Both,
        }
    }
}

impl From<EventEdge> for EEVTEDGSELECT_A {
    fn from(value: EventEdge) -> Self {
        match value {
            EventEdge::None => EEVTEDGSELECT_A::NONE,
            EventEdge::Rising => EEVTEDGSELECT_A::RISING,
            EventEdge::Falling => EEVTEDGSELECT_A::FALLING,
            EventEdge::Both => EEVTEDGSELECT_A::EDGE,
        }
    }
}

/// Enumeration listing available external event signals.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ExternalEventSignal {
    /// Timer Output B (TIOB becomes input)
    ///
    /// Note: if TIOB is chosen as the external event signal, it is configured
    /// as an input and no longer generates waveforms, and subsequently, no interrupts.
    /// Default per datasheet.
    #[default]
    TIOB,
    /// External clock 0 (TIOB becomes output)
    XC0,
    /// External clock 1 (TIOB becomes output)
    XC1,
    /// External clock 2 (TIOB becomes output)
    XC2,
}

impl From<EEVTSELECT_A> for ExternalEventSignal {
    fn from(value: EEVTSELECT_A) -> Self {
        match value {
            EEVTSELECT_A::TIOB => ExternalEventSignal::TIOB,
            EEVTSELECT_A::XC0 => ExternalEventSignal::XC0,
            EEVTSELECT_A::XC1 => ExternalEventSignal::XC1,
            EEVTSELECT_A::XC2 => ExternalEventSignal::XC2,
        }
    }
}

impl From<ExternalEventSignal> for EEVTSELECT_A {
    fn from(value: ExternalEventSignal) -> Self {
        match value {
            ExternalEventSignal::TIOB => EEVTSELECT_A::TIOB,
            ExternalEventSignal::XC0 => EEVTSELECT_A::XC0,
            ExternalEventSignal::XC1 => EEVTSELECT_A::XC1,
            ExternalEventSignal::XC2 => EEVTSELECT_A::XC2,
        }
    }
}

/// Enumeration listing available waveform counting modes.
#[derive(Debug, Copy, Default, Clone, Eq, PartialEq)]
pub enum CountMode {
    /// Counter counts "up" until it's triggered or overflows, then it's reset to 0.
    /// Default per datasheet.
    #[default]
    Up,
    /// Counters counts "up" until it's triggered or overflows,
    /// then it counts "down" until it's triggered or reaches 0,
    UpDown,
    /// Counter counts "up" until it's triggered or reaches RC value, then it's reset to 0.
    UpToRc,
    /// Counters counts "up" until it's triggered or reaches RC value,
    /// then it counts "down" until it's triggered or reaches 0,
    UpDownToRc,
}

impl From<WAVSELSELECT_A> for CountMode {
    fn from(value: WAVSELSELECT_A) -> Self {
        match value {
            WAVSELSELECT_A::UP => CountMode::Up,
            WAVSELSELECT_A::UPDOWN => CountMode::UpDown,
            WAVSELSELECT_A::UP_RC => CountMode::UpToRc,
            WAVSELSELECT_A::UPDOWN_RC => CountMode::UpDownToRc,
        }
    }
}

impl From<CountMode> for WAVSELSELECT_A {
    fn from(value: CountMode) -> Self {
        match value {
            CountMode::Up => WAVSELSELECT_A::UP,
            CountMode::UpDown => WAVSELSELECT_A::UPDOWN,
            CountMode::UpToRc => WAVSELSELECT_A::UP_RC,
            CountMode::UpDownToRc => WAVSELSELECT_A::UPDOWN_RC,
        }
    }
}

/// Enumeration listing possible comparison effects.
#[derive(Debug, Copy, Default, Clone, Eq, PartialEq)]
pub enum ComparisonEffect {
    /// Comparison has no effect on signal.
    /// Default per datasheet.
    #[default]
    None,
    /// Comparison sets the signal.
    Set,
    /// Comparison clears the signal.
    Clear,
    /// Comparison toggles the signal.
    Toggle,
}

impl ComparisonEffect {
    /// Converts comparison effect to numeric ID representing it's value in
    /// channel's Waveform mode configuration register.
    ///
    /// Normally, [`From`] trait implementation would be used, but this enum values
    /// are used in multiple registers fields, while also having different types
    /// in PAC, so it's much less boilerplate to erase their type into u8.
    ///
    /// To prevent accidental typos, values are taken directly from PAC, and
    /// converted to u8. This allows easy type erasure, while also retaining
    /// value safety.
    pub(super) fn id(self) -> u8 {
        match self {
            ComparisonEffect::None => ACPASELECT_A::NONE as u8,
            ComparisonEffect::Set => ACPASELECT_A::SET as u8,
            ComparisonEffect::Clear => ACPASELECT_A::CLEAR as u8,
            ComparisonEffect::Toggle => ACPASELECT_A::TOGGLE as u8,
        }
    }

    /// Converts comparison effect's ID from TC registers into a ComparisonEffect
    /// instance.
    ///
    /// To prevent accidental typos, values are taken directly from PAC, and
    /// converted to u8. This allows easy type erasure, while also retaining
    /// value safety.
    ///
    /// # Returns
    /// A [`ComparisonEffect`], or [`None`] if an invalid ID is provided.
    pub(super) fn from_id(id: u8) -> Option<Self> {
        if id == (ACPASELECT_A::NONE as u8) {
            Some(ComparisonEffect::None)
        } else if id == (ACPASELECT_A::SET as u8) {
            Some(ComparisonEffect::Set)
        } else if id == (ACPASELECT_A::CLEAR as u8) {
            Some(ComparisonEffect::Clear)
        } else if id == (ACPASELECT_A::TOGGLE as u8) {
            Some(ComparisonEffect::Toggle)
        } else {
            None
        }
    }
}
