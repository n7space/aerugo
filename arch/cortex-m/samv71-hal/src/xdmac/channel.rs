//! Implementation of XDMAC's channel.

use samv71q21_pac::{
    xdmac::{xdmac_chid::CIS, xdmac_chid::XDMAC_CHID as ChannelRegisters, RegisterBlock},
    XDMAC,
};

pub use super::interrupts::{ChannelInterrupts, InterruptStatus};

/// XDMAC channel.
///
/// Channels can be created only via [`Xdmac`](super::Xdmac). After acquiring a channel, it can be
/// used to configure an XDMAC transaction and manage it (start, stop, suspend, flush).
///
/// To check channel's status, you must use [`ChannelStatusReader`] instance that can be acquired
/// from Channel via [`Channel::take_status_reader`]. It can be taken only once - but can be
/// returned, and it must be present when giving the Channel back to [`Xdmac`](super::Xdmac), to
/// make sure that there's no dangling Reader after returning ownership of Channel.
///
/// This requirement may be ignored with `unsafe` variant of
/// [`Xdmac::return_channel`](super::Xdmac::return_channel):
/// [`Xdmac::mark_channel_as_free`](super::Xdmac::mark_channel_as_free). You can call this function
/// safely if you can guarantee that the Reader won't exist when Channel's ownership is returned.
///
/// # Safety
///
/// Most channel-related functions are safe. However, few functions that require a read-modify-write
/// operation on XDMAC global registers are unsafe, as using them without proper precautions (making
/// sure that these registers will not be accessed from another thread/IRQ for the operation's
/// duration) might cause data races.
///
/// As described in driver's module documentation, channels share global XDMAC registers, and proper
/// precautions must be taken if they must be shared between main thread and IRQs. For handling the
/// IRQs, you should use [`ChannelStatusReader`] along [`StatusReader`](super::status::StatusReader).
pub struct Channel {
    /// Pointer to channel's registers.
    _channel_registers: *const ChannelRegisters,
    /// Channel's numeric identifier.
    id: usize,
    /// Channel's status reader.
    status_reader: Option<ChannelStatusReader>,
}

impl Channel {
    /// Returns channel's ID.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Takes the status reader out of Channel.
    /// If the reader was already taken, and not put back, this function will return `None`.
    /// To return the status reader to the channel, use [`Channel::return_status_reader`].
    pub fn take_status_reader(&mut self) -> Option<ChannelStatusReader> {
        self.status_reader.take()
    }

    /// Returns the status reader back to the Channel.
    pub fn return_status_reader(&mut self, reader: ChannelStatusReader) {
        self.status_reader.replace(reader);
    }

    /// Returns `true` if status reader is currently stored in Channel.
    pub fn is_status_reader_available(&self) -> bool {
        self.status_reader.is_some()
    }

    /// Creates a new channel.
    pub(super) fn new(id: usize, registers: *const ChannelRegisters) -> Self {
        // Safety: This is safe, as channel registers pointer is provided by Xdmac (and therefore
        // valid).
        let status_register = &unsafe { &*registers }.cis;
        Self {
            id,
            _channel_registers: registers,
            status_reader: Some(ChannelStatusReader::new(id, status_register)),
        }
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

    /// Pointer to XDMAC's registers.
    const _XDMAC_REGISTERS: *const RegisterBlock = XDMAC::PTR;
}

/// Helper structure, use it to read XDMAC's channel status.
///
/// After getting it's instance from [`Channel`], you can use it to check which interrupts are
/// currently pending for this channel.
///
/// # Safety
///
/// **Reading the status register clears the flags inside it, so you should always handle pending
/// interrupts as soon as possible after the status is read.**
pub struct ChannelStatusReader {
    /// Channel's numeric identifier.
    id: usize,
    /// Pointer to channel's registers.
    channel_status_register: *const CIS,
}

impl ChannelStatusReader {
    /// Returns channel's pending interrupts.
    ///
    /// # Safety
    ///
    /// **Reading the status register clears the flags inside it, so you should always handle
    /// pending interrupts as soon as possible after the status is read.**
    pub fn get_pending_interrupts(&mut self) -> ChannelInterrupts {
        // Safety: This is safe, because pointer address is valid, as it's provided by Channel.
        unsafe { &*self.channel_status_register }.read().into()
    }

    /// Returns ID of the channel this reader belongs to.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Creates new instance of [`ChannelStatusReader`]. Should be called only by [`Channel`].
    /// You should never create it's instance manually.
    pub(super) fn new(id: usize, channel_status_register: *const CIS) -> Self {
        Self {
            id,
            channel_status_register,
        }
    }
}
