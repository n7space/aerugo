//! System time manager.
//!
//! This module contains a system event manager. It stores events created in the system and manages
//! event sets that are assigned to the tasklets.

use env_parser::read_env;

use crate::aerugo::Aerugo;
use crate::api::RuntimeError;
use crate::error::SystemError;
use crate::event::{Event, EventId, EventSet};
use crate::internal_list::InternalList;
use crate::tasklet::TaskletPtr;

/// Type for list of events.
type EventList = InternalList<&'static Event, { EventManager::EVENT_COUNT }>;
/// Type for list of event sets.
type EventSetList = InternalList<EventSet, { Aerugo::TASKLET_COUNT }>;

/// System events manager.
///
/// This shouldn't be created by hand by the user or anywhere else in the code.
/// It should be used as a singleton (crate::aerugo::EVENT_MANAGER) and shouldn't be directly accessed
/// by any other part of the system.
pub(crate) struct EventManager {
    /// List of events in the system.
    events: EventList,
    /// List of event sets.
    event_sets: EventSetList,
}

impl EventManager {
    /// Number of events in the system.
    #[read_env("AERUGO_EVENT_COUNT")]
    pub(crate) const EVENT_COUNT: usize = 0;

    /// Creates new EventManager instance.
    pub(crate) const fn new() -> Self {
        EventManager {
            events: EventList::new(),
            event_sets: EventSetList::new(),
        }
    }

    /// Creates new event in the system.
    ///
    /// # Parameters
    /// * `event_id` - ID for the new event.
    ///
    /// # Return
    /// `()` if successful, `SystemError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of events.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn add_event(
        &'static self,
        event: &'static Event,
    ) -> Result<(), SystemError> {
        if self.has_event(event.id()) {
            return Err(SystemError::EventAlreadyExists(event.id()));
        }

        match self.events.add(event) {
            Ok(_) => Ok(()),
            Err(_) => Err(SystemError::EventListFull),
        }
    }

    /// Creates new event set.
    ///
    /// # Parameters
    /// * `tasklet` - Tasklet that will be assigned to this event set.
    ///
    /// # Returns
    /// Reference to `EventSet` if successful, `SystemError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of event sets.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn create_event_set(
        &'static self,
        tasklet: TaskletPtr,
    ) -> Result<&'static EventSet, SystemError> {
        let event_set = EventSet::new(tasklet);

        match self.event_sets.add(event_set) {
            Ok(_) => (),
            Err(_) => return Err(SystemError::EventSetListFull),
        };

        Ok(self.event_sets.as_ref().last().unwrap())
    }

    /// Checks if event of given ID exists.
    pub(crate) fn has_event(&'static self, event_id: EventId) -> bool {
        self.events.iter().any(|&event| event.id() == event_id)
    }

    /// Returns reference to the event with given ID.
    pub(crate) fn get_event(&'static self, event_id: EventId) -> Option<&'static Event> {
        self.events
            .iter()
            .find(|&event| event.id() == event_id)
            .copied()
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
        for event_set in &self.event_sets {
            event_set.clear();
        }
    }
}

unsafe impl Sync for EventManager {}
