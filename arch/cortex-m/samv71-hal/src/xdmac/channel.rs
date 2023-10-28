//! Implementation of XDMAC's channel.

use samv71q21_pac::{
    xdmac::{xdmac_chid::XDMAC_CHID as ChannelRegisters, RegisterBlock},
    XDMAC,
};

pub use super::channel_status::ChannelStatusReader;
pub use super::events::ChannelEvents;

/// XDMAC channel.
///
/// Channels can be created only via [`Xdmac`](super::Xdmac). After acquiring a channel, it can be
/// used to configure an XDMAC transaction and manage it (start, stop, suspend, flush).
///
/// To check channel's status, you must use [`ChannelStatusReader`] instance that can be acquired
/// from Channel via [`Channel::take_status_reader`]. It can be taken only once - but can be
/// returned, and it must be present when giving the Channel back to [`Xdmac`](super::Xdmac), to
/// make sure that there's no dangling Reader after returning ownership of a Channel.
///
/// This requirement may be ignored with `unsafe` variant of
/// [`Xdmac::return_channel`](super::Xdmac::return_channel):
/// [`Xdmac::mark_channel_as_free`](super::Xdmac::mark_channel_as_free). You can call this function
/// safely if you can guarantee that the Reader won't exist when Channel's ownership is returned.
///
/// In order to configure an XDMAC transfer, you must create transfer block, and pass it to the
/// Channel. Then, you can start the transaction by enabling the channel.
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
    channel_registers: *const ChannelRegisters,
    /// Channel's numeric identifier.
    id: usize,
    /// Channel's status reader.
    status_reader: Option<ChannelStatusReader>,
}

impl Channel {
    /// Returns `true` if Channel is currently enabled and XDMAC transaction is in progress.
    pub fn is_busy(&self) -> bool {
        self.is_channels_bit_set(self.xdmac_registers_ref().gs.read().bits())
    }

    /// Enables the channel and starts the transaction, if the channel is not busy.
    ///
    /// # Returns
    ///
    /// `true` if the channel was successfully enabled. `false` if the channel is busy.
    pub fn enable(&mut self) -> bool {
        if self.is_busy() {
            return false;
        }

        self.xdmac_registers_ref()
            .ge
            // Safety: This is safe, because channel's ID must be valid for a Channel to exist.
            .write(|w| unsafe { w.bits(self.channel_bitmask()) });
        true
    }

    /// Disables the channel, stopping ongoing transaction (if any is in progress).
    ///
    /// If the channel is performing peripheral-to-memory transaction, the pending bytes from XDMAC
    /// FIFO are written to destination memory. Otherwise, the transaction is terminated immediately.
    ///
    /// Hardware-synchronized transactions automatically disable the channel when they're completed.
    pub fn disable(&mut self) {
        self.xdmac_registers_ref()
            .gd
            .write(|w| unsafe { w.bits(self.channel_bitmask()) });
    }

    /// Enables channel's global interrupt.
    ///
    /// While channel's global interrupt is enabled, IRQ will be triggered when one of the enabled
    /// channel events happens.
    pub fn enable_interrupt(&mut self) {
        self.xdmac_registers_ref()
            .gie
            // Safety: This is safe, because channel's ID must be valid for a Channel to exist.
            .write(|w| unsafe { w.bits(self.channel_bitmask()) });
    }

    /// Disables channel's global interrupt.
    ///
    /// While channel's global interrupt is disabled, IRQ will **not** be triggered when one of the
    /// enabled channel events happens.
    pub fn disable_interrupt(&mut self) {
        self.xdmac_registers_ref()
            .gid
            // Safety: This is safe, because channel's ID must be valid for a Channel to exist.
            .write(|w| unsafe { w.bits(self.channel_bitmask()) });
    }

    /// Returns `true` if channel's global interrupt is enabled.
    pub fn is_interrupt_enabled(&self) -> bool {
        self.is_channels_bit_set(self.xdmac_registers_ref().gim.read().bits())
    }

