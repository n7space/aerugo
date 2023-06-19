//! Configuration for creating tasklets.

use crate::aerugo::Aerugo;
use crate::api::init_api;

/// Configuration for tasklets.
pub struct TaskletConfig {
    /// Name of the tasklet.
    pub name: &'static str,
}

impl init_api::TaskConfig for TaskletConfig {}

impl init_api::TaskConfigType for Aerugo {
    type TaskConfig = TaskletConfig;
}

impl Default for TaskletConfig {
    fn default() -> Self {
        TaskletConfig {
            name: "MISSING_TASKLET_NAME",
        }
    }
}
