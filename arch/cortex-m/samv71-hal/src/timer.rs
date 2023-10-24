//! Implementation of HAL Timer Counter driver.
//!
//! This driver provides two primary structures - [`Timer`] and [`Channel`].
//! In typical scenario, you want to use PAC's TC instance (for example [`TC0`])
//! to create a [`Timer`], and then use [`Channel`]s provided by [`Timer`] by taking them from
//! it's instance.

pub mod channel;
pub mod channel_config;
pub mod channel_waveform;
pub mod timer_config;
pub mod timer_error;
pub mod waveform_config;

mod tc_metadata;

pub use channel::*;
pub use tc_metadata::*;
pub use timer_error::*;

use self::timer_config::{ExternalClock, ExternalClockSource};
use core::marker::PhantomData;

/// Structure representing a Timer instance.
///
/// # Generic Parameters
/// * `TimerMetadata` - PAC timer counter instance metadata, see `TcMetadata` private trait.
///
/// # Safety
/// Only a single instance of a [`Timer`] per physical timer should exist. Creating multiple instances
/// may lead to unexpected behaviors.
/// [`Channel`] instances should never be created manually, they should only be taken from
/// [`Timer`] instances.
///
/// This structure is not thread/interrupt-safe, as it uses shared state (registers).
/// If you need to share it, wrap it in a proper container that implements [`Sync`].
pub struct Timer<TimerMetadata> {
    /// Channel 0.
    pub channel_0: Option<Channel<TimerMetadata, Ch0, NotConfigured>>,
    /// Channel 1.
    pub channel_1: Option<Channel<TimerMetadata, Ch1, NotConfigured>>,
    /// Channel 2.
    pub channel_2: Option<Channel<TimerMetadata, Ch2, NotConfigured>>,
    /// PhantomData for TC metadata.
    _tc_peripheral: PhantomData<TimerMetadata>,
}

impl<Instance> Timer<Instance>
where
    Instance: TcMetadata,
{
    /// Creates a new timer instance from PAC timer structure.
    ///
    /// # Parameters
    /// * `_instance` - PAC Timer Counter instance, consumed on construction to prevent
    ///                 creation of duplicate Timer instances.
    pub fn new(_instance: Instance) -> Self {
        let tc = unsafe { &*Instance::REGISTERS };

        Self {
            channel_0: Some(Channel::new(&tc.tc_channel0)),
            channel_1: Some(Channel::new(&tc.tc_channel1)),
            channel_2: Some(Channel::new(&tc.tc_channel2)),

            _tc_peripheral: PhantomData,
        }
    }

    /// Triggers all channels, starting them if they are enabled.
    pub fn trigger_all_channels(&mut self) {
        self.registers_ref().bcr.write(|w| w.sync().set_bit());
    }

    /// Configures external clock signal source. This signal can be used as timer channel's input clock.
    ///
    /// # Parameters
    /// * `clock` - Selected external clock that will be changed.
    /// * `source` - External clock source that will be connected to selected clock.
    ///
    /// # Return
    /// `Ok(())` if configuration arguments are valid,
    /// `Err(TimerConfigurationError::InvalidClockSource)` if selected clock cannot be connected
    /// to selected source (consult MCU manual for details).
    ///
    /// # Safety
    /// This function directly modifies the registers of a timer in an unsafe manner, but values put in these
    /// registers come from PAC and are validated before using, so they should be valid.
    pub fn configure_external_clock_source(
        &mut self,
        clock: ExternalClock,
        source: ExternalClockSource,
    ) -> Result<(), TimerConfigurationError> {
        let reg = &self.registers_ref().bmr;
        let clock_source_id = match clock.id(source) {
            Some(id) => id,
            None => return Err(TimerConfigurationError::InvalidClockSource),
        };

        // SAFETY: `ExternalClockSource::id` will either return a valid clock source ID from PAC,
        // or None, which is handled above.
        match clock {
            ExternalClock::XC0 => reg.modify(|_, w| unsafe { w.tc0xc0s().bits(clock_source_id) }),
            ExternalClock::XC1 => reg.modify(|_, w| unsafe { w.tc1xc1s().bits(clock_source_id) }),
            ExternalClock::XC2 => reg.modify(|_, w| unsafe { w.tc2xc2s().bits(clock_source_id) }),
        }

        Ok(())
    }

    /// Returns a reference to Timer's registers.
    ///
    /// # Safety
    /// This function dereferences a raw pointer.
    /// It's safe to use, as long as there aren't multiple instances of [`Timer`] sharing the same registers,
    /// and existing instances of [`Timer`] are created only with [`new`](Timer::new()) method  
    fn registers_ref(&self) -> &RegisterBlock {
        unsafe { &*Instance::REGISTERS }
    }
}