    /// Sets channel events state (enabled/disabled). Channel events are usually handled via IRQs,
    /// make sure to enable channel's global interrupt using [`Channel::enable_interrupt`] if you
    /// indent to do that.
    pub fn set_events_state(&mut self, events_state: ChannelEvents) {
        self.channel_registers_ref().cie.write(|w| {
            w.roie()
                .bit(events_state.request_overflow_error)
                .wbie()
                .bit(events_state.write_bus_error)
                .rbie()
                .bit(events_state.read_bus_error)
                .fie()
                .bit(events_state.end_of_flush)
                .die()
                .bit(events_state.end_of_disable)
                .lie()
                .bit(events_state.end_of_list)
                .bie()
                .bit(events_state.end_of_block)
        });

        self.channel_registers_ref().cid.write(|w| {
            w.roid()
                .bit(!events_state.request_overflow_error)
                .wbeid()
                .bit(!events_state.write_bus_error)
                .rbeid()
                .bit(!events_state.read_bus_error)
                .fid()
                .bit(!events_state.end_of_flush)
                .did()
                .bit(!events_state.end_of_disable)
                .lid()
                .bit(!events_state.end_of_list)
                .bid()
                .bit(!events_state.end_of_block)
        });
    }

    /// Returns channel events state (enabled/disabled).
    pub fn events_state(&self) -> ChannelEvents {
        self.channel_registers_ref().cim.read().into()
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

    /// Requests a DMA transfer for this channel, if the request is not pending already.
    /// This function must be used to trigger a software-synchronized DMA transfer.
    ///
    /// # Returns
    ///
    /// `true` if channel was successfully triggered, `false` if a request is already pending.
    pub fn trigger(&mut self) {
        if !self.is_software_request_pending() {
            // Safety: This is safe, because we just verified that a request is not pending.
            unsafe { self.force_trigger() };
        }
    }

    /// Returns `true` if a software request is currently pending on the channel.
    pub fn is_software_request_pending(&self) -> bool {
        self.is_channels_bit_set(self.xdmac_registers_ref().gsws.read().bits())
    }

    /// Requests a DMA transfer for this channel.
    ///
    /// # Safety
    ///
    /// This function does not check whether a software request is currently pending, or not.
    /// If you want a safe function that performs that check automatically, use [`Channel::trigger`].
    pub unsafe fn force_trigger(&mut self) {
        self.xdmac_registers_ref()
            .gswr
            // Safety: This is safe, because channel's ID must be valid for a Channel to exist.
            .write(|w| unsafe { w.bits(self.channel_bitmask()) });
    }

    /// Flushes the channel, if the channel is peripheral-synchronized.
    /// Otherwise, does nothing.
    pub fn flush(&mut self) {
        if self.is_peripheral_synchronized() {
            self.xdmac_registers_ref()
                .gswf
                // Safety: This is safe, because channel's ID must be valid for a Channel to exist.
                .write(|w| unsafe { w.bits(self.channel_bitmask()) });
        }
    }

    /// Returns `true` if source requests for this channel are suspended.
    pub fn is_read_suspended(&self) -> bool {
        self.is_channels_bit_set(self.xdmac_registers_ref().grs.read().bits())
    }

    /// Returns `true` if destination requests for this channel are suspended.
    pub fn is_write_suspended(&self) -> bool {
        self.is_channels_bit_set(self.xdmac_registers_ref().gws.read().bits())
    }

    /// Suspends source requests for the channel.
    /// Source requests for this channel are no longer serviced by the system scheduler.
    ///
    /// # Safety
    ///
    /// This is a read-modify-write operation that uses global XDMAC registers. Be very careful
    /// with that if you share the Channels between threads/IRQs.
    pub fn suspend_read(&mut self) {
        self.xdmac_registers_ref()
            .grs
            // Safety: This is safe, because channel's ID must be valid for a Channel to exist.
            // Also, channel bits are correctly masked with old value, as this is an R-M-W operation.
            .modify(|r, w| unsafe { w.bits(r.bits() & self.channel_bitmask()) });
    }

    /// Suspends destination requests for the channel.
    /// Destination requests for this channel are no longer routed to the scheduler.
    ///
    /// # Safety
    ///
    /// This is a read-modify-write operation that uses global XDMAC registers. Be very careful
    /// with that if you share the Channels between threads/IRQs.
    pub fn suspend_write(&mut self) {
        self.xdmac_registers_ref()
            .gws
            // Safety: This is safe, because channel's ID must be valid for a Channel to exist.
            // Also, channel bits are correctly masked with old value, as this is an R-M-W operation.
            .modify(|r, w| unsafe { w.bits(r.bits() & self.channel_bitmask()) });
    }

    /// Suspends read and write operations at the same time.
    pub fn suspend_read_and_write(&mut self) {
        self.xdmac_registers_ref()
            .grws
            // Safety: This is safe, because channel's ID must be valid for a Channel to exist.
            .write(|w| unsafe { w.bits(self.channel_bitmask()) });
    }

    /// Resumes source requests for the channel.
    ///
    /// # Safety
    ///
    /// This is a read-modify-write operation that uses global XDMAC registers. Be very careful
    /// with that if you share the Channels between threads/IRQs.
    pub fn resume_read(&mut self) {
        self.xdmac_registers_ref()
            .grs
            // Safety: This is safe, because channel's ID must be valid for a Channel to exist.
            // Also, channel bits are correctly masked with old value, as this is an R-M-W operation.
            // Channel bitmask must be negated, as this operation is supposed to clear the channel bit.
            .modify(|r, w| unsafe { w.bits(r.bits() & !self.channel_bitmask()) });
    }

    /// Resumes destination requests for the channel.
    ///
    /// # Safety
    ///
    /// This is a read-modify-write operation that uses global XDMAC registers. Be very careful
    /// with that if you share the Channels between threads/IRQs.
    pub fn resume_write(&mut self) {
        self.xdmac_registers_ref()
            .gws
            // Safety: This is safe, because channel's ID must be valid for a Channel to exist.
            // Also, channel bits are correctly masked with old value, as this is an R-M-W operation.
            // Channel bitmask must be negated, as this operation is supposed to clear the channel bit.
            .modify(|r, w| unsafe { w.bits(r.bits() & !self.channel_bitmask()) });
    }

    /// Resumes read and write operations at the same time.
    pub fn resume_read_and_write(&mut self) {
        self.xdmac_registers_ref()
            .grwr
            // Safety: This is safe, because channel's ID must be valid for a Channel to exist.
            .write(|w| unsafe { w.bits(self.channel_bitmask()) });
    }

    /// Returns channel's ID.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Creates a new channel.
    pub(super) fn new(id: usize, registers: *const ChannelRegisters) -> Self {
        // Safety: This is safe, as channel registers pointer must be provided by Xdmac driver (and
        // therefore be valid).
        let status_register = &unsafe { &*registers }.cis;
        Self {
            id,
            channel_registers: registers,
            status_reader: Some(ChannelStatusReader::new(id, status_register)),
        }
    }

    /// Returns `true` if the channel is currently configured as peripheral-synchronized.
    fn is_peripheral_synchronized(&self) -> bool {
        self.channel_registers_ref().cc.read().type_().is_per_tran()
    }

    /// Returns a reference to channel's registers.
    #[inline(always)]
    fn channel_registers_ref(&self) -> &ChannelRegisters {
        // Safety: This is safe, as the address of the register is guaranteed to be valid by Xdmac.
        unsafe { &*self.channel_registers }
    }

    /// Returns a reference to XDMAC's registers.
    #[inline(always)]
    fn xdmac_registers_ref(&self) -> &RegisterBlock {
        // Safety: This is safe, as the address of XDMAC register is guaranteed to be valid by Xdmac.
        unsafe { &*Self::XDMAC_REGISTERS }
    }

    /// Returns channel's bitmask (`1` shifted by `n` bits, where `n` is channel's ID)
    /// This function will return valid value as long, as channel's ID is also valid.
    #[inline(always)]
    fn channel_bitmask(&self) -> u32 {
        1 << self.id
    }

    /// Returns `true` if channel's bit is set in specified value.
    /// The value should usually be a register's content.
    #[inline(always)]
    fn is_channels_bit_set(&self, value: u32) -> bool {
        value & self.channel_bitmask() != 0
    }

    /// Pointer to XDMAC's registers.
    const XDMAC_REGISTERS: *const RegisterBlock = XDMAC::PTR;
}
