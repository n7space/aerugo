//! Boolean condition set.

use heapless::Vec;

use crate::aerugo::error::InitError;
use crate::boolean_condition::{BooleanCondition, BooleanConditionHandle};
use crate::tasklet::TaskletPtr;

/// Type of the set conditions list.
type ConditionsList<const N: usize> = Vec<&'static BooleanCondition, N>;

/// Set of boolean conditions.
pub struct BooleanConditionSet<const N: usize> {
    /// Type of the set.
    set_type: BooleanConditionSetType,
    /// Set conditions.
    conditions: ConditionsList<N>,
}

impl<const N: usize> BooleanConditionSet<N> {
    /// Creates new condition set of given type.
    ///
    /// # Parameters
    /// * `set_type` - Type of the condition set.
    pub fn new(set_type: BooleanConditionSetType) -> Self {
        BooleanConditionSet {
            set_type,
            conditions: ConditionsList::new(),
        }
    }

    /// Creates new condition set from array.
    ///
    /// # Parameters
    /// * `condition` - Conditions array.
    /// * `set_type` - Type of the condition set.
    pub fn from_array(
        conditions: [&BooleanConditionHandle; N],
        set_type: BooleanConditionSetType,
    ) -> Self {
        BooleanConditionSet {
            set_type,
            conditions: ConditionsList::from_slice(&conditions.map(|handle| handle.condition()))
                .unwrap(),
        }
    }

    /// Add a condition to the set.
    ///
    /// # Parameters
    /// * `handle` - Handle to the condition.
    pub fn add(&mut self, handle: &BooleanConditionHandle) -> Result<(), BooleanConditionSetError> {
        match self.conditions.push(handle.condition()) {
            Ok(_) => Ok(()),
            Err(_) => Err(BooleanConditionSetError::SetFull),
        }
    }

    /// Registers tasklet to each condition in this set.
    ///
    /// # Parameter
    /// * `tasklet` - Tasklet to register
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This is unsafe, because it mutably borrows the list of registered tasklets.
    /// This is safe to call before the system initialization.
    pub(crate) unsafe fn register_tasklet(&self, tasklet: TaskletPtr) -> Result<(), InitError> {
        for cond in &self.conditions {
            cond.register_tasklet(tasklet.clone())?;
        }

        Ok(())
    }

    /// Evaluates value of this condition set.
    pub(crate) fn evaluate(&self) -> bool {
        match self.set_type {
            BooleanConditionSetType::And => self.evaluate_and(),
            BooleanConditionSetType::Or => self.evaluate_or(),
        }
    }

    /// Evaluates value of this condition set for `and` type.
    fn evaluate_and(&self) -> bool {
        self.conditions.iter().all(|cond| cond.get_value())
    }

    /// Evaluates value of this condition set for `or` type.
    fn evaluate_or(&self) -> bool {
        self.conditions.iter().any(|cond| cond.get_value())
    }
}

impl Default for BooleanConditionSet<1> {
    /// Creates new condition set with one element.
    fn default() -> Self {
        BooleanConditionSet {
            set_type: BooleanConditionSetType::And,
            conditions: ConditionsList::new(),
        }
    }
}

impl From<BooleanConditionHandle> for BooleanConditionSet<1> {
    /// Creates new condition set with given condition.
    fn from(handle: BooleanConditionHandle) -> Self {
        BooleanConditionSet {
            set_type: BooleanConditionSetType::And,
            conditions: ConditionsList::from_slice(&[handle.condition()]).unwrap(),
        }
    }
}

/// Type of the boolean condition set
pub enum BooleanConditionSetType {
    /// All conditions in the set has to be true.
    And,
    /// At least one condition in the set has to be true.
    Or,
}

/// Boolean condition set errors
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BooleanConditionSetError {
    /// Added condition to a full set.
    SetFull,
}
