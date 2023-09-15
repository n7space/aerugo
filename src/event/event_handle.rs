//! Handle to an event.
//!
//! This module contains event handle implementation, which can be used to reference an event in
//! the system.

use crate::error::RuntimeError;
use crate::event::Event;

/// Event handle.
///
/// Event handle is available to the user of the system to reference and interact with the event
/// via exposed interface.
#[derive(Copy, Clone)]
pub struct EventHandle {
    /// Reference to the event.
    event: &'static Event,
}

impl EventHandle {
    /// Creates new event handle.
    ///
    /// # Parameters
    /// * `event` - Reference to the event.
    pub(crate) fn new(event: &'static Event) -> Self {
        EventHandle { event }
    }

    /// Emits this event.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    #[inline(always)]
    pub fn emit(&self) -> Result<(), RuntimeError> {
        self.event.emit()
    }

    /// Cancels this event.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    #[inline(always)]
    pub fn cancel(&self) -> Result<(), RuntimeError> {
        self.event.cancel()
    }
}
