//! Boolean condition.

mod boolean_condition_handle;
mod boolean_condition_storage;

pub use self::boolean_condition_handle::BooleanConditionHandle;
pub use self::boolean_condition_storage::BooleanConditionStorage;

/// Boolean condition.
pub(crate) struct BooleanCondition {}

/// Set of boolean conditions.
pub struct BooleanConditionSet {
    /// Type of the set.
    _set_type: BooleanConditionSetType,
}

impl BooleanConditionSet {
    /// Creates new condition set.
    ///
    /// * `set_type` - Type of the condition set.
    pub const fn new(set_type: BooleanConditionSetType) -> Self {
        BooleanConditionSet {
            _set_type: set_type,
        }
    }

    /// Add a condition to the set.
    ///
    /// * `_handle` - Handle to the condition.
    pub fn add(&mut self, _handle: BooleanConditionHandle) {
        todo!();
    }
}

/// Type of the boolean condition set
pub enum BooleanConditionSetType {
    And,
    Or,
}
