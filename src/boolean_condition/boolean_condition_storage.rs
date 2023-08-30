//! Static storage for [boolean condition](crate::boolean_condition::BooleanCondition).
//!
//! This module contains a boolean condition storage, which is a statically allocated memory that will
//! store condition structure for the duration of the system life.

use super::BooleanCondition;

use core::cell::OnceCell;

use heapless::Vec;

use crate::api::InitError;
use crate::boolean_condition::BooleanConditionHandle;

/// Type of the queue data storage.
pub(crate) type BooleanConditionBuffer = Vec<u8, { core::mem::size_of::<BooleanCondition>() }>;

/// Structure containing memory for BooleanCondition creation.
///
/// As this system cannot use dynamic memory allocation, all structures have to be allocated
/// statically. Per good practices user is separated from the actual implementation and instead
/// only has to provide a static memory (via this structure) where the BooleanCondition will be allocated.
pub struct BooleanConditionStorage {
    /// Marks whether this storage has been initialized.
    initialized: OnceCell<()>,
    /// Buffer for the condition structure.
    condition_buffer: OnceCell<BooleanConditionBuffer>,
}

impl BooleanConditionStorage {
    /// Creates new storage.
    pub const fn new() -> Self {
        BooleanConditionStorage {
            initialized: OnceCell::new(),
            condition_buffer: OnceCell::new(),
        }
    }

    /// Returns initialization status of this storage.
    pub fn is_initialized(&'static self) -> bool {
        self.initialized.get().is_some()
    }

    /// Creates new handle to a boolean condition allocated in ths storage.
    ///
    /// # Return
    /// `Some(handle)` if this storage has been initialized. `None` otherwise.
    pub fn create_handle(&'static self) -> Option<BooleanConditionHandle> {
        match self.initialized.get() {
            Some(_) => {
                let boolean_condition = self
                    .boolean_condition()
                    .expect("Failed to get reference to the stored BooleanCondition");
                Some(BooleanConditionHandle::new(boolean_condition))
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
    /// This is unsafe, because it mutably borrows the stored condition buffer.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn init(&'static self, value: bool) -> Result<(), InitError> {
        if self.initialized.get().is_some() {
            return Err(InitError::StorageAlreadyInitialized);
        }

        let condition = BooleanCondition::new(value);

        // This is safe, because `condition_buffer` doesn't contain any value yet, and it's size is
        // guaranteed to be large enough to store queue structure.
        let condition_buffer = BooleanConditionBuffer::new();
        unsafe {
            let condition_buffer_ptr = condition_buffer.as_ptr() as *mut BooleanCondition;
            core::ptr::write(condition_buffer_ptr, condition);
        }

        self.condition_buffer
            .set(condition_buffer)
            .expect("Failed to initialize BooleanConditionStorage buffer");

        self.initialized
            .set(())
            .expect("Failed to initialize BooleanConditionStorage");

        Ok(())
    }

    /// Returns a reference to the stored BooleanCondition structure.
    #[inline(always)]
    fn boolean_condition(&'static self) -> Option<&'static BooleanCondition> {
        match self.condition_buffer.get() {
            Some(buffer) => {
                // This is safe, because buffer is initialized
                unsafe { Some(&*(buffer.as_ptr() as *const BooleanCondition)) }
            }
            None => None,
        }
    }
}

unsafe impl Sync for BooleanConditionStorage {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        static STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

        assert!(!STORAGE.is_initialized());
    }

    #[test]
    fn initialize() {
        static STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

        let init_result = unsafe { STORAGE.init(true) };
        assert!(init_result.is_ok());
        assert!(STORAGE.is_initialized());
    }

    #[test]
    fn fail_double_initialization() {
        static STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

        let init_result = unsafe { STORAGE.init(true) };
        assert!(init_result.is_ok());
        let init_result = unsafe { STORAGE.init(false) };

        assert!(init_result.is_err());
        assert_eq!(
            init_result.err().unwrap(),
            InitError::StorageAlreadyInitialized
        );
    }

    #[test]
    fn create_handle() {
        static STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

        let _ = unsafe { STORAGE.init(true) };

        let handle = STORAGE.create_handle();
        assert!(handle.is_some());
    }

    #[test]
    fn fail_create_handle_uninitialized() {
        static STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();

        let handle = STORAGE.create_handle();
        assert!(handle.is_none());
    }
}
