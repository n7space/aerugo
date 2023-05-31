//! System event.

/// Module for Event handle.
mod event_handle;
/// Module for Event storage.
mod event_storage;

pub use self::event_handle::EventHandle;
pub use self::event_storage::EventStorage;

/// System event.
pub(crate) struct Event {}
