//! Module for system API traits.

/// Module for initialization API.
pub mod init_api;
/// Module for runtime API.
pub mod runtime_api;

pub use self::init_api::InitApi;
pub use self::runtime_api::RuntimeApi;
