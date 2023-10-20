//! System time manager.
//!
//! This module contains a system event manager. It stores events created in the system and manages
//! event sets that are assigned to the tasklets.

use env_parser::read_env;
use heapless::Vec;

use crate::aerugo::Aerugo;
use crate::error::{RuntimeError, SystemError};
use crate::event::{Event, EventId, EventSet};
use crate::internal_list::InternalList;
use crate::mutex::Mutex;
use crate::tasklet::TaskletPtr;
use crate::time::Instant;
use crate::time_source::TimeSource;

/// Type for list of events.
type EventList = InternalList<&'static Event, { EventManager::EVENT_COUNT }>;
/// Type for list of event sets.
type EventSetList = InternalList<EventSet, { Aerugo::TASKLET_COUNT }>;
/// Type for list of scheduled events.
type ScheduledEventList = Vec<ScheduledEvent, { EventManager::EVENT_COUNT }>;

/// Stores info about scheduled event.
struct ScheduledEvent {
    /// Reference to the event.
    pub event: &'static Event,
    /// Time when event should become active.
    pub time: Instant,
}

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
    /// List of scheduled events.
    scheduled_events: Mutex<ScheduledEventList>,
    /// Time source.
    time_source: &'static TimeSource,
}

/// It is safe assuming that it's modified only during system initialization (before scheduler is
/// started) and that those modifications cannot be interrupted.
///
/// EventManager stores a list of scheduled events which is guarded with [Mutex] which ensures that
/// modifications cannot be interrupted.
unsafe impl Sync for EventManager {}

impl EventManager {
    /// Number of events in the system.
    #[read_env("AERUGO_EVENT_COUNT")]
    pub(crate) const EVENT_COUNT: usize = 0;

    /// Creates new EventManager instance.
    pub(crate) const fn new(time_source: &'static TimeSource) -> Self {
        EventManager {
            events: EventList::new(),
            event_sets: EventSetList::new(),
            scheduled_events: Mutex::new(ScheduledEventList::new()),
            time_source,
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
    /// This is safe if it's executed in a critical section during system initialization
    /// (before scheduler is started).
    /// Accessing `EventManager` from IRQ context during this function is undefined behaviour.
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
    /// This is safe to call during system initialization (before scheduler is started).
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

    /// Emits event with the given ID.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to emit.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    pub(crate) fn emit(&'static self, event_id: EventId) -> Result<(), RuntimeError> {
        let event = match self.get_event(event_id) {
            Some(event) => event,
            None => return Err(RuntimeError::EventNotFound(event_id)),
        };

        event.emit();

        Ok(())
    }

    /// Schedule event of given ID.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to emit.
    /// * `time` - Time since the scheduler start when event should be activated.
    ///
    /// # Return
    /// `bool` indicating if event was successfully scheduled, `RuntimeError` if some error
    /// occurred.
    ///
    pub(crate) fn schedule(
        &'static self,
        event_id: EventId,
        time: Instant,
    ) -> Result<bool, RuntimeError> {
        let event = match self.get_event(event_id) {
            Some(event) => event,
            None => return Err(RuntimeError::EventNotFound(event_id)),
        };

        let reschedule = self.is_scheduled(event_id).unwrap();

        if reschedule {
            self.reschedule_event(event, time)
                .expect("Failed to reschedule event");
        } else {
            self.schedule_event(event, time)
                .expect("Failed to schedule event");
        }

        Ok(reschedule)
    }

    /// Checks if event of given ID is scheduled to be emitted.
    ///
    /// If event was already scheduled at this time, that event will be rescheduled to the given time.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to check.
    ///
    /// # Return
    /// `bool` indicating if event was rescheduled, `RuntimeError` if some error occurred.
    pub(crate) fn is_scheduled(&'static self, event_id: EventId) -> Result<bool, RuntimeError> {
        let event = match self.get_event(event_id) {
            Some(event) => event,
            None => return Err(RuntimeError::EventNotFound(event_id)),
        };

        let is_scheduled = self.scheduled_events.lock(|se| {
            se.iter()
                .any(|scheduled_event| scheduled_event.event == event)
        });

        Ok(is_scheduled)
    }

    /// Cancels event with the given ID.
    ///
    /// # Parameters
    /// * `event_id` - ID of event to emit.
    ///
    /// # Return
    /// `()` if successful, `RuntimeError` otherwise.
    pub(crate) fn cancel(&'static self, event_id: EventId) -> Result<bool, RuntimeError> {
        let event = match self.get_event(event_id) {
            Some(event) => event,
            None => return Err(RuntimeError::EventNotFound(event_id)),
        };

        self.scheduled_events.lock(|se| {
            match se
                .iter()
                .position(|scheduled_event| scheduled_event.event == event)
            {
                Some(index) => {
                    se.remove(index);
                    Ok(true)
                }
                None => Ok(false),
            }
        })
    }

    /// Clears event queue
    pub(crate) fn clear(&'static self) {
        self.scheduled_events.lock(|se| se.clear())
    }

    /// Activate events that were scheduled for the current time.
    pub(crate) fn activate_scheduled_events(&'static self) {
        self.scheduled_events.lock(|se| {
            se.retain(|scheduled_event| {
                let current_time = self.time_source.system_time();

                if current_time >= scheduled_event.time {
                    scheduled_event.event.emit();
                    false
                } else {
                    true
                }
            });
        })
    }

    /// Schedules event for a given time.
    ///
    /// # Parameters
    /// * `event` - Event to schedule.
    /// * `time` - Time for event to become active.
    ///
    /// # Return
    /// `()` if successful, `SystemError` in case of an error.
    fn schedule_event(
        &'static self,
        event: &'static Event,
        time: Instant,
    ) -> Result<(), SystemError> {
        let scheduled_event = ScheduledEvent { event, time };

        self.scheduled_events
            .lock(|se| match se.push(scheduled_event) {
                Ok(_) => Ok(()),
                Err(_) => Err(SystemError::ScheduledEventListFull),
            })
    }

    /// Reschedules event to a new given time.
    ///
    /// # Parameters
    /// * `event` - Event to reschedule.
    /// * `new_time` - New time for the event to become active.
    ///
    /// # Return
    /// `()` if successful, `SystemError` in case of an error.
    fn reschedule_event(
        &'static self,
        event: &'static Event,
        new_time: Instant,
    ) -> Result<(), SystemError> {
        self.scheduled_events.lock(|se| {
            let scheduled_event = se
                .iter_mut()
                .find(|scheduled_event| scheduled_event.event == event)
                .unwrap();

            scheduled_event.time = new_time;

            Ok(())
        })
    }
}
