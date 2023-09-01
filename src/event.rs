//! Module containing sturctures related to system events.

mod event_enabler;
mod event_set;

pub use self::event_enabler::EventEnabler;

pub(crate) use self::event_set::EventSet;

use crate::aerugo::Aerugo;
use crate::api::{InitError, RuntimeError};
use crate::internal_list::InternalList;

/// Type for list of event sets.
type EventSetList = InternalList<&'static EventSet, { Aerugo::TASKLET_COUNT }>;

/// System event used for sending signals between tasklets.
pub(crate) struct Event {
    /// ID of this event.
    id: EventId,
    /// List of sets that this event is in.
    sets: EventSetList,
}

impl Event {
    /// Creates new event.
    pub(crate) fn new(id: EventId) -> Self {
        Self {
            id,
            sets: EventSetList::new(),
        }
    }

    /// Returns ID of this event.
    pub(crate) fn id(&self) -> EventId {
        self.id
    }

    /// Adds new set to the list.
    ///
    /// # Parameters
    /// * `event_set` - Set to add.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of event sets.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn add_set(&self, event_set: &'static EventSet) -> Result<(), InitError> {
        match self.sets.add(event_set) {
            Ok(_) => Ok(()),
            Err(_) => Err(InitError::EventSetListFull),
        }
    }

    /// Emits this event.
    ///
    /// This sets the value of this event to `true` in each event set and wakes tasklet assigned to
    /// those sets.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise
    pub(crate) fn emit(&self) -> Result<(), RuntimeError> {
        for event_set in &self.sets {
            event_set.activate_event(self.id)?;
        }

        Ok(())
    }

    /// Cancels this event.
    ///
    /// This sets the value of this event to `false` in each event set.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise
    pub(crate) fn cancel(&self) -> Result<(), RuntimeError> {
        for event_set in &self.sets {
            event_set.deactivate_event(self.id)?;
        }

        Ok(())
    }
}

/// System event ID.
pub type EventId = u32;
