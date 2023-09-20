//! Implementation of HAL PIO driver.
//!
//! Parallel Input/Output (PIO) driver controls general purpose pins of SAMV71 MCU.
//! You can use it to configure the mode and state of I/O pins, or give them to other
//! peripherals for control.

/*
Implementation plan/ideas/issues:

PIO module contains Port and Pin types.
User should first create Port instance, which should consume PAC PIO port instance.
Port instance can be used as whole (for performance reasons), or be consumed to create
a structure with every pin as separate object.

*/

pub mod input_pin;
pub mod output_pin;
pub mod peripheral_pin;
pub mod pin;
pub mod port;
mod port_metadata;

pub use pin::Pin;
pub use port::Port;
