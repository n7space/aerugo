//! Internal system API.
//!
//! This API is used for the communication between different parts of the system.

use crate::event::{Event, EventId};
use crate::tasklet::TaskletPtr;

/// Internal system API.
pub(crate) trait SystemApi {
    /// Returns reference to the event with given id.
    ///
    /// # Parameters
    /// * `event_id` - ID of the event to find.
    ///
    /// # Return
    /// Event reference if found or `None` otherwise.
    fn get_event(&'static self, event_id: EventId) -> Option<&'static Event>;

    /// Wakes given tasklet by scheduling it for execution.
    ///
    /// # Parameters
    /// * `tasklet` - Tasklet to wake
    fn wake_tasklet(&'static self, tasklet: &TaskletPtr);

    /// Runs subsystems updates between tasklet executions.
    fn update(&'static self);
}
