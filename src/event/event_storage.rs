//! Static storage for [event](crate::event::Event).
//!
//! This module contains a event storage, which is a statically allocated memory that will
//! store condition structure for the duration of the system life.

use core::cell::OnceCell;

use heapless::Vec;

use crate::api::InitError;
use crate::event::{Event, EventHandle, EventId};

/// Type of the event data storage.
type EventBuffer = Vec<u8, { core::mem::size_of::<Event>() }>;

/// Structure containing memory for Event creation.
///
/// As this system cannot use dynamic memory allocation, all structures have to be allocated
/// statically. Per good practices user is separated from the actual implementation and instead
/// only has to provide a static memory (via this structure) where the Event will be allocated.
pub struct EventStorage {
    /// Marks whether this storage has been initialized.
    initialized: OnceCell<()>,
    /// Buffer for the event structure.
    event_buffer: OnceCell<EventBuffer>,
}

impl EventStorage {
    /// Creates new storage.
    pub const fn new() -> Self {
        EventStorage {
            initialized: OnceCell::new(),
            event_buffer: OnceCell::new(),
        }
    }

    /// Returns initialization status of thie storage.
    pub fn is_initialized(&'static self) -> bool {
        self.initialized.get().is_some()
    }

    /// Creates new handle to an event allocated in this storage.
    ///
    /// # Return
    /// `Some(handle)` if this storage has been initialized. `None` otherwise.
    pub fn create_handle(&'static self) -> Option<EventHandle> {
        self.event().map(EventHandle::new)
    }

    /// Initializes this storage.
    ///
    /// # Parameters
    /// * `event_id` - ID of the new event.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the stored condition buffer.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn init(&'static self, event_id: EventId) -> Result<(), InitError> {
        if self.initialized.get().is_some() {
            return Err(InitError::StorageAlreadyInitialized);
        }

        let event = Event::new(event_id);

        // This is safe, because `event_buffer` doesn't contain any value yet, and it's size is
        // guaranteed to be large enough to store event structure.
        let event_buffer = EventBuffer::new();
        unsafe {
            let event_buffer_ptr = event_buffer.as_ptr() as *mut Event;
            core::ptr::write(event_buffer_ptr, event);
        }

        self.event_buffer
            .set(event_buffer)
            .expect("Failed to initialize EventStorage buffer");

        self.initialized
            .set(())
            .expect("Failed to set EventStorage initialization status");

        Ok(())
    }

    /// Returns reference to the stored Event structure
    ///
    /// # Return
    /// `Some(event)` if storage is initialized, `None` otherwise.
    #[inline(always)]
    pub(crate) fn event(&'static self) -> Option<&'static Event> {
        match (self.initialized.get(), self.event_buffer.get()) {
            // SAFETY: This is safe, because the storage is initialized.
            (Some(_), Some(buffer)) => unsafe { Some(&*(buffer.as_ptr() as *const Event)) },
            (_, _) => None,
        }
    }
}

unsafe impl Sync for EventStorage {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        static STORAGE: EventStorage = EventStorage::new();

        assert!(!STORAGE.is_initialized());
    }

    #[test]
    fn initialize() {
        static STORAGE: EventStorage = EventStorage::new();

        let init_result = unsafe { STORAGE.init(0) };
        assert!(init_result.is_ok());
        assert!(STORAGE.is_initialized());
    }

    #[test]
    fn fail_double_initialization() {
        static STORAGE: EventStorage = EventStorage::new();

        let init_result = unsafe { STORAGE.init(0) };
        assert!(init_result.is_ok());
        let init_result = unsafe { STORAGE.init(1) };

        assert!(init_result.is_err());
        assert_eq!(
            init_result.err().unwrap(),
            InitError::StorageAlreadyInitialized
        );
    }

    #[test]
    fn create_handle() {
        static STORAGE: EventStorage = EventStorage::new();

        let _ = unsafe { STORAGE.init(0) };

        let handle = STORAGE.create_handle();
        assert!(handle.is_some());
    }

    #[test]
    fn fail_create_handle_uninitialized() {
        static STORAGE: EventStorage = EventStorage::new();

        let handle = STORAGE.create_handle();
        assert!(handle.is_none());
    }
}
