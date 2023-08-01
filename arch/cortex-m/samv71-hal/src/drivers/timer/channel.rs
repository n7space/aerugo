//! Module representing timer counter's channel
use core::marker::PhantomData;
use pac::tc0::tc_channel::TC_CHANNEL;

use super::TcMetadata;

/// Structure representing a timer's channel.
pub struct Channel<Timer, ID, State, Mode> {
    registers: *const TC_CHANNEL,
    _timer: PhantomData<Timer>,
    _id: PhantomData<ID>,
    _state: PhantomData<State>,
    _mode: PhantomData<Mode>,
}

/// Enumeration listing available channels.
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

/// Trait representing channel's state
pub trait ChannelState {}

/// Empty structure representing timer's disabled state.
pub struct Disabled;
/// Empty structure representing timer's enabled state.
pub struct Enabled;

impl ChannelState for Disabled {}
impl ChannelState for Enabled {}

/// Trait representing channel's mode
pub trait ChannelMode {}

/// Empty structure representing not configured channel.
pub struct NotConfigured;
/// Empty structure representing channel in Waveform mode.
pub struct WaveformMode;
/// Empty structure representing channel in Capture mode.
pub struct CaptureMode;

impl ChannelMode for NotConfigured {}
impl ChannelMode for WaveformMode {}
impl ChannelMode for CaptureMode {}

/// Channel implementation for all available states and modes
impl<Timer, ID, State, Mode> Channel<Timer, ID, State, Mode>
where
    Timer: TcMetadata,
    ID: ChannelId,
    State: ChannelState,
    Mode: ChannelMode,
{
    /// Returns current counter value read from channel's registers.
    ///
    /// # Implementation note
    /// CV register is 32-bit, but all timer counters of SAMV71 MCUs are 16-bit, therefore
    /// the returned value is casted to u16 to avoid confusion (or increase it, and make the user read MCU manual).
    pub fn counter_value(&self) -> u16 {
        self.registers_ref().cv.read().cv().bits() as u16
    }

    /// Returns current value of channel's `A` register.
    ///
    /// # Implementation note
    /// RA register is 32-bit, but all timer counters of SAMV71 MCUs are 16-bit, therefore
    /// the returned value is casted to u16 to avoid confusion (or increase it, and make the user read MCU manual).
    pub fn ra(&self) -> u16 {
        self.registers_ref().ra.read().ra().bits() as u16
    }

    /// Returns current value of channel's `B` register.
    ///
    /// # Implementation note
    /// RB register is 32-bit, but all timer counters of SAMV71 MCUs are 16-bit, therefore
    /// the returned value is casted to u16 to avoid confusion (or increase it, and make the user read MCU manual).
    pub fn rb(&self) -> u16 {
        self.registers_ref().rb.read().rb().bits() as u16
    }

    /// Returns current value of channel's `C` register.
    ///
    /// # Implementation note
    /// RC register is 32-bit, but all timer counters of SAMV71 MCUs are 16-bit, therefore
    /// the returned value is casted to u16 to avoid confusion (or increase it, and make the user read MCU manual).
    pub fn rc(&self) -> u16 {
        self.registers_ref().rc.read().rc().bits() as u16
    }

    /// Set the value of channel's `C` register.
    ///
    /// # Implementation note
    /// RC register is 32-bit, but all timer counters of SAMV71 MCUs are 16-bit, therefore
    /// this function accepts only u16 to avoid confusion (or increase it, and make the user read MCU manual).
    pub fn set_rc(&self, rc: u16) {
        self.registers_ref().rc.write(|w| w.rc().variant(rc as u32));
    }

    /// Returns a reference to Channel's registers.
    ///
    /// # Safety
    /// This function dereferences a raw pointer.
    /// It's safe to use as long as Channel is created only using provided [`new`](Channel::new()) method via [`Timer`](super::Timer) instance,
    /// as it guarantees that the pointer will be valid.
    fn registers_ref(&self) -> &TC_CHANNEL {
        unsafe { &*self.registers }
    }

    /// Transform the channel into a type with different state and/or mode.
    ///
    /// This is a helper function that allows to reduce state transition boilerplate to minimum.
    ///
    /// Rust compiler can deduce `Self` from function's return type, and transformation is basically a
    /// no-op, so no code should be generated from this. This function is only to signalize the compiler
    /// that we really want to change the type of an object.
    ///
    /// # Example
    /// ```no_run
    /// fn disable(self) -> Channel<Timer, ID, Disabled, Mode> {
    ///     // Do some logic to disable the timer here
    ///     // ...
    ///     
    ///     // Return channel with `State` changed to `Disabled`
    ///     // All generic arguments are deduced from this function's return type.
    ///     Channel::transform(self)
    /// }
    /// ```
    ///
    /// # Parameters
    /// * `channel` - Channel to be transformed.
    const fn transform<NewState, NewMode>(channel: Channel<Timer, ID, NewState, NewMode>) -> Self {
        Self {
            registers: channel.registers,
            _timer: PhantomData,
            _id: PhantomData,
            _state: PhantomData,
            _mode: PhantomData,
        }
    }
}

