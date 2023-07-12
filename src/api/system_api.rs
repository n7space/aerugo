//! Internal system API.
//!
//! This API is used for the communication between different parts of the system.

use crate::tasklet::TaskletPtr;

/// Internal system API.
pub(crate) trait SystemApi {
    fn wake_tasklet(&'static self, _tasklet: &TaskletPtr);
}
