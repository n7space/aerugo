//! Module with system errors.

pub mod runtime_error;
pub use self::runtime_error::RuntimeError;

mod system_error;
pub(crate) use self::system_error::SystemError;
