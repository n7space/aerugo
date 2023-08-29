//! Module containing stubs with generic implementations of optional functions

/// No-op `log` that replaces the actual implementation when `log` feature is disabled
#[macro_export]
#[cfg(not(feature = "log"))]
macro_rules! log {
    ($($arg:tt)*) => {{}};
}

/// No-op `logln` that replaces the actual implementation when `log` feature is disabled
#[macro_export]
#[cfg(not(feature = "log"))]
macro_rules! logln {
    () => {};
    ($($arg:tt)*) => {{}};
}
