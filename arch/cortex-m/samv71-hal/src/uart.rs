//! Module containing Universal Asynchronous Receiver/Transmitter (UART) HAL driver.
//!
//! Before using UART, make sure to
//! - Enable UART peripheral clock using PMC driver
//! - Set appropriate pins mode to peripheral mode using PIO driver
//! Consult the SAMV71 datasheet for more information.
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

use core::marker::PhantomData;

use self::metadata::UartMetadata;

pub mod metadata;

/// Structure representing UART driver
pub struct UART<Metadata: UartMetadata> {
    /// PAC UART instance metadata.
    _meta: PhantomData<Metadata>,
}

impl<Metadata: UartMetadata> UART<Metadata> {}
