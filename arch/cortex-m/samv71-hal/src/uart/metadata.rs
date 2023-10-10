//! Module containing meta-traits and their implementations for HAL UART driver
pub(super) use crate::pac::uart0::RegisterBlock;
pub use crate::pac::{UART0, UART1, UART2, UART3, UART4};

/// Trait for PAC UART instances.
///
/// This trait erases the type of UART instance, so it can be used as
/// generic argument for [`UART`](super::UART) instead of concrete type.
pub trait UartMetadata {
    /// Pointer to UART registers.
    const REGISTERS: *const RegisterBlock;
}

/// Internal macro used to generate UartMetadata implementations for every available UART.
macro_rules! implement_uart_metadata_for {
    ($uart:ty) => {
        impl UartMetadata for $uart {
            const REGISTERS: *const RegisterBlock = <$uart>::PTR;
        }
    };
}

implement_uart_metadata_for!(UART0);
implement_uart_metadata_for!(UART1);
implement_uart_metadata_for!(UART2);
implement_uart_metadata_for!(UART3);
implement_uart_metadata_for!(UART4);
