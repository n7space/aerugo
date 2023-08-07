//! Module with functionalities of timer's channel in waveform mode.

use pac::tc0::tc_channel::CMR_WAVEFORM_MODE;

use super::{
    waveform_config::{
        ComparisonEffect, CountMode, ExternalEventConfig, OutputSignalEffects, RcCompareEffect,
        RcCompareEffectFlags, WaveformModeConfig,
    },
    Channel, ChannelId, Disabled, TcMetadata, WaveformMode,
};

/// Channel implementation for Waveform mode while disabled.
impl<Timer, ID> Channel<Timer, ID, Disabled, WaveformMode>
where
    Timer: TcMetadata,
    ID: ChannelId,
{
    /// Sets waveform mode configuration.
    pub fn configure(&self, config: WaveformModeConfig) {
        let rc_event_flags: RcCompareEffectFlags = config.rc_compare_effect.into();

        self.registers_ref().cmr_waveform_mode().write(|w| {
            w.cpcstop()
                .variant(rc_event_flags.stops)
                .cpcdis()
                .variant(rc_event_flags.disables)
                .eevtedg()
                .variant(config.external_event.edge.into())
                .eevt()
                .variant(config.external_event.signal.into())
                .enetrg()
                .variant(config.external_event.enabled)
                .wavsel()
                .variant(config.mode.into())
                .wave()
                .set_bit()
                .acpa()
                .bits(config.tioa_effects.rx_comparison.id())
                .acpc()
                .bits(config.tioa_effects.rc_comparison.id())
                .aeevt()
                .bits(config.tioa_effects.external_event.id())
                .aswtrg()
                .bits(config.tioa_effects.software_trigger.id())
                .bcpb()
                .bits(config.tiob_effects.rx_comparison.id())
                .bcpc()
                .bits(config.tiob_effects.rc_comparison.id())
                .beevt()
                .bits(config.tiob_effects.external_event.id())
                .bswtrg()
                .bits(config.tiob_effects.software_trigger.id())
        });
    }

    /// Returns the effect of RC Compare event on channel's counter state.
    pub fn rc_compare_effect(&self) -> RcCompareEffect {
        let reg = self.mode_register().read();

        RcCompareEffectFlags {
            stops: reg.cpcstop().bit_is_set(),
            disables: reg.cpcdis().bit_is_set(),
        }
        .into()
    }

    /// Sets the effect of RC Compare event on channel's counter state.
    pub fn set_rc_compare_effect(&self, effect: RcCompareEffect) {
        let flags: RcCompareEffectFlags = effect.into();

        self.mode_register().modify(|_, w| {
            w.cpcstop()
                .variant(flags.stops)
                .cpcdis()
                .variant(flags.disables)
        });
    }

    /// Returns current external event configuration.
    pub fn external_event_config(&self) -> ExternalEventConfig {
        let reg = self.mode_register().read();

        ExternalEventConfig {
            edge: reg.eevtedg().variant().into(),
            signal: reg.eevt().variant().into(),
            enabled: reg.enetrg().bit_is_set(),
        }
    }

    /// Sets current external event configuration.
    pub fn set_external_event_config(&self, config: ExternalEventConfig) {
        self.mode_register().modify(|_, w| {
            w.eevtedg()
                .variant(config.edge.into())
                .eevt()
                .variant(config.signal.into())
                .enetrg()
                .variant(config.enabled)
        });
    }

    /// Returns current counting mode.
    pub fn count_mode(&self) -> CountMode {
        self.mode_register().read().wavsel().variant().into()
    }

    /// Sets current counting mode.
    pub fn set_count_mode(&self, mode: CountMode) {
        self.mode_register()
            .modify(|_, w| w.wavsel().variant(mode.into()));
    }

    /// Sets TIOA event/trigger effects.
    ///
    /// # Safety
    /// This function will panic if an unexpected value is read from timer's registers.
    /// If this happens, that means the PAC is broken and there's nothing that can be done on
    /// user side to avoid it, as that kind of situation should never happen on correctly working
    /// hardware. See [`ComparisonEffect::from_id`](ComparisonEffect::from_id) for details about
    /// value conversion.
    pub fn tioa_effects(&self) -> OutputSignalEffects {
        let reg = self.mode_register().read();
        let panic_message = "invalid comparison effect ID read from TC registers";

        OutputSignalEffects {
            rx_comparison: ComparisonEffect::from_id(reg.acpa().variant() as u8)
                .expect(panic_message),
            rc_comparison: ComparisonEffect::from_id(reg.acpc().variant() as u8)
                .expect(panic_message),
            external_event: ComparisonEffect::from_id(reg.aeevt().variant() as u8)
                .expect(panic_message),
            software_trigger: ComparisonEffect::from_id(reg.aswtrg().variant() as u8)
                .expect(panic_message),
        }
    }

    /// Returns TIOA event/trigger effects.
    pub fn set_tioa_effects(&self, effects: OutputSignalEffects) {
        self.mode_register().modify(|_, w| {
            w.acpa()
                .bits(effects.rx_comparison.id())
                .acpc()
                .bits(effects.rc_comparison.id())
                .aeevt()
                .bits(effects.external_event.id())
                .aswtrg()
                .bits(effects.software_trigger.id())
        });
    }

    /// Sets TIOB event/trigger effects.
    ///
    /// # Safety
    /// This function will panic if an unexpected value is read from timer's registers.
    /// If this happens, that means the PAC is broken and there's nothing that can be done on
    /// user side to avoid it, as that kind of situation should never happen on correctly working
    /// hardware. See [`ComparisonEffect::from_id`](ComparisonEffect::from_id) for details about
    /// value conversion.
    pub fn tiob_effects(&self) -> OutputSignalEffects {
        let reg = self.mode_register().read();
        let panic_message = "invalid comparison effect ID read from TC registers";

        OutputSignalEffects {
            rx_comparison: ComparisonEffect::from_id(reg.bcpb().variant() as u8)
                .expect(panic_message),
            rc_comparison: ComparisonEffect::from_id(reg.bcpc().variant() as u8)
                .expect(panic_message),
            external_event: ComparisonEffect::from_id(reg.beevt().variant() as u8)
                .expect(panic_message),
            software_trigger: ComparisonEffect::from_id(reg.bswtrg().variant() as u8)
                .expect(panic_message),
        }
    }

    /// Returns TIOB event/trigger effects.
    pub fn set_tiob_effects(&self, effects: OutputSignalEffects) {
        self.mode_register().modify(|_, w| {
            w.bcpb()
                .bits(effects.rx_comparison.id())
                .bcpc()
                .bits(effects.rc_comparison.id())
                .beevt()
                .bits(effects.external_event.id())
                .bswtrg()
                .bits(effects.software_trigger.id())
        });
    }

    /// Returns a reference to channel mode register.
    fn mode_register(&self) -> &CMR_WAVEFORM_MODE {
        self.registers_ref().cmr_waveform_mode()
    }
}
