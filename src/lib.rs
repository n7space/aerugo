/*!
# aerugo

`aerugo` is a safety-critical applications oriented Real-Time Operating System.
*/
#![no_std]

mod aerugo;
mod api;
mod crit_cell;
mod data_provider;
mod data_receiver;
mod internal_cell;
mod notifier;
mod queue;
mod task;

pub use self::aerugo::Aerugo;
pub use self::api::InitApi;
pub use self::crit_cell::CritCell;
pub use self::queue::MessageQueueStorage;
pub use self::task::TaskletStorage;
