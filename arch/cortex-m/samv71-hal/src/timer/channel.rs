//! Module representing timer counter's channel

use crate::pac::tc0::tc_channel::TC_CHANNEL;
use core::marker::PhantomData;

use super::channel_config::*;
use super::waveform_config::WaveformModeConfig;
use super::TcMetadata;

/// Structure representing a timer's channel.
///
/// This structure uses a typestate pattern, which means that in order to use it's instance
/// you must convert it to a proper type using `into_*_channel`. Currently, only Waveform
/// mode is supported. See [`Channel::into_waveform_channel`](Channel::into_waveform_channel) for more details.
///
/// # Safety
/// [`Channel`] instances should never be created manually, they should only be taken
/// from [`Timer`](crate::timer::Timer) instances.
///
/// This structure is not thread/interrupt-safe, as it uses shared state (registers).
/// If you need to share it, wrap it in a proper container that implements [`Sync`].
pub struct Channel<Timer, ID, Mode> {
    /// Timer channel's registers.
    registers: *const TC_CHANNEL,
    /// PhantomData for Timer type.
    _timer: PhantomData<Timer>,
    /// PhantomData for ID type.
    _id: PhantomData<ID>,
    /// PhantomData for Mode type.
    _mode: PhantomData<Mode>,
}

/// Assuming that the user does not create an instance of channel by himself, and instead relies on
/// instances provided by HAL, it's safe to send channels to other threads, as there's only a single
/// instance that can access hardware channel's registers at once, and it cannot be copied.
///
/// Sharing references ([`Sync`]) to a channel between threads is not safe, and should be managed by the user.
///
/// If that invariant is broken by the user, any usage of cloned Channels from other thread's context (including
/// interrupt context) can be considered unsafe.
unsafe impl<Timer, ID, Mode> Send for Channel<Timer, ID, Mode> {}

/// Enumeration listing available channels.
///
/// It's value-level equivalent of ChannelId trait.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChannelNo {
    /// Channel 0.
    Ch0 = 0,
    /// Channel 1.
    Ch1 = 1,
    /// Channel 2.
    Ch2 = 2,
}

/// Trait representing channel's ID
///
/// It's type-level equivalent of ChannelNo enumeration.
pub trait ChannelId {
    /// Numeric value od channel ID.
    const ID: usize;
}

/// Empty structure representing Channel 0 ID
pub struct Ch0;
/// Empty structure representing Channel 1 ID
pub struct Ch1;
/// Empty structure representing Channel 2 ID
pub struct Ch2;

impl ChannelId for Ch0 {
    const ID: usize = 0;
}

impl ChannelId for Ch1 {
    const ID: usize = 1;
}

impl ChannelId for Ch2 {
    const ID: usize = 2;
}

/// Trait representing channel's mode
pub trait ChannelMode {}

/// Empty structure representing not configured channel.
pub struct NotConfigured;
/// Empty structure representing channel in Waveform mode.
pub struct Waveform;
/// Empty structure representing channel in Capture mode.
pub struct Capture;

impl ChannelMode for NotConfigured {}
impl ChannelMode for Waveform {}
impl ChannelMode for Capture {}

