//! Module representing timer counter's channel
use core::marker::PhantomData;
use pac::tc0::tc_channel::TC_CHANNEL;

/// Structure representing a timer.
pub struct Channel<Timer, ID, State, Mode> {
    registers: *const TC_CHANNEL,
    _timer: PhantomData<Timer>,
    _id: PhantomData<ID>,
    _state: PhantomData<State>,
    _mode: PhantomData<Mode>,
}

/// Trait representing channel's ID
pub trait ChannelId {
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

impl<Timer, ID, State, Mode> Channel<Timer, ID, State, Mode> {
    /// Returns a reference to Channel's registers.
    ///
    /// # Safety
    /// This function dereferences a raw pointer.
    /// It's safe to use as long as Channel is created only using provided [`new`](Channel::new()) method via [`Timer`](super::Timer) instance,
    /// as it guarantees that the pointer will be valid.
    fn registers_ref(&self) -> &TC_CHANNEL {
        unsafe { &*self.registers }
    }
}

impl<Timer, ID> Channel<Timer, ID, Disabled, NotConfigured>
where
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
