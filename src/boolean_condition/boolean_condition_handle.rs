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
    _condition: &'static BooleanCondition,
}
