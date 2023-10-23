//! Module with tasklet ID.

use core::sync::atomic::{AtomicU32, Ordering};

/// Tasklet unique ID.
#[derive(Copy, Clone)]
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
