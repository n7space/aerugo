//! Implementation of XDMAC's channel.

use samv71q21_pac::xdmac::xdmac_chid::XDMAC_CHID as ChannelRegisters;

/// XDMAC channel.
pub struct Channel {
    /// Channel's numeric identifier.
    id: u8,
    /// Pointer to channel's registers
    _registers: *const ChannelRegisters,
}

impl Channel {
    /// Creates a new channel.
    pub(super) fn new(id: u8, registers: *const ChannelRegisters) -> Self {
        Self {
            id,
            _registers: registers,
        }
    }

    /// Returns channel's ID.
    pub fn id(&self) -> u8 {
        self.id
    }
}