/// Channel implementation for all available modes.
impl<Timer, ID, Mode> Channel<Timer, ID, Mode>
where
    Timer: TcMetadata,
    ID: ChannelId,
    Mode: ChannelMode,
{
    /// Sets the clock source used by channel.
    ///
    /// # Parameters
    /// * `clock` - New clock source for current channel.
    pub fn set_clock_source(&self, clock: ChannelClock) {
        match clock {
            // Timer peripheral clock setting is configured via different register
            // than the rest of the clocks.
            ChannelClock::TimerPeripheralClock => {
                self.registers_ref()
                    .emr
                    .modify(|_, w| w.nodivclk().set_bit());
            }
            clock => {
                // If an error happens here, it's a hard bug in HAL, there's no way for the user
                // to handle this as it should be impossible to fail here. Hence, we panic.
                let clock_id = clock.try_into().expect(
                    "internal HAL error - invalid clock tried to be converted into clock ID",
                );

                // This field is the same in Capture and Waveform mode. Hence, it can be changed in either.
                self.registers_ref()
                    .cmr_waveform_mode()
                    .modify(|_, w| w.tcclks().variant(clock_id))
            }
        }
    }

    /// Returns currently used clock source.
    pub fn clock_source(&self) -> ChannelClock {
        let is_timer_peripheral_clock_used =
            self.registers_ref().emr.read().nodivclk().bit_is_set();

        // This setting overrides clock configuration from CMR register.
        if is_timer_peripheral_clock_used {
            return ChannelClock::TimerPeripheralClock;
        }

        self.registers_ref()
            .cmr_waveform_mode()
            .read()
            .tcclks()
            .variant()
            .into()
    }

    /// Returns current counter value from channel's register.
    ///
    /// # Implementation notes
    /// CV register is 32-bit, but all timer counters of SAMV71 MCUs are 16-bit, therefore
    /// the returned value is casted to u16 to avoid confusion (or increase it, and make the user read MCU manual).
    pub fn counter_value(&self) -> u16 {
        self.registers_ref().cv.read().cv().bits() as u16
    }

    /// Returns current value of channel's `A` register.
    ///
    /// # Implementation notes
    /// RA register is 32-bit, but all timer counters of SAMV71 MCUs are 16-bit, therefore
    /// the returned value is casted to u16 to avoid confusion (or increase it, and make the user read MCU manual).
    pub fn ra(&self) -> u16 {
        self.registers_ref().ra.read().ra().bits() as u16
    }

    /// Returns current value of channel's `B` register.
    ///
    /// # Implementation notes
    /// RB register is 32-bit, but all timer counters of SAMV71 MCUs are 16-bit, therefore
    /// the returned value is casted to u16 to avoid confusion (or increase it, and make the user read MCU manual).
    pub fn rb(&self) -> u16 {
        self.registers_ref().rb.read().rb().bits() as u16
    }

    /// Returns current value of channel's `C` register.
    ///
    /// # Implementation notes
    /// RC register is 32-bit, but all timer counters of SAMV71 MCUs are 16-bit, therefore
    /// the returned value is casted to u16 to avoid confusion (or increase it, and make the user read MCU manual).
    pub fn rc(&self) -> u16 {
        self.registers_ref().rc.read().rc().bits() as u16
    }

    /// Sets the value of channel's `C` register.
    ///
    /// # Implementation notes
    /// RC register is 32-bit, but all timer counters of SAMV71 MCUs are 16-bit, therefore
    /// this function accepts only u16 to avoid confusion (or increase it, and make the user read MCU manual).
    pub fn set_rc(&self, rc: u16) {
        self.registers_ref().rc.write(|w| w.rc().variant(rc as u32));
    }

    /// Reads channel's status register, and clean interrupt status flags after that.
    ///
    /// # Safety
    /// **Reading status register will clear interrupt status flags**. If you are using interrupts,
    /// make sure to be careful with this function in critical sections - if a timer interrupt will happen
    /// during critical section, and you'll read the status flag, the interrupt handler that will execute after
    /// critical section has ended will not know about events that happened between critical section start and
    /// reading status register. Same scenario can happen in interrupt handlers, if timer interrupt has lower priority
    /// than currently handled interrupt.
    ///
    /// For this reason, this function is marked as mutable (it does not mutate the structure, but it might mutate
    /// status register).
    pub fn read_and_clear_status(&mut self) -> ChannelStatus {
        let sr = self.registers_ref().sr.read();

        ChannelStatus {
            interrupts: ChannelInterrupts {
                counter_overflow: sr.covfs().bit(),
                load_overrun: sr.lovrs().bit(),
                ra_compare: sr.cpas().bit(),
                rb_compare: sr.cpbs().bit(),
                rc_compare: sr.cpcs().bit(),
                ra_load: sr.ldras().bit(),
                rb_load: sr.ldrbs().bit(),
                external_trigger: sr.etrgs().bit(),
            },
            clock_enabled: sr.clksta().bit(),
            tioa_state: sr.mtioa().bit(),
            tiob_state: sr.mtiob().bit(),
        }
    }

    /// Enables selected interrupts.
    ///
    /// State of other interrupts will not be changed.
    ///
    /// # Parameters
    /// * `interrupts` - Structure with interrupts to be enabled. All interrupts set
    ///                  to `true` will be enabled.
    pub fn enable_interrupts(&self, interrupts: ChannelInterrupts) {
        self.registers_ref().ier.write(|w| {
            w.covfs()
                .variant(interrupts.counter_overflow)
                .lovrs()
                .variant(interrupts.load_overrun)
                .cpas()
                .variant(interrupts.ra_compare)
                .cpbs()
                .variant(interrupts.rb_compare)
                .cpcs()
                .variant(interrupts.rc_compare)
                .ldras()
                .variant(interrupts.ra_load)
                .ldrbs()
                .variant(interrupts.rb_load)
                .etrgs()
                .variant(interrupts.external_trigger)
        });
    }

    /// Disables selected interrupts.
    ///
    /// State of other interrupts will not be changed.
    ///
    /// # Parameters
    /// * `interrupts` - Structure with interrupts to be disabled. All interrupts set
    ///                  to `true` will be disabled.
    pub fn disable_interrupts(&self, interrupts: ChannelInterrupts) {
        self.registers_ref().idr.write(|w| {
            w.covfs()
                .variant(interrupts.counter_overflow)
                .lovrs()
                .variant(interrupts.load_overrun)
                .cpas()
                .variant(interrupts.ra_compare)
                .cpbs()
                .variant(interrupts.rb_compare)
                .cpcs()
                .variant(interrupts.rc_compare)
                .ldras()
                .variant(interrupts.ra_load)
                .ldrbs()
                .variant(interrupts.rb_load)
                .etrgs()
                .variant(interrupts.external_trigger)
        });
    }

    /// Returns the status (enabled/disabled) of channel's interrupts.
    pub fn interrupts_masks(&self) -> ChannelInterrupts {
        let masks = self.registers_ref().imr.read();

        ChannelInterrupts {
            counter_overflow: masks.covfs().bit(),
            load_overrun: masks.lovrs().bit(),
            ra_compare: masks.cpas().bit(),
            rb_compare: masks.cpbs().bit(),
            rc_compare: masks.cpcs().bit(),
            ra_load: masks.ldras().bit(),
            rb_load: masks.ldrbs().bit(),
            external_trigger: masks.etrgs().bit(),
        }
    }

    /// Returns a reference to Channel's registers.
    ///
    /// # Safety
    /// This function dereferences a raw pointer.
    /// It's safe to use as long as Channel is created only using provided [`new`](Channel::new()) method via [`Timer`](super::Timer) instance,
    /// as it guarantees that the pointer will be valid.
    pub(super) fn registers_ref(&self) -> &TC_CHANNEL {
        unsafe { &*self.registers }
    }

    /// Transforms the channel into a type with different mode.
    ///
    /// This is a helper function that allows to reduce state transition boilerplate to minimum.
    ///
    /// Rust compiler can deduce `Self` from function's return type, and transformation is basically a
    /// no-op, so no code should be generated from this. This function is only to signalize the compiler
    /// that we really want to change the type of an object.
    ///
    /// # Example
    /// ```ignore
    /// fn waveform(self) -> Channel<Timer, ID, Waveform> {
    ///     // Do some logic to configure the new mode here
    ///     // ...
    ///     
    ///     // Return channel with `Mode` changed to `Waveform`
    ///     // All generic parameters are deduced from this function's return type.
    ///     Channel::transform(self)
    /// }
    /// ```
    ///
    /// # Parameters
    /// * `channel` - Channel to be transformed.
    const fn transform<NewMode>(channel: Channel<Timer, ID, NewMode>) -> Self {
        Self {
            registers: channel.registers,
            _timer: PhantomData,
            _id: PhantomData,
            _mode: PhantomData,
        }
    }
}

