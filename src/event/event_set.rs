//! Module containing event set.

use heapless::spsc::Queue;

use crate::aerugo::Aerugo;
use crate::data_provider::DataProvider;
use crate::error::SystemError;
use crate::event::EventId;
use crate::event_manager::EventManager;
use crate::mutex::Mutex;
use crate::tasklet::TaskletPtr;
use crate::utils::max;

/// Type for event queue.
type EventQueue = Queue<EventId, { max(EventManager::EVENT_COUNT, 2) }>;

/// Event set.
///
/// Event set is used as a data provider for the Tasklet. It keeps track to which events is given
/// tasklet subscribed to and which events are active.
pub(crate) struct EventSet {
    /// Tasklet assigned to this set.
    tasklet: TaskletPtr,
    /// Event queue.
    event_queue: Mutex<EventQueue>,
}

impl EventSet {
    /// Creates new event set.
    pub(crate) fn new(tasklet: TaskletPtr) -> Self {
        EventSet {
            tasklet,
            event_queue: EventQueue::new().into(),
        }
    }

    /// Activates event
    ///
    /// # Parameters
    /// * `event_id` - Event ID to activate.
    ///
    /// # Return
    /// `true` if successfully activated event, `false` if event was already on the event queue
    /// and is waiting for trigger, `SystemError` otherwise.
    pub(crate) fn activate_event(&self, event_id: EventId) -> Result<bool, SystemError> {
        let event_activated = self.event_queue.lock(|event_queue| {
            let found_event = event_queue.iter().find(|&&id| id == event_id);

            match found_event {
                Some(_) => Ok(false),
                None => match event_queue.enqueue(event_id) {
                    Ok(_) => Ok(true),
                    Err(_) => Err(SystemError::EventQueueFull),
                },
            }
        })?;

        if event_activated {
            Aerugo::wake_tasklet(&self.tasklet);
        }

        Ok(event_activated)
    }
}

impl DataProvider<EventId> for EventSet {
    /// Returns ID of the event active in this set.
    ///
    /// It takes the event in the order by the ID, starting from the lowest.
    ///
    /// # Return
    /// `Some(EventId)` if there is any event active, `None` otherwise.
    fn get_data(&self) -> Option<EventId> {
        self.event_queue.lock(|event_queue| event_queue.dequeue())
    }

    /// Checks if any event in this set is active.
    fn data_waiting(&self) -> bool {
        self.event_queue.lock(|event_queue| !event_queue.is_empty())
    }
}
