//! Module containing event set.

use heapless::Vec;

use crate::aerugo::AERUGO;
use crate::api::{InitError, RuntimeError, SystemApi};
use crate::arch::Mutex;
use crate::data_provider::DataProvider;
use crate::event::EventId;
use crate::event_manager::EventManager;
use crate::tasklet::TaskletPtr;

/// Stores information about event state in given set.
struct EventState {
    /// ID of the event.
    pub event_id: EventId,
    /// Whether event is active
    pub active: bool,
}

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
        let event_state = EventState {
            event_id,
            active: false,
        };

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
                .find(|event_state| event_state.event_id == event_id)
            {
                Some(event_state) => event_state.active = true,
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
                .find(|event_state| event_state.event_id == event_id)
            {
                Some(event_state) => event_state.active = false,
                None => return Err(RuntimeError::EventNotFound(event_id)),
            };

            Ok(())
        })
    }
}

impl DataProvider<EventId> for EventSet {
    fn data_ready(&self) -> bool {
        self.event_states
            .lock(|event_states| event_states.iter().any(|event_state| event_state.active))
    }

    fn get_data(&self) -> Option<EventId> {
        self.event_states.lock(|event_states| {
            match event_states
                .iter_mut()
                .find(|event_state| event_state.active)
            {
                Some(event_state) => {
                    event_state.active = false;
                    Some(event_state.event_id)
                }
                None => None,
            }
        })
    }
}
