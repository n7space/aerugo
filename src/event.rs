//! Module containing sturctures related to system events.

mod event_enabler;
mod event_set;

pub use self::event_enabler::EventEnabler;

pub(crate) use self::event_set::EventSet;

use heapless::Vec;
use internal_cell::InternalCell;

use crate::aerugo::Aerugo;
use crate::api::{InitError, RuntimeError};

/// Type for list of event sets.
type EventSetList = Vec<&'static EventSet, { Aerugo::TASKLET_COUNT }>;

/// System event used for sending signals between tasklets.
pub(crate) struct Event {
    /// ID of this event.
    event_id: EventId,
    /// List of sets that this tasklet is in.
    sets: InternalCell<EventSetList>,
}

impl Event {
    /// Creates new event.
    pub(crate) fn new(event_id: EventId) -> Self {
        Self {
            event_id,
            sets: EventSetList::new().into(),
        }
    }

    /// Returns ID of this event.
    pub(crate) fn id(&self) -> EventId {
        self.event_id
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
        match self.sets.as_mut_ref().push(event_set) {
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
        for event_set in unsafe { self.sets.as_ref() } {
            event_set.activate_event(self.event_id)?;
        }

        Ok(())
    }
}

/// System event ID.
pub type EventId = u32;
