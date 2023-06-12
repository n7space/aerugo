/// Handle to an event.
///
/// Handle is used in the system to access an event.
use crate::aerugo::error::RuntimeError;
use crate::event::Event;
use crate::time::MillisDurationU32;

/// Event handle.
#[derive(Copy, Clone)]
pub struct EventHandle {
    /// Reference to the event.
    _event: &'static Event,
}

impl EventHandle {
    /// Emits an immediate event.
    ///
    /// Returns `RuntimeError` in case of an error, `Ok(())` otherwise.
    #[inline(always)]
    pub fn emit(&self) -> Result<(), RuntimeError> {
        todo!()
    }

    /// Emits an event that is delayed in time
    ///
    /// * `event` - Event to emit.
    /// * `delay` - Delay amount.
    ///
    /// Returns `RuntimeError` in case of an error, `Ok(())` otherwise.
    #[inline(always)]
    pub fn emit_delayed(&self, _delay: MillisDurationU32) -> Result<(), RuntimeError> {
        todo!()
    }

    /// Cancels event.
    ///
    /// * `event` - Event to cancel.
    ///
    /// Returns `RuntimeError` in case of an error, `Ok(())` otherwise.
    #[inline(always)]
    pub fn cancel(&self) -> Result<(), RuntimeError> {
        todo!()
    }
}
