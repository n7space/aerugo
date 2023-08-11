//! Module containing event set.

use heapless::Vec;

use crate::aerugo::AERUGO;
use crate::api::{InitError, RuntimeError, SystemApi};
use crate::arch::Mutex;
use crate::data_provider::DataProvider;
use crate::event::EventId;
use crate::event_manager::EventManager;
use crate::tasklet::TaskletPtr;

/// Stores information about event value in given set.
struct EventState {
    /// ID of the event.
    pub event_id: EventId,
    /// Value of the event.
    pub value: bool,
}

/// Type for list of event states in a set.
type EventStateList = Vec<EventState, { EventManager::EVENT_COUNT }>;

/// Event set.
///
/// Event set is used as a data provider for the Tasklet. It keeps track to which events is given
/// tasklet subscribed to and what are values of those events.
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
            value: false,
        };

        self.event_states
            .lock(|event_states| match event_states.push(event_state) {
                Ok(_) => Ok(()),
                Err(_) => Err(InitError::EventSetFull),
            })
    }

    /// Sets value of given event.
    ///
    /// # Parameters
    /// * `event_id` - Event ID to change value of.
    /// * `value` - New value.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    pub(crate) fn set_event_value(
        &self,
        event_id: EventId,
        value: bool,
    ) -> Result<(), RuntimeError> {
        self.event_states.lock(|event_states| {
            match event_states
                .iter_mut()
                .find(|event_state| event_state.event_id == event_id)
            {
                Some(event_state) => event_state.value = value,
                None => return Err(RuntimeError::EventNotFound(event_id)),
            };

            Ok(())
        })?;

        if value {
            AERUGO.wake_tasklet(&self.tasklet);
        }

        Ok(())
    }
}

impl DataProvider<EventId> for EventSet {
    fn data_ready(&self) -> bool {
        self.event_states
            .lock(|event_states| event_states.iter().any(|event_state| event_state.value))
    }

    fn get_data(&self) -> Option<EventId> {
        self.event_states.lock(|event_states| {
            match event_states
                .iter_mut()
                .find(|event_state| event_state.value)
            {
                Some(event_state) => {
                    event_state.value = false;
                    Some(event_state.event_id)
                }
                None => None,
            }
        })
    }
}
