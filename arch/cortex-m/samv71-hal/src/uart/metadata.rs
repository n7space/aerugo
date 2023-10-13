//! Module containing meta-traits and their implementations for HAL UART driver
use crate::pac::uart0::RegisterBlock;
pub use crate::pac::{UART0, UART1, UART2, UART3, UART4};

/// Trait for PAC UART instances.
///
/// This trait erases the type of UART instance, so it can be used as
/// generic argument for [`UART`](super::UART) instead of concrete type.
pub trait UARTMetadata {
    /// Pointer to UART registers.
    const REGISTERS: *const RegisterBlock;

    /// Returns a reference to UART's register block.
    ///
    /// # Safety
    /// This function dereferences a raw pointer.
    /// It's safe to use, as long as there aren't multiple instances of the same UART peripheral.
    #[inline(always)]
    fn registers() -> &'static RegisterBlock {
        unsafe { &*Self::REGISTERS }
    }
}

/// Internal macro used to generate UartMetadata implementations for every available UART.
macro_rules! implement_uart_metadata_for {
    ($uart:ty) => {
        impl UARTMetadata for $uart {
            const REGISTERS: *const RegisterBlock = <$uart>::PTR;
        }
    };
}

implement_uart_metadata_for!(UART0);
implement_uart_metadata_for!(UART1);
implement_uart_metadata_for!(UART2);
implement_uart_metadata_for!(UART3);
implement_uart_metadata_for!(UART4);