/// Channel implementation for disabled and not configured channels (default state).
impl<Timer, ID> Channel<Timer, ID, Disabled, NotConfigured>
where
    Timer: TcMetadata,
    ID: ChannelId,
{
    /// Create new timer channel.
    ///
    /// # Parameters
    /// * `channel` - Pointer to Timer Counter channel registers.
    ///
    /// # Safety
    /// This function should be called only by [`Timer`](super::Timer), not by the user.
    /// It's safe to use, as long as no duplicate channels (sharing the same registers) exist.
    pub(super) fn new(channel: *const TC_CHANNEL) -> Channel<Timer, ID, Disabled, NotConfigured> {
        Self {
            registers: channel,
            _timer: PhantomData,
            _id: PhantomData,
            _state: PhantomData,
            _mode: PhantomData,
        }
        .synced_with_hardware()
    }

    /// Resets the hardware state of the timer to correctly reflect it's typestate.
    /// In this state, the function will disable timer's channel.
    /// Our typestate should always be treated as "hard" guarantee, to which the hardware
    /// state of timer's channel is always synchronized.
    fn synced_with_hardware(self) -> Self {
        self.registers_ref().ccr.write(|w| w.clkdis().set_bit());
        self
    }
}

/// Channel implementation for disabled channels.
impl<Timer, ID, Mode> Channel<Timer, ID, Disabled, Mode>
where
    Timer: TcMetadata,
    ID: ChannelId,
    Mode: ChannelMode,
{
    /// Enables the channel.
    ///
    /// Consumes current instance and returns new one, with `Enabled` state.
    pub fn enable(self) -> Channel<Timer, ID, Enabled, Mode> {
        self.registers_ref().ccr.write(|w| w.clken().set_bit());
        Channel::transform(self)
    }
}

/// Channel implementation for enabled channels.
impl<Timer, ID, Mode> Channel<Timer, ID, Enabled, Mode>
where
    Timer: TcMetadata,
    ID: ChannelId,
    Mode: ChannelMode,
{
    /// Disables the channel.
    ///
    /// Consumes current instance and returns new one, with `Disabled` state.
    pub fn disable(self) -> Channel<Timer, ID, Disabled, Mode> {
        self.registers_ref().ccr.write(|w| w.clkdis().set_bit());
        Channel::transform(self)
    }

    /// Triggers the channel via software, starting it.
    ///
    /// Channel instance does not store the state indicating whether the timer is running or not,
    /// due to the fact that it can automatically stop without notifications, depending on configuration.
    pub fn trigger(&self) {
        self.registers_ref().ccr.write(|w| w.swtrg().set_bit());
    }
}
