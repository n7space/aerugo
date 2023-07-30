//! Boolean condition.

mod boolean_condition_handle;
mod boolean_condition_set;
mod boolean_condition_storage;

pub use self::boolean_condition_handle::BooleanConditionHandle;
pub use self::boolean_condition_set::BooleanConditionSet;
pub use self::boolean_condition_storage::BooleanConditionStorage;

use crate::arch::Mutex;

/// Boolean condition.
#[repr(C)]
pub(crate) struct BooleanCondition {
    /// Condition value.
    value: Mutex<bool>,
}

impl BooleanCondition {
    /// Creates new `BooleanCondition`
    pub(crate) fn new(value: bool) -> Self {
        BooleanCondition {
            value: Mutex::new(value),
        }
    }

    /// Gets value of the condition.
    pub fn get_value(&self) -> bool {
        self.value.lock(|v| *v)
    }

    /// Sets value of the condition.
    pub fn set_value(&self, value: bool) {
        self.value.lock(|v| *v = value)
    }
}
