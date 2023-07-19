//! Static storage for [boolean condition](crate::boolean_condition::BooleanCondition).
//!
//! As this system cannot use dynamic memory allocation, all structures have to be allocated
//! statically. Per good practices user is separated from the actual implementation and instead
//! only has to provide a static memory (via this structure) where the BooleanCondition will be allocated.

use super::BooleanCondition;

use heapless::Vec;

use crate::boolean_condition::BooleanConditionHandle;
use crate::internal_cell::InternalCell;

/// Type of the queue data storage.
pub(crate) type BooleanConditionBuffer = Vec<u8, { core::mem::size_of::<BooleanCondition>() }>;

/// Structure containing memory for BooleanCondition creation.
pub struct BooleanConditionStorage {
    /// Marks whether this storage has been initialized.
    _initialized: InternalCell<bool>,
    /// Buffer for the condition structure.
    _condition_buffer: InternalCell<BooleanConditionBuffer>,
}

impl BooleanConditionStorage {
    /// Creates new storage.
    pub const fn new() -> Self {
        BooleanConditionStorage {
            _initialized: InternalCell::new(false),
            _condition_buffer: InternalCell::new(BooleanConditionBuffer::new()),
        }
    }

    /// Creates new handle to a boolean condition allocated in ths storage.
    ///
    /// # Return
    /// `Some(handle)` if this storage has been initialized. `None` otherwise.
    pub fn create_condition_handle(&'static self) -> Option<BooleanConditionHandle> {
        todo!()
    }
}
