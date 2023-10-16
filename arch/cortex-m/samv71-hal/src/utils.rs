//! Utility functions that provide abstraction over common HAL operations.

/// Waits until provided functor returns `true`.
///
/// # Parameters
/// * `check_function` - Functor that returns `true` when operation finishes, and `false` otherwise.
/// * `max_checks_amount` - Maximum amount of `functor` executions.
///
/// # Returns
/// `Some(u32)`, with amount of checks left before "timeout", or `None` if maximum checks amount
/// has been reached.
pub fn wait_until<F>(check_function: F, max_checks_amount: u32) -> Option<u32>
where
    F: Fn() -> bool,
{
    for wasted_cycles in 0..max_checks_amount {
        if check_function() {
            return Some(max_checks_amount - wasted_cycles);
        }
    }

    None
}
