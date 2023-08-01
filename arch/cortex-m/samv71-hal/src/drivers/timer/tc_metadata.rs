//! Module with PAC TC metadata implementation.
pub(super) use pac::tc0::RegisterBlock;
use pac::{Interrupt, TC0};

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
