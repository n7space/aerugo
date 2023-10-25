//! Implementation of XDMAC's [`Channel`](super::channel::Channel) status reader.

use samv71q21_pac::xdmac::xdmac_chid::CIS;

pub use super::events::ChannelEvents;

/// Helper structure, use it to read XDMAC's channel status.
///
/// After getting it's instance from [`Channel`](super::Channel), you can use it to check which interrupts are
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
    pub fn get_pending_interrupts(&mut self) -> ChannelEvents {
        // Safety: This is safe, because pointer address is valid, as it's provided by Channel.
        unsafe { &*self.channel_status_register }.read().into()
    }

    /// Returns ID of the channel this reader belongs to.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Creates new instance of [`ChannelStatusReader`]. Should be called only by
    /// [`Channel`](super::Channel).
    /// You should never create it's instance manually.
    pub(super) fn new(id: usize, channel_status_register: *const CIS) -> Self {
        Self {
            id,
            channel_status_register,
        }
    }
}
