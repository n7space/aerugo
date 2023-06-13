/// Handle to a boolean condition.
///
/// Handle is used in the system to access a boolean condition.
use super::BooleanCondition;

/// Boolean condition handle.
#[derive(Copy, Clone)]
pub struct BooleanConditionHandle {
    /// Reference to the boolean condition.
    _condition: &'static BooleanCondition,
}
