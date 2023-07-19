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
    /// # Parameters
    /// * `set_type` - Type of the condition set.
    pub const fn new(set_type: BooleanConditionSetType) -> Self {
        BooleanConditionSet {
            _set_type: set_type,
        }
    }

    /// Add a condition to the set.
    ///
    /// # Parameters
    /// * `_handle` - Handle to the condition.
    pub fn add(&mut self, _handle: BooleanConditionHandle) {
        todo!();
    }
}

/// Type of the boolean condition set
pub enum BooleanConditionSetType {
    /// All conditions in the set has to be true.
    And,
    /// At least one condition in the set has to be true.
    Or,
}
