//! Possible system errors.

use core::fmt;

use crate::event::EventId;

/// System error.
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum SystemError {
    /// Executor tasklet queue was full.
    ExecutorTaskletQueueFull,
    /// Given storage was initialized twice.
    StorageAlreadyInitialized,
    /// Failed to set storage buffer.
    StorageBufferAlreadySet,
    /// Failed to set storage initialization status.
    StorageInitializedAlreadySet,
    /// Tasklet already has a condition set.
    TaskletAlreadyHasConditionSet(&'static str),
    /// Tasklet is already subscribed to a data provider.
    TaskletAlreadySubscribed(&'static str),
    /// Tasklet list was full when tried to add a new one.
    TaskletListFull,
    /// Event list was full when tried to create a new one.
    EventListFull,
    /// Event set list was full when tried to create a new one.
    EventSetListFull,
    /// Scheduled event list was full when tried to schedule a new one.
    #[allow(dead_code)]
    ScheduledEventListFull,
    /// Enqueued event to a full event queue.
    EventQueueFull,
    /// Event already exists in the system.
    EventAlreadyExists(EventId),
    /// Cyclic execution was full when tried to create a new one.
    CyclicExecutionListFull,
}

impl fmt::Debug for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SystemError::ExecutorTaskletQueueFull => {
                write!(f,
                    "internal system error. Executor tasklet queue have size equal to the maximum number of
                    tasklets that can be created in the system. Each tasklet should be in the queue only once
                    at a time. This error means that there is some fault logic deciding whether tasklet should
                    be added to the queue.")
            }
            SystemError::StorageAlreadyInitialized => {
                write!(f,
                    "this storage was used twice to create an element. Each storage can be used only once.")
            }
            SystemError::StorageBufferAlreadySet => {
                write!(f,
                    "internal system error. Setting the buffer in storage failed, which should never happen.
                    This means there is some faulty logic deciding whether storage is already initialized.")
            }
            SystemError::StorageInitializedAlreadySet => {
                write!(f,
                    "internal system error. Setting the initialization status in storage failed, which should
                    never happen. This means there is some faulty logic deciding whether storage is already
                    initialized.")
            }
            SystemError::TaskletAlreadyHasConditionSet(tasklet_name) => {
                write!(f,
                    "tasklet '{}' already has a condition set. Tasklet can only have at maximum one condition set.
                    To add more conditions for a tasklet, add them to the original condition set.",
                    tasklet_name)
            }
            SystemError::TaskletAlreadySubscribed(tasklet_name) => {
                write!(f,
                    "tasklet '{}' is already subscribed to a data provider. Tasklet can only have at maximum one
                    data provider.",
                    tasklet_name)
            }
            SystemError::TaskletListFull => {
                write!(f,
                    "tasklet list is full. To configure number of tasklets in the system use the AERUG_TASKLET_COUNT
                    enviromental variable.")
            }
            SystemError::EventListFull => {
                write!(f,
                    "event list is full. To configure number of events in the system use the AERUGO_EVENTS_COUNT
                    enviromental variable.")
            }
            SystemError::EventSetListFull => {
                write!(f,
                    "internal system error. Event manager stores a list of event sets of size equal to the maximum
                    number of tasklets that can be created in the system. Each tasklet should have at maximum only one
                    event set. This error means that there is some fault logic in event set creation.")
            }
            SystemError::ScheduledEventListFull => {
                write!(f,
                    "internal system error. Event managers stores a list of events that are scheduled to become active
                    at a given time of size equal to the maximum number of events that can be created in the system.
                    Each event should be scheduled only once at a given time. This error means that there is some fault
                    logic in scheduling events.")
            }
            SystemError::EventQueueFull => {
                write!(f,
                    "internal system error. Event set stores a list of activated events of size equal to the maximum
                    number of exents that can be created in the system. Each event should be active only once in a
                    given set at a given time. This error means that there is some fault logic in scheduling events.")
            }
            SystemError::EventAlreadyExists(event_id) => {
                write!(f,
                    "event with the ID {} already exists in the system. Each event has to have an unique ID.",
                    event_id)
            }
            SystemError::CyclicExecutionListFull => {
                write!(f,
                    "internal system error. Cyclic execution manager stores a list of cyclic executions of size equal
                    to the maximum number of tasklets that can be created in the system. Each tasklet should have at
                    maximum only one cyclic execution. This error means that there is some fault logic in cyclic
                    exection creation.")
            }
        }
    }
}
