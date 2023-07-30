//! Handle to a boolean condition.
//!
//! This module contains boolean condition handle implementation, which is used to reference
//! a boolean condition in the system.

use super::BooleanCondition;

/// Boolean condition handle.
///
/// Boolean condition handle is available to the user of the system to reference and interact with the
/// condition via exposed interface. All system API functions shall use handles when a reference to
/// boolean condition is required.
pub struct BooleanConditionHandle {
    /// Reference to the boolean condition.
    condition: &'static BooleanCondition,
}

impl BooleanConditionHandle {
    /// Creates new condition handle.
    ///
    /// # Parameters
    /// * `condition` - Reference to the condition
    pub(crate) fn new(condition: &'static BooleanCondition) -> Self {
        BooleanConditionHandle { condition }
    }

    /// Gets value of the condition.
    pub fn get_value(&self) -> bool {
        self.condition.get_value()
    }

    /// Sets value of the condition.
    pub fn set_value(&self, value: bool) {
        self.condition.set_value(value)
    }
}
