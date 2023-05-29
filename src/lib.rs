/*!
# aerugo

`aerugo` is a safety-critical applications oriented Real-Time Operating System.
*/
#![no_std]

mod aerugo;
mod api;
mod boolean_condition;
mod data_provider;
mod data_receiver;
mod event;
mod execution_monitoring;
mod internal_cell;
mod message_queue;
mod peripherals;
mod queue;
mod task;
mod tasklet;

pub use self::aerugo::{Aerugo, TaskletConfiguration};
pub use self::api::InitApi;
pub use self::boolean_condition::{BooleanConditionSet, BooleanConditionStorage};
pub use self::event::EventStorage;
pub use self::message_queue::MessageQueueStorage;
pub use self::peripherals::Peripherals;
pub use self::tasklet::TaskletStorage;

pub use fugit as time;
