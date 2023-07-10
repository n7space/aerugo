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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default() {
        let tasklet_config = TaskletConfig::default();

        assert_eq!(tasklet_config.name, "MISSING_TASKLET_NAME");
    }

    #[test]
    fn create() {
        let name = "TaskName";

        let tasklet_config = TaskletConfig { name };

        assert_eq!(tasklet_config.name, name);
    }
}
