//! Structures related to the system configuration.

use super::Aerugo;

use crate::api::init_api;

/// Configuration for tasklets.
pub struct TaskletConfiguration {}

impl init_api::TaskConfiguration for TaskletConfiguration {}

impl Default for TaskletConfiguration {
    fn default() -> Self {
        TaskletConfiguration {}
    }
}

impl init_api::TaskConfigType for Aerugo {
    type TaskConfig = TaskletConfiguration;
}
