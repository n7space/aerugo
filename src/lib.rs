/*!
# aerugo

`aerugo` is a safety-critical applications oriented Real-Time Operating System.
*/
#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_crate_level_docs)]

extern crate internal_cell;

mod aerugo;
mod api;
mod boolean_condition;
mod data_provider;
mod event;
mod event_manager;
mod execution_monitoring;
mod executor;
mod message_queue;
mod stubs;
mod tasklet;
mod time_manager;
mod time_source;
mod utils;

pub use self::aerugo::{Aerugo, AERUGO};
pub use self::api::{InitApi, RuntimeApi};
pub use self::boolean_condition::{
    BooleanConditionHandle, BooleanConditionSet, BooleanConditionSetType, BooleanConditionStorage,
};
pub use self::event::{EventEnabler, EventId};
pub use self::message_queue::{MessageQueueHandle, MessageQueueStorage};
pub use self::tasklet::{TaskletConfig, TaskletStorage};
pub use aerugo_hal::{time, SystemDuration, SystemHardwareConfig, SystemInstant};

#[cfg(feature = "use-aerugo-cortex-m")]
pub(crate) use aerugo_cortex_m as arch;
#[cfg(feature = "use-aerugo-cortex-m")]
pub use aerugo_samv71_hal as hal;

#[cfg(feature = "use-aerugo-x86")]
pub(crate) use aerugo_x86 as arch;
#[cfg(feature = "use-aerugo-x86")]
pub use aerugo_x86_hal as hal;

#[cfg(feature = "log")]
pub use arch::{log, logln};
