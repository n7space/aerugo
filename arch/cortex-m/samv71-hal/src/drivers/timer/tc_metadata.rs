//! Module with PAC TC metadata implementation.
pub(super) use pac::tc0::RegisterBlock;
use pac::{Interrupt, TC0, TC1, TC2, TC3};

/// Amount of channels per timer instance.
const CHANNELS_COUNT_PER_TIMER: usize = 3;

/// Trait for PAC timer counter instances.
/// This trait erases the type of TC instance, so it can be used as generic argument for [`Timer`](super::Timer)
pub trait TcMetadata {
    /// Pointer to Timer Counter's registers block.
    const REGISTERS: *const RegisterBlock;
    /// Interrupt ID of each channel.
    const INTERRUPTS: [Interrupt; CHANNELS_COUNT_PER_TIMER];
}

impl TcMetadata for TC0 {
    const REGISTERS: *const RegisterBlock = TC0::PTR;
    const INTERRUPTS: [Interrupt; CHANNELS_COUNT_PER_TIMER] =
        [Interrupt::TC0, Interrupt::TC1, Interrupt::TC2];
}

impl TcMetadata for TC1 {
    const REGISTERS: *const RegisterBlock = TC1::PTR;
    const INTERRUPTS: [Interrupt; CHANNELS_COUNT_PER_TIMER] =
        [Interrupt::TC3, Interrupt::TC4, Interrupt::TC5];
}

impl TcMetadata for TC2 {
    const REGISTERS: *const RegisterBlock = TC2::PTR;
    const INTERRUPTS: [Interrupt; CHANNELS_COUNT_PER_TIMER] =
        [Interrupt::TC6, Interrupt::TC7, Interrupt::TC8];
}

impl TcMetadata for TC3 {
    const REGISTERS: *const RegisterBlock = TC3::PTR;
    const INTERRUPTS: [Interrupt; CHANNELS_COUNT_PER_TIMER] =
        [Interrupt::TC9, Interrupt::TC10, Interrupt::TC11];
}
