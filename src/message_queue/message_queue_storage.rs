//! Static storage for [message queue](crate::message_queue::MessageQueue).
//!
//! This module contains a message queue storage, which is a statically allocated memory that will
//! store queue structure for the duration of the system life.

use super::MessageQueue;

use core::cell::OnceCell;

use heapless::Vec;

use crate::api::InitError;
use crate::message_queue::MessageQueueHandle;
use crate::mutex::Mutex;

/// Type of the queue buffer storage.
pub(crate) type QueueBuffer = Vec<u8, { core::mem::size_of::<MessageQueue<(), 0>>() }>;
/// Type of the queue data storage.
pub(crate) type QueueData<T, const N: usize> = heapless::spsc::Queue<T, N>;

/// Structure containing memory for MessageQueue creation.
///
/// As this system cannot use dynamic memory allocation, all structures have to be allocated
/// statically. Per good practices user is separated from the actual implementation and instead
/// only has to provide a static memory (via this structure) where the MessageQueue will be allocated.
///
/// # Generic Parameters
/// * `T` - Type of the stored data.
/// * `N` - Size of the queue.
pub struct MessageQueueStorage<T, const N: usize> {
    /// Marks whether this storage has been initialized.
    initialized: OnceCell<()>,
    /// Buffer for the queue structure.
    queue_buffer: OnceCell<QueueBuffer>,
    /// Buffer for the queue data.
    queue_data: Mutex<QueueData<T, N>>,
}

impl<T, const N: usize> MessageQueueStorage<T, N> {
    /// Creates new storage.
    pub const fn new() -> Self {
        MessageQueueStorage {
            initialized: OnceCell::new(),
            queue_buffer: OnceCell::new(),
            queue_data: Mutex::new(QueueData::new()),
        }
    }

    /// Returns initialization status of this storage.
    pub fn is_initialized(&'static self) -> bool {
        self.initialized.get().is_some()
    }

    /// Creates new handle to a queue allocated in this storage.
    ///
    /// # Return
    /// `handle` if this storage has been initialized.
    pub fn create_handle(&'static self) -> Option<MessageQueueHandle<T, N>> {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // the `init` function.
        match self.initialized.get() {
            Some(_) => {
                let message_queue = self
                    .message_queue()
                    .expect("Failed to get reference to the stored MessageQueue");
                Some(MessageQueueHandle::new(message_queue))
            }
            None => None,
        }
    }

    /// Initializes this storage.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the stored queue and queue data buffers.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn init(&'static self) -> Result<(), InitError> {
        if self.initialized.get().is_some() {
            return Err(InitError::StorageAlreadyInitialized);
        }

        let queue = MessageQueue::<T, N>::new(&self.queue_data);

        // This is safe, because `queue_buffer` doesn't contain any value yet, and it's size is
        // guaranteed to be large enough to store queue structure.
        let queue_buffer = QueueBuffer::new();
        unsafe {
            let queue_buffer_ptr = queue_buffer.as_ptr() as *mut MessageQueue<T, N>;
            core::ptr::write(queue_buffer_ptr, queue);
        }

        self.queue_buffer
            .set(queue_buffer)
            .expect("Failed to initialize MessageQueueStorage buffer");

        self.initialized
            .set(())
            .expect("Failed to initialize MessageQueueStorage");

        Ok(())
    }

    /// Returns a reference to the stored MessageQueue structure.
    #[inline(always)]
    fn message_queue(&'static self) -> Option<&'static MessageQueue<T, N>> {
        match self.queue_buffer.get() {
            Some(buffer) => {
                // This is safe, because buffer is initialized
                unsafe { Some(&*(buffer.as_ptr() as *const MessageQueue<T, N>)) }
            }
            None => None,
        }
    }
}

unsafe impl<T, const N: usize> Sync for MessageQueueStorage<T, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        static STORAGE: MessageQueueStorage<u8, 2> = MessageQueueStorage::new();

        assert!(!STORAGE.is_initialized());
    }

    #[test]
    fn initialize() {
        static STORAGE: MessageQueueStorage<u8, 2> = MessageQueueStorage::new();

        let init_result = unsafe { STORAGE.init() };
        assert!(init_result.is_ok());
        assert!(STORAGE.is_initialized());
    }

    #[test]
    fn fail_double_initialization() {
        static STORAGE: MessageQueueStorage<u8, 2> = MessageQueueStorage::new();

        let mut init_result = unsafe { STORAGE.init() };
        assert!(init_result.is_ok());

        init_result = unsafe { STORAGE.init() };
        assert!(init_result.is_err());
        assert_eq!(
            init_result.err().unwrap(),
            InitError::StorageAlreadyInitialized
        );
    }

    #[test]
    fn create_handle() {
        static STORAGE: MessageQueueStorage<u8, 2> = MessageQueueStorage::new();

        let _ = unsafe { STORAGE.init() };

        let handle = STORAGE.create_handle();
        assert!(handle.is_some());
    }

    #[test]
    fn fail_create_handle_uninitialized() {
        static STORAGE: MessageQueueStorage<u8, 2> = MessageQueueStorage::new();

        let handle = STORAGE.create_handle();
        assert!(handle.is_none());
    }
}
