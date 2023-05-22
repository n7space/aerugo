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
mod event;
mod internal_cell;
mod message_queue;
mod queue;
mod task;
mod tasklet;

pub use self::aerugo::Aerugo;
pub use self::api::InitApi;
pub use self::message_queue::MessageQueueStorage;
pub use self::tasklet::TaskletStorage;
