//! Module with tasklet ID.

use core::fmt;
use core::sync::atomic::{AtomicU32, Ordering};

/// Tasklet unique ID.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TaskletId(pub u32);

impl TaskletId {
    /// Generates next ID.
    pub(crate) fn get_next() -> Self {
        /// Stores next ID to generate.
        static NEXT_ID: AtomicU32 = AtomicU32::new(0);

        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        TaskletId(id)
    }
}

impl fmt::Display for TaskletId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
