//! Configuration for creating tasklets.

/// Configuration for tasklets.
#[derive(Copy, Clone)]
pub struct TaskletConfig {
    /// Name of the tasklet.
    pub name: &'static str,
    /// Priority of the tasklet.
    pub priority: u8,
}

impl Default for TaskletConfig {
    fn default() -> Self {
        TaskletConfig {
            name: "MISSING_TASKLET_NAME",
            priority: 0,
        }
    }
}
