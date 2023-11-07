//! Module containing sturctures related to system events.

mod event_handle;
mod event_set;
mod event_storage;

pub use self::event_handle::EventHandle;
pub use self::event_storage::EventStorage;

pub(crate) use self::event_set::EventSet;

use crate::aerugo::Aerugo;
use crate::error::SystemError;
use crate::internal_list::InternalList;

/// System event ID.
pub type EventId = u32;

/// Type for list of event sets.
type EventSetList = InternalList<&'static EventSet, { Aerugo::TASKLET_COUNT }>;

/// System event used for sending signals between tasklets.
pub(crate) struct Event {
    /// ID of this event.
    id: EventId,
    /// List of sets that this event is in.
    sets: EventSetList,
}

/// It is safe assuming that Event is not available from IRQ context before it's created and that
/// modifications cannot be interrupted.
///
/// Event structure is hidden from the user. Functionalities are exposed to the user via
/// [EventHandle] and [RuntimeApi](crate::api::RuntimeApi).
///
/// Event is only created by `EventStorage` with [create_event](crate::api::InitApi::create_event)
/// which is not accessible from the IRQ context.
///
/// Initializations and modifications mustn't be interrupted. Event is only accessiable with an
/// unmutable reference.
unsafe impl Sync for Event {}

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
    /// `()` if successful, `SystemError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of event sets.
    /// This is safe if it's executed in a critical section during system initialization
    /// (before scheduler is started).
    /// Accessing event from IRQ context during adding to set is undefined behaviour.
    pub(crate) unsafe fn add_set(&self, event_set: &'static EventSet) -> Result<(), SystemError> {
        match self.sets.add(event_set) {
            Ok(_) => Ok(()),
            Err(_) => Err(SystemError::EventSetListFull),
        }
    }

    /// Emits this event.
    ///
    /// This sets the value of this event to `true` in each event set and wakes tasklet assigned to
    /// those sets.
    pub(crate) fn emit(&self) {
        for event_set in &self.sets {
            event_set
                .activate_event(self.id)
                .expect("Failed to activate an event");
        }
    }
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
