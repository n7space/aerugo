//! Static storage for [event](crate::event::Event).
//!
//! As this system cannot use dynamic memory allocation, all structures have to be allocated
//! statically. Per good practices user is separated from the actual implementation and instead
//! only has to provide a static memory (via this structure) where the Event will be allocated.

use super::Event;

use crate::event::EventHandle;
use crate::internal_cell::InternalCell;
use heapless::Vec;

/// Type of the event buffer storage.
pub(crate) type EventBuffer = Vec<u8, { core::mem::size_of::<Event>() }>;

/// Structure containing memory for Event creation.
pub struct EventStorage {
    /// Marks whether this storage has been initialized.
    _initialized: InternalCell<bool>,
    /// Buffer for the event structure.
    _event_buffer: InternalCell<EventBuffer>,
}

impl EventStorage {
    /// Creates new storage.
    pub const fn new() -> Self {
        EventStorage {
            _initialized: InternalCell::new(false),
            _event_buffer: InternalCell::new(EventBuffer::new()),
        }
    }

    /// Creates new handle to an event allocated in this storage.
    ///
    /// Returns `Some(handle)` if this storage has been initialized, `None` otherwise.
    pub fn create_event_handle(&'static self) -> Option<EventHandle> {
        todo!()
    }
}
