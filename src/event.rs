//! System event.

mod event_handle;
mod event_storage;

pub use self::event_handle::EventHandle;
pub use self::event_storage::EventStorage;

pub(crate) struct Event {}
