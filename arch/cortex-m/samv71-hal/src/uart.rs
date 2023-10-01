//! Implementation of Universal Asynchronous Receiver/Transmitter (UART) HAL driver.
//!
//! Before using UART, make sure to
//! - Enable UART peripheral clock using PMC driver
//! - Set appropriate pins mode to peripheral mode using PIO driver
//! Consult the SAMV71 datasheet and [Safety](#safety) section for details.
//!
//! This driver allows you to configure and use any available UART driver in safe way.
//! Currently, the driver supports:
//! - Baudrate and source clock configuration
//! - Transmitter and receiver status checks and configuration
//! - Mode configuration (normal/echo/loopback)
//! - Parity configuration
//! - Parity/overrun/framing error detection
//! - Interrupt configuration
//! - Digital filter configuration
//!
//! Currently, it does NOT support:
//! - Comparison configuration
//! - Sleepwalking mode
//!
//! # Safety
//! UART can be driven by either peripheral or programmable clock.
//! If programmable clock (PCK4) is selected, the baud rate is independent of the
//! processor/bus clock, thus the processor clock can be changed while UART is enabled.
//! However, only setting of processor clock that can be changed while UART is enabled is
//! Main Clock (MCK) prescaler. Other methods to modify the processor/bus clock frequency
//! (PLL multiplier, etc.) are forbidden when UART is enabled.
//!
//! The peripheral clock frequency must be at least three times higher than PCK.

use core::marker::PhantomData;

use self::metadata::UartMetadata;

pub mod interrupt;
pub mod metadata;
pub mod mode;
pub mod status;

/// Structure representing UART driver
pub struct UART<Metadata: UartMetadata> {
    /// PAC UART instance metadata.
    _meta: PhantomData<Metadata>,
}

impl<Metadata: UartMetadata> UART<Metadata> {}
