//! Module containing event set.

use heapless::Vec;

use crate::aerugo::AERUGO;
use crate::api::{InitError, RuntimeError, SystemApi};
use crate::arch::Mutex;
use crate::data_provider::DataProvider;
use crate::event::EventId;
use crate::event_manager::EventManager;
use crate::tasklet::TaskletPtr;

/// Module containing event state.
mod state {
    use crate::event::EventId;

    /// Stores information about event state in given set.
    pub(crate) struct EventState {
        /// ID of the event.
        event_id: EventId,
        /// Whether event is active
        active: bool,
    }

    impl EventState {
        /// Creates new EventState
        ///
        /// # Parameters
        /// * `event_id` - ID of the event.
        pub(crate) fn new(event_id: EventId) -> Self {
            EventState {
                event_id,
                active: false,
            }
        }

        /// Returns ID of the event.
        pub(crate) fn id(&self) -> EventId {
            self.event_id
        }

        /// Returns state of the event.
        pub(crate) fn is_active(&self) -> bool {
            self.active
        }

        /// Sets new state of the event.
        pub(crate) fn set_active(&mut self, active: bool) {
            self.active = active;
        }
    }
}

use state::EventState;

/// Type for list of event states in a set.
type EventStateList = Vec<EventState, { EventManager::EVENT_COUNT }>;

/// Event set.
///
/// Event set is used as a data provider for the Tasklet. It keeps track to which events is given
/// tasklet subscribed to and which events are active.
pub(crate) struct EventSet {
    /// Tasklet assigned to this set.
    tasklet: TaskletPtr,
    /// List of event states.
    event_states: Mutex<EventStateList>,
}

impl EventSet {
    /// Creates new event set.
    pub(crate) fn new(tasklet: TaskletPtr) -> Self {
        EventSet {
            tasklet,
            event_states: EventStateList::new().into(),
        }
    }

    /// Adds new event to the set.
    ///
    /// # Parameters
    /// * `event_id` - ID of the event that is to be added to this set.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of event states.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn add_event(&self, event_id: EventId) -> Result<(), InitError> {
        let event_state = EventState::new(event_id);

        self.event_states
            .lock(|event_states| match event_states.push(event_state) {
                Ok(_) => Ok(()),
                Err(_) => Err(InitError::EventSetFull),
            })
    }

    /// Activates event
    ///
    /// # Parameters
    /// * `event_id` - Event ID to activate.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    pub(crate) fn activate_event(&self, event_id: EventId) -> Result<(), RuntimeError> {
        self.event_states.lock(|event_states| {
            match event_states
                .iter_mut()
                .find(|event_state| event_state.id() == event_id)
            {
                Some(event_state) => event_state.set_active(true),
                None => return Err(RuntimeError::EventNotFound(event_id)),
            };

            Ok(())
        })?;

        AERUGO.wake_tasklet(&self.tasklet);

        Ok(())
    }

    /// Deactivates event
    ///
    /// # Parameters
    /// * `event_id` - Event ID to deactivate.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    #[allow(dead_code)]
    pub(crate) fn deactivate_event(&self, event_id: EventId) -> Result<(), RuntimeError> {
        self.event_states.lock(|event_states| {
            match event_states
                .iter_mut()
                .find(|event_state| event_state.id() == event_id)
            {
                Some(event_state) => event_state.set_active(false),
                None => return Err(RuntimeError::EventNotFound(event_id)),
            };

            Ok(())
        })
    }

    /// Deactivates all events in the set.
    pub(crate) fn clear(&self) {
        self.event_states.lock(|event_states| {
            for event_state in event_states {
                event_state.set_active(false);
            }
        })
    }
}

impl DataProvider<EventId> for EventSet {
    /// Returns ID of the event active in this set.
    ///
    /// # Return
    /// `Some(EventId)` if there is any event active, `None` otherwise.
    fn get_data(&self) -> Option<EventId> {
        self.event_states.lock(|event_states| {
            match event_states
                .iter_mut()
                .find(|event_state| event_state.is_active())
            {
                Some(event_state) => {
                    event_state.set_active(false);
                    Some(event_state.id())
                }
                None => None,
            }
        })
    }

    /// Checks if any event in this set is active.
    fn data_waiting(&self) -> bool {
        self.event_states.lock(|event_states| {
            event_states
                .iter()
                .any(|event_state| event_state.is_active())
        })
    }
}
