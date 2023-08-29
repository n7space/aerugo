//! Module containing utility functions.

/// Const function for calculating max between two values.
pub(crate) const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}
