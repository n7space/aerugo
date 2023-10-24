//! Implementation of XDMAC's channel.

use samv71q21_pac::{
    xdmac::{xdmac_chid::XDMAC_CHID as ChannelRegisters, RegisterBlock},
    XDMAC,
};

/// XDMAC channel.
///
/// Channels can be created only via [`Xdmac`](super::Xdmac). After acquiring a channel, it can be
/// used to configure an XDMAC transaction and manage it (start, stop, suspend, flush).
///
/// # Safety
///
/// Most channel-related functions are safe. However, few functions that require a read-modify-write
/// operation on XDMAC global registers are unsafe, as using them without proper precautions (making
/// sure that these registers will not be accessed from another thread/IRQ for the operation's
/// duration) might cause data races.
pub struct Channel {
    /// Pointer to channel's registers.
    _channel_registers: *const ChannelRegisters,
    /// Channel's numeric identifier.
    id: usize,
}

impl Channel {
    /// Pointer to XDMAC's registers.
    const _XDMAC_REGISTERS: *const RegisterBlock = XDMAC::PTR;

    /// Creates a new channel.
    pub(super) fn new(id: usize, registers: *const ChannelRegisters) -> Self {
        Self {
            id,
            _channel_registers: registers,
        }
    }

    /// Returns channel's ID.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns a reference to channel's registers.
    fn _channel_registers_ref(&self) -> &ChannelRegisters {
        // Safety: This is safe, as the address of the register is guaranteed to be valid by Xdmac.
        unsafe { &*self._channel_registers }
    }

    /// Returns a reference to XDMAC's registers.
    fn _xdmac_registers_ref(&self) -> &RegisterBlock {
        // Safety: This is safe, as the address of XDMAC register is guaranteed to be valid by Xdmac.
        unsafe { &*Self::_XDMAC_REGISTERS }
    }
}
