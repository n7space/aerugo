use core::marker::PhantomData;

use rtt_target::{DownChannel, UpChannel};

/// Structure representing a streaming DownChannel.
/// Should never be constructed manually by the user.
pub struct DownStream {
    channel: DownChannel,
}

/// Structure representing a streaming UpChannel.
/// Should never be constructed manually by the user.
///
/// # Generic arguments
/// * `State` - Current state of the stream.
pub struct UpStream<State>
where
    State: StreamState,
{
    channel: UpChannel,
    _state: PhantomData<State>,
}

/// Trait representing stream's state.
pub trait StreamState {}

/// Structure representing stream in idle state.
pub struct Idle;
/// Structure representing stream in reception or transmission state.
pub struct Busy;

impl StreamState for Idle {}
impl StreamState for Busy {}

impl DownStream {
    /// Creates new idle stream from channel.
    pub(super) fn new(channel: DownChannel) -> Self {
        Self { channel }
    }
}

impl<State> UpStream<State>
where
    State: StreamState,
{
    /// Transforms the stream's type into different one.
    /// Destination type is usually deduced from the context.
    ///
    /// This is a helper function that minimizes the boilerplate of typestate pattern.
    const fn transform<NewState>(stream: UpStream<NewState>) -> Self
    where
        NewState: StreamState,
    {
        Self {
            channel: stream.channel,
            _state: PhantomData,
        }
    }
}

impl UpStream<Idle> {
    /// Creates new idle stream from channel.
    pub(super) fn new(channel: UpChannel) -> Self {
        Self {
            channel,
            _state: PhantomData,
        }
    }
}

impl UpStream<Busy> {}
