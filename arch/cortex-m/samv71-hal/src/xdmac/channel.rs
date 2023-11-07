//! Implementation of XDMAC's channel.

use core::marker::PhantomData;

use samv71q21_pac::{
    xdmac::{xdmac_chid::XDMAC_CHID as ChannelRegisters, RegisterBlock},
    XDMAC,
};

pub use super::channel_status::ChannelStatusReader;
pub use super::events::ChannelEvents;
use super::transfer::{ErrataTransferBlockConfig, TransferBlock};

/// Typestate trait representing generic XDMAC channel's state.
pub trait State {}

/// Typestate struct representing XDMAC channel in default, not configured state.
pub struct NotConfigured;

/// Typestate struct representing XDMAC channel in configured state.
pub struct Configured;

impl State for NotConfigured {}
impl State for Configured {}

/// XDMAC channel.
///
/// Channels can be created only via [`Xdmac`](super::Xdmac). After acquiring a channel, it can be
/// used to configure an XDMAC transfer and manage it (start, stop, suspend, flush).
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
/// Channel. Then, you can start the transfer by enabling the channel.
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
pub struct Channel<CurrentState: State> {
    /// Pointer to channel's registers.
    channel_registers: *const ChannelRegisters,
    /// Channel's numeric identifier.
    id: usize,
    /// Channel's status reader.
    status_reader: Option<ChannelStatusReader>,
    /// Channel's state metadata
    _state: PhantomData<CurrentState>,
}

/// Implementation of Channel functions available in every state.
impl<AnyState: State> Channel<AnyState> {
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

    /// Transforms Channel into a type with different state.
    ///
    /// This is a helper function that reduces state transition boilerplate.
    ///
    /// # Parameters
    /// * `channel` - Channel instance to be consumed and transformed.
    ///
    /// # Returns
    /// Transformed Channel instance.
    const fn transform<NewState: State>(channel: Channel<NewState>) -> Self {
        Self {
            channel_registers: channel.channel_registers,
            id: channel.id,
            status_reader: channel.status_reader,
            _state: PhantomData,
        }
    }

    /// Pointer to XDMAC's registers.
    const XDMAC_REGISTERS: *const RegisterBlock = XDMAC::PTR;
}

