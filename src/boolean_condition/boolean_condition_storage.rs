//! Static storage for [boolean condition](crate::boolean_condition::BooleanCondition).
//!
//! This module contains a boolean condition storage, which is a statically allocated memory that will
//! store condition structure for the duration of the system life.

use super::BooleanCondition;

use core::cell::OnceCell;

use heapless::Vec;

use crate::boolean_condition::BooleanConditionHandle;
use crate::error::SystemError;

/// Type of the boolean condition data storage.
type BooleanConditionBuffer = Vec<u8, { core::mem::size_of::<BooleanCondition>() }>;

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

/// SAFETY: It is safe assuming that BooleanConditionStorage is not modified in IRQ context and that
/// modification of the stored BooleanCondition cannot be interrupted.
///
/// BooleanConditionStorage is initialized only in
/// [create_boolean_condition](crate::api::InitApi::create_boolean_condition) implemented by
/// [Aerugo](crate::aerugo::Aerugo) which is not accessible from the IRQ context.
///
/// It's not possible to access the stored BooleanCondition with mutable reference, so safety of
/// BooleanCondition modification are subject of its implementation, which should disable interrupts
/// for the time of the mutable access. Interrupt can use some of the BooleanCondition functionalities
/// using [`BooleanConditionHandle`].
///
/// If any of those invariants are broken, then any usage can be considered unsafe.
unsafe impl Sync for BooleanConditionStorage {}

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

    /// Creates new handle to a boolean condition allocated in this storage.
    ///
    /// # Return
    /// `Some(handle)` if this storage has been initialized. `None` otherwise.
    pub fn create_handle(&'static self) -> Option<BooleanConditionHandle> {
        self.boolean_condition().map(BooleanConditionHandle::new)
    }

    /// Initializes this storage.
    ///
    /// # Parameters
    /// * `value` - Initial condition value.
    ///
    /// # Return
    /// `()` if successful, `SystemError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the stored condition buffer.
    /// This is safe to call during system initialization (before scheduler is started).
    /// Accessing storage from IRQ context during initialization is undefined behaviour.
    pub(crate) unsafe fn init(&'static self, value: bool) -> Result<(), SystemError> {
        if self.initialized.get().is_some() {
            return Err(SystemError::StorageAlreadyInitialized);
        }

        let condition = BooleanCondition::new(value);

        // This is safe, because `condition_buffer` doesn't contain any value yet, and it's size is
        // guaranteed to be large enough to store boolean condition structure.
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
            .expect("Failed to set BooleanConditionStorage initialization status");

        Ok(())
    }

    /// Returns a reference to the stored BooleanCondition structure.
    ///
    /// # Return
    /// `Some(condition)` if storage is initialized, `None` otherwise.
    #[inline(always)]
    fn boolean_condition(&'static self) -> Option<&'static BooleanCondition> {
        match (self.initialized.get(), self.condition_buffer.get()) {
            // SAFETY: This is safe, because the storage is initialized.
            (Some(_), Some(buffer)) => unsafe {
                Some(&*(buffer.as_ptr() as *const BooleanCondition))
            },
            (_, _) => None,
        }
    }
}

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
            SystemError::StorageAlreadyInitialized
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
