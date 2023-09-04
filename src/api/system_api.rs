//! Internal system API.
//!
//! This API is used for the communication between different parts of the system.

use crate::tasklet::TaskletPtr;

/// Internal system API.
pub(crate) trait SystemApi {
    /// Wakes given tasklet by scheduling it for execution.
    ///
    /// # Parameters
    /// * `tasklet` - Tasklet to wake
    fn wake_tasklet(&'static self, tasklet: &TaskletPtr);

    /// Runs subsystems updates between tasklet executions.
    fn update(&'static self);
}