impl Channel<NotConfigured> {
    /// Configures an XDMAC transfer on this channel.
    /// Consumes channel's instance, and returns one with new state.
    pub fn configure_transfer(self, block: TransferBlock) -> Channel<Configured> {
        // Per the procedure described in SAMV71 datasheet:
        // 1. Select a free channel - already done.
        // 2. Clear pending event status bits by reading Channel Interrupt Status Register
        self.channel_registers_ref().cis.read();

        // 3. Write source address
        self.channel_registers_ref()
            .csa
            .write(|w| w.sa().variant(block.source().address as u32));
        // 4. Write destination address
        self.channel_registers_ref()
            .cda
            .write(|w| w.da().variant(block.destination().address as u32));

        // 5. Program the amount of data in a microblock
        self.channel_registers_ref()
            .cubc
            .write(|w| w.ublen().variant(block.microblock_length().get()));

        // 6. Program channel configuration register
        // In order to prevent some issues described in errata, some settings must be transformed
        let errata_config = ErrataTransferBlockConfig::from_transfer_block(&block);

        self.channel_registers_ref().cc.write(|w| {
            // Safety: some errata-specific things need unsafe handling, specifically - peripheral
            // ID must be set to an "invalid" unused value to prevent a mem2mem transfer issue.
            unsafe {
                w.type_()
                    .variant(block.transfer_type().into())
                    .mbsize()
                    .variant(block.memory_burst_size().into())
                    .dsync()
                    .variant(block.transfer_type().into())
                    .swreq()
                    .variant(block.transfer_type().into())
                    // Memset is not supported.
                    .memset()
                    .normal_mode()
                    .csize()
                    .variant(block.chunk_size().into())
                    .dwidth()
                    .variant(block.data_width().into())
                    .sif()
                    .variant(block.source().interface.into())
                    .dif()
                    .variant(block.destination().interface.into())
                    .sam()
                    .variant(errata_config.source_addressing_mode)
                    .dam()
                    .variant(errata_config.destination_addressing_mode)
                    .perid()
                    .bits(errata_config.peripheral_id)
            }
        });

        // 7. Program the number of microblocks in data
        self.channel_registers_ref()
            .cbc
            // -1 here is required, as the value in register is offset by 1 (i.e. 0 in the register
            // means that block has 1 microblock). Block length type is bound to valid range of
            // values, so it cannot be 0.
            .write(|w| w.blen().variant(block.block_length().get() - 1));

        // 7.5. Program errata-specific data striding settings
        self.channel_registers_ref().cds_msp.write(|w| {
            w.sds_msp()
                .variant(errata_config.data_striding)
                .dds_msp()
                .variant(errata_config.data_striding)
        });

        // 8. Clear unused registers.
        self.channel_registers_ref().cnda.reset();
        self.channel_registers_ref().cndc.reset();
        self.channel_registers_ref().csus.reset();
        self.channel_registers_ref().cdus.reset();

        // Now the user can configure interrupts/events and start the channel's operation.
        Channel::transform(self)
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

    /// Creates a new channel.
    pub(super) fn new(id: usize, registers: *const ChannelRegisters) -> Self {
        // Safety: This is safe, as channel registers pointer must be provided by Xdmac driver (and
        // therefore be valid).
        let status_register = &unsafe { &*registers }.cis;
        Self {
            id,
            channel_registers: registers,
            status_reader: Some(ChannelStatusReader::new(id, status_register)),
            _state: PhantomData,
        }
    }
}

impl Channel<Configured> {
    /// Restores it's default configuration.
    /// Consumes channel's instance, and returns one with new state.
    /// Channel must be disabled before calling this function.
    ///
    /// # Details
    /// This function will reset only the transfer-related configuration registers.
    /// Event/interrupt settings and global XDMAC configuration will not be modified.
    ///
    /// # Returns
    /// Channel with default configuration, or `None` if channel is currently enabled.
    pub fn reset_state(self) -> Option<Channel<NotConfigured>> {
        if self.is_busy() {
            return None;
        }

        self.channel_registers_ref().csa.reset();
        self.channel_registers_ref().cda.reset();
        self.channel_registers_ref().cnda.reset();
        self.channel_registers_ref().cndc.reset();
        self.channel_registers_ref().cubc.reset();
        self.channel_registers_ref().cbc.reset();
        self.channel_registers_ref().cc.reset();
        self.channel_registers_ref().cds_msp.reset();
        self.channel_registers_ref().csus.reset();
        self.channel_registers_ref().cdus.reset();

        let mut channel: Channel<NotConfigured> = Channel::transform(self);
        channel.disable_interrupt();
        channel.set_events_state(ChannelEvents {
            end_of_block: false,
            end_of_list: false,
            end_of_disable: false,
            end_of_flush: false,
            read_bus_error: false,
            write_bus_error: false,
            request_overflow_error: false,
        });

        Some(channel)
    }

    /// Returns `true` if Channel is currently enabled and XDMAC transfer is in progress.
    pub fn is_busy(&self) -> bool {
        self.is_channels_bit_set(self.xdmac_registers_ref().gs.read().bits())
    }

    /// Enables the channel and starts the transfer, if the channel is not busy.
    ///
    /// # Returns
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

    /// Disables the channel, stopping ongoing transfer (if any is in progress).
    ///
    /// If the channel is performing peripheral-to-memory transfer, the pending bytes from XDMAC
    /// FIFO are written to destination memory. Otherwise, the transfer is terminated immediately.
    ///
    /// Hardware-synchronized transfers automatically disable the channel when they're completed.
    pub fn disable(&mut self) {
        self.xdmac_registers_ref()
            .gd
            .write(|w| unsafe { w.bits(self.channel_bitmask()) });
    }

    /// Requests a DMA transfer for this channel, if the request is not pending already.
    /// This function must be used to trigger a software-synchronized DMA transfer.
    ///
    /// # Returns
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

    /// Returns `true` if the channel is currently configured as peripheral-synchronized.
    fn is_peripheral_synchronized(&self) -> bool {
        self.channel_registers_ref().cc.read().type_().is_per_tran()
    }
}