/// Channel implementation for not configured channels (default state).
impl<Timer, ID> Channel<Timer, ID, NotConfigured>
where
    Timer: TcMetadata,
    ID: ChannelId,
{
    /// Creates new timer channel.
    ///
    /// # Parameters
    /// * `channel` - Pointer to Timer Counter channel registers.
    ///
    /// # Safety
    /// This function should be called only by [`Timer`](super::Timer), not by the user.
    /// It's safe to use, as long as no duplicate channels (sharing the same registers) exist.
    pub(super) fn new(channel: *const TC_CHANNEL) -> Channel<Timer, ID, NotConfigured> {
        Self {
            registers: channel,
            _timer: PhantomData,
            _id: PhantomData,
            _mode: PhantomData,
        }
        .synced_with_hardware()
    }

    /// Resets the hardware state of the timer to correctly reflect it's typestate.
    ///
    /// In this state, the function will disable timer's channel.
    /// Our typestate should always be treated as "hard" guarantee, to which the hardware
    /// state of timer's channel is always synchronized.
    fn synced_with_hardware(self) -> Self {
        self.registers_ref().ccr.write(|w| w.clkdis().set_bit());
        self
    }
}

/// Channel implementation for channels in all modes.
impl<Timer, ID, Mode> Channel<Timer, ID, Mode>
where
    Timer: TcMetadata,
    ID: ChannelId,
    Mode: ChannelMode,
{
    /// Changes channel's mode to Waveform mode.
    ///
    /// Consumes current instance and returns new one, in `Waveform` mode.
    ///
    /// # Parameters
    /// * `config` - Waveform mode configuration. Can be changed later.
    pub fn into_waveform_channel(self, config: WaveformModeConfig) -> Channel<Timer, ID, Waveform> {
        let transformed_channel = Channel::transform(self);
        transformed_channel.configure(config);
        transformed_channel
    }
}
