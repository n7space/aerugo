//! Handle to an event.
//!
//! This module contains event handle implementation, which is used to reference a event in the system.

use crate::aerugo::error::RuntimeError;
use crate::event::Event;
use crate::time::MillisDurationU32;

/// Event handle.
///
/// Event handle is available to the user of the system to reference and interact with the
/// event via exposed interface. All system API functions shall use handles when a reference to
/// event is required.
pub struct EventHandle {
    /// Reference to the event.
    _event: &'static Event,
}

impl EventHandle {
    /// Emits an immediate event.
    ///
    /// # Return
    /// `RuntimeError` in case of an error, `Ok(())` otherwise.
    #[inline(always)]
    pub fn emit(&self) -> Result<(), RuntimeError> {
        todo!()
    }

    /// Emits an event that is delayed in time
    ///
    /// # Parameters
    /// * `event` - Event to emit.
    /// * `delay` - Delay amount.
    ///
    /// # Return
    /// `RuntimeError` in case of an error, `Ok(())` otherwise.
    #[inline(always)]
    pub fn emit_delayed(&self, _delay: MillisDurationU32) -> Result<(), RuntimeError> {
        todo!()
    }

    /// Cancels event.
    ///
    /// # Parameters
    /// * `event` - Event to cancel.
    ///
    /// # Return
    /// `RuntimeError` in case of an error, `Ok(())` otherwise.
    #[inline(always)]
    pub fn cancel(&self) -> Result<(), RuntimeError> {
        todo!()
    }
}
