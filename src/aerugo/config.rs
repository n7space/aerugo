//! Structures related to the system configuration.

use super::Aerugo;

use crate::api::init_api;

/// Configuration for tasklets.
#[derive(Default)]
pub struct TaskletConfig {}

impl init_api::TaskConfig for TaskletConfig {}

impl init_api::TaskConfigType for Aerugo {
    type TaskConfig = TaskletConfig;
}
