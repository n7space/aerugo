/// System runtime API.
///
/// This API can be used by the user in tasklet functions to interact with the system.
use crate::event::EventId;

/// System runtime API.
pub trait RuntimeApi: ErrorType {
    /// Emits an immediate event.
    ///
    /// * `event` - Event to emit.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn emit_event(&'static self, event: EventId) -> Result<(), Self::Error>;

    /// Emits an event that is delayed in time
    ///
    /// * `event` - Event to emit.
    /// * `delay` - Delay amount.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn emit_event_delayed(&'static self, event: EventId, delay: f64) -> Result<(), Self::Error>;

    /// Cancels event.
    ///
    /// * `event` - Event to cancel.
    ///
    /// Returns `Error` in case of an error, `Ok(())` otherwise.
    fn cancel_event(&'static self, event: EventId) -> Result<(), Self::Error>;

    /// Gets current system time timestamp.
    fn get_system_time(&'static self) -> f64;

    /// Sets system time offset.
    ///
    /// * `offset` - Time offset.
    fn set_system_time_offset(&'static self, offset: f64);
}

/// Runtime error
pub trait Error: core::fmt::Debug {}

/// Runtime error type trait
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}
