//! Structures related to the system configuration.

use super::Aerugo;

use crate::api::init_api;

/// Configuration for tasklets.
#[derive(Default)]
pub struct TaskletConfiguration {}

impl init_api::TaskConfiguration for TaskletConfiguration {}

impl init_api::TaskConfigType for Aerugo {
    type TaskConfig = TaskletConfiguration;
}
