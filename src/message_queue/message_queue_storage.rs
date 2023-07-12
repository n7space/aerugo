//! Static storage for [message queue](crate::message_queue::MessageQueue).
//!
//! As this system cannot use dynamic memory allocation, all structures have to be allocated
//! statically. Per good practices user is separated from the actual implementation and instead
//! only has to provide a static memory (via this structure) where the MessageQueue will be allocated.

use super::MessageQueue;

use heapless::Vec;

use crate::aerugo::InitError;
use crate::arch::Mutex;
use crate::internal_cell::InternalCell;
use crate::message_queue::MessageQueueHandle;

/// Type of the queue buffer storage.
pub(crate) type QueueBuffer = Vec<u8, { core::mem::size_of::<MessageQueue<(), 0>>() }>;
/// Type of the queue data storage.
pub(crate) type QueueData<T, const N: usize> = heapless::spsc::Queue<T, N>;

/// Structure containing memory for MessageQueue creation.
///
/// * `T` - Type of the stored data.
/// * `N` - Size of the queue.
pub struct MessageQueueStorage<T, const N: usize> {
    /// Marks whether this storage has been initialized.
    initialized: InternalCell<bool>,
    /// Buffer for the queue structure.
    queue_buffer: InternalCell<QueueBuffer>,
    /// Buffer for the queue data.
    queue_data: Mutex<QueueData<T, N>>,
}

impl<T, const N: usize> MessageQueueStorage<T, N> {
    /// Creates new storage.
    pub const fn new() -> Self {
        MessageQueueStorage {
            initialized: InternalCell::new(false),
            queue_buffer: InternalCell::new(QueueBuffer::new()),
            queue_data: Mutex::new(QueueData::new()),
        }
    }

    /// Returns initialization status of this storage.
    pub fn is_initialized(&'static self) -> bool {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // the `init` function.
        unsafe { *self.initialized.as_ref() }
    }

    /// Creates new handle to a queue allocated in this storage.
    ///
    /// Returns `Some(handle)` if this storage has been initialized, `None` otherwise.
    pub fn create_handle(&'static self) -> Option<MessageQueueHandle<T, N>> {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // the `init` function.
        match unsafe { *self.initialized.as_ref() } {
            true => {
                // SAFETY: This is safe because storage has been initialized.
                let message_queue = unsafe { self.message_queue() };
                Some(MessageQueueHandle::new(message_queue))
            }
            false => None,
        }
    }

    /// Initializes this storage.
    ///
    /// TODO: Returns
    pub(crate) fn init(&'static self) -> Result<(), InitError> {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // this function.
        if unsafe { *self.initialized.as_ref() } {
            return Err(InitError::StorageAlreadyInitialized);
        }

        let queue = MessageQueue::<T, N>::new(&self.queue_data);

        // SAFETY: This is safe, because it is borrowed mutably only here. It can be modified
        // (initialized) only once, because it is guarded by the `initialized` field. No external
        // borrow can be made before initialization.
        unsafe {
            // Because the `MessageQueue` structure is of a constant size, the `queue_buffer` field is
            // statically allocated with enough memory for a 'placement new'.
            let queue_buffer =
                self.queue_buffer.as_mut_ref().as_mut_ptr() as *mut MessageQueue<T, N>;
            core::ptr::write(queue_buffer, queue);
        }

        // SAFETY: This is safe because it is only modified only here, and can't be externally
        // borrowed.
        unsafe {
            *self.initialized.as_mut_ref() = true;
        }

        Ok(())
    }

    /// Returns a reference to the stored MessageQueue structer.
    ///
    /// SAFETY: This is safe to call only when this storage has been initialized.
    #[inline(always)]
    unsafe fn message_queue(&'static self) -> &'static MessageQueue<T, N> {
        &*(self.queue_buffer.as_ref().as_ptr() as *const MessageQueue<T, N>)
    }
}
