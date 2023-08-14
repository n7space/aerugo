//! System time manager.
//!
//! This module contains a system event manager. It stores events created in the system and manages
//! event sets that are assigned to the tasklets.

use env_parser::read_env;
use heapless::Vec;
use internal_cell::InternalCell;

use crate::aerugo::Aerugo;
use crate::api::{InitError, RuntimeError};
use crate::event::{Event, EventId, EventSet};
use crate::tasklet::TaskletPtr;

/// Type for list of events.
type EventList = Vec<Event, { EventManager::EVENT_COUNT }>;
/// Type fo list of event sets.
type EventSetList = Vec<EventSet, { Aerugo::TASKLET_COUNT }>;

/// System events manager.
///
/// This shouldn't be created by hand by the user or anywhere else in the code.
/// It should be used as a singleton (crate::aerugo::EVENT_MANAGER) and shouldn't be directly accessed
/// by any other part of the system.
pub(crate) struct EventManager {
    /// List of events in the system.
    events: InternalCell<EventList>,
    /// List of event sets.
    event_sets: InternalCell<EventSetList>,
}

impl EventManager {
    /// Number of events in the system.
    #[read_env("AERUGO_EVENT_COUNT")]
    pub(crate) const EVENT_COUNT: usize = 0;

    /// Creates new EventManager instance.
    pub(crate) const fn new() -> Self {
        EventManager {
            events: InternalCell::new(EventList::new()),
            event_sets: InternalCell::new(EventSetList::new()),
        }
    }

    /// Creates new event in the system.
    ///
    /// # Parameters
    /// * `event_id` - ID for the new event.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of events.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn create_event(&'static self, event_id: EventId) -> Result<(), InitError> {
        let event = Event::new(event_id);

        match self.events.as_mut_ref().push(event) {
            Ok(_) => Ok(()),
            Err(_) => Err(InitError::EventListFull),
        }
    }

    /// Creates new event set.
    ///
    /// # Parameters
    /// * `tasklet` - Tasklet that will be assigned to this event set.
    ///
    /// # Returns
    /// Reference to `EventSet` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of event sets.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn create_event_set(
        &'static self,
        tasklet: TaskletPtr,
    ) -> Result<&'static EventSet, InitError> {
        let event_set = EventSet::new(tasklet);

        match self.event_sets.as_mut_ref().push(event_set) {
            Ok(_) => (),
            Err(_) => return Err(InitError::EventSetListFull),
        };

        Ok(self.event_sets.as_ref().last().unwrap())
    }

    /// Returns reference to the event with given ID.
    pub(crate) fn get_event(&'static self, event_id: EventId) -> Option<&'static Event> {
        // SAFETY: This is safe, because no mutable references should be able to exist at the same time.
        unsafe {
            self.events
                .as_ref()
                .iter()
                .find(|&event| event.id() == event_id)
        }
    }

    /// Emits event with the given ID.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to emit.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    pub(crate) fn emit(&'static self, event_id: EventId) -> Result<(), RuntimeError> {
        match self.get_event(event_id) {
            Some(event) => event.emit(),
            None => Err(RuntimeError::EventNotFound(event_id)),
        }
    }

    /// Cancels event with the given ID.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to emit.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    pub(crate) fn cancel(&'static self, event_id: EventId) -> Result<(), RuntimeError> {
        match self.get_event(event_id) {
            Some(event) => event.cancel(),
            None => Err(RuntimeError::EventNotFound(event_id)),
        }
    }

    /// Clears event queue
    pub(crate) fn clear(&'static self) {
        // SAFETY: This is safe, because no mutable references should be able to exist at the same time.
        for event_set in unsafe { self.event_sets.as_ref() } {
            event_set.clear();
        }
    }
}
