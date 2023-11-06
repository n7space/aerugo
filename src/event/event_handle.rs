//! Handle to an event.
//!
//! This module contains event handle implementation, which can be used to reference an event in
//! the system.

use crate::error::RuntimeError;
use crate::event::Event;
use crate::event_manager::EventManager;
use crate::time::Instant;

/// Event handle.
///
/// Event handle is available to the user of the system to reference and interact with the event
/// via exposed interface.
#[derive(Copy, Clone)]
pub struct EventHandle {
    /// Reference to the event.
    event: &'static Event,
    /// Reference to the event manager.
    event_manager: &'static EventManager,
}

impl EventHandle {
    /// Creates new event handle.
    ///
    /// # Parameters
    /// * `event` - Reference to the event.
    pub(crate) fn new(event: &'static Event, event_manager: &'static EventManager) -> Self {
        EventHandle {
            event,
            event_manager,
        }
    }

    /// Emits this event.
    #[inline(always)]
    pub fn emit(&self) {
        self.event.emit()
    }

    /// Schedules this event.
    ///
    /// # Parameters
    /// * `time` - Time since the scheduler start when event should be activated.
    #[inline(always)]
    pub fn schedule(&self, time: Instant) -> Result<bool, RuntimeError> {
        self.event_manager.schedule(self.event.id(), time)
    }

    /// Returns reference to the event.
    pub(crate) fn event(&self) -> &'static Event {
        self.event
    }
}
