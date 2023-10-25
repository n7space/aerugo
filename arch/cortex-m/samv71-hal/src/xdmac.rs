//! Implementation of HAL XDMAC driver.
//!
//! # Introduction
//!
//! XDMAC (DMA Controller) performs peripheral data transfer and memory copy/set operations.
//! It's connected to the MCU's bus matrix using two master bus interfaces, interconnected with
//! memories and peripherals.
//!
//! XDMAC transfers are performed using channels. On SAME70/S70/V70/V71 XDMAC provides 24 channels
//! for 24 simultaneous transfers.
//!
//! Transfers can be triggered manually by software, or automatically by hardware (in case of
//! peripheral-synchronized transfers). Memory-to-memory transfers can be triggered only by software.
//!
//! Transfers are made of linked blocks. Most transfer settings can be changed per block.
//! Blocks are made of a programmable number of microblocks, which are made of programmable
//! number of data units (chunks or bursts).
//!
//! Bursts are used for accessing external dynamic memory, while chunks are used for peripheral data
//! transfers. Microblock's size doesn't have to be an integral multiple of the chunk/burst size, an
//! incomplete chunk/burst transfer may be performed if necessary.
//!
//! # Using the driver
//!
//! Main [`Xdmac`] structure allows you to fetch XDMAC info (number of available peripheral requests,
//! FIFO size, number of channels), get an instance of XDMAC [`Channel`].
//!
//! Channels can be selected automatically with [`Xdmac::take_next_free_channel`], or manually, using
//! [`Xdmac::take_channel`]. You can check channel's availability with [`Xdmac::is_channel_available`].
//!
//! After receiving instance of [`Channel`] from [`Xdmac`], you can use it to configure the transfer
//! list, and manage channel's state.
//!
//! Both [`Xdmac`] and [`Channel`] provide status reader objects -
//! [`StatusReader`] and [`ChannelStatusReader`](channel::ChannelStatusReader), that should be given
//! to interrupt handlers to check IRQ-related flags, or can be used to check the status manually in
//! interrupt-less systems.
//!
//! # Thread/interrupt safety
//!
//! Due to the fact that some channel-related XDMAC functionality is stored in main XDMAC registers,
//! which means that they must be shared between [`Xdmac`] and all [`Channel`] instances,
//! [`Channel`]s are not thread-safe, as they share the state - even if we assume that the register
//! read/write operation are atomic, some registers might require read-modify-write operation to apply
//! certain settings, which usually isn't atomic. Therefore, if you intend to share the [`Channel`]
//! instances, you must provide appropriate synchronization mechanisms (for example, Mutex) to
//! prevent simultaneous access to XDMAC registers from happening.
//!
//! With that in mind, both [`Xdmac`] and [`Channel`] provide status reader objects that can be
//! passed safely to IRQ handlers, as mentioned above, instead of sharing the [`Channel`] and
//! [`Xdmac`] instances. Status readers have exclusive access to status registers, and assuming
//! that the [`Xdmac`] object is not misused, there should always be at most 1 [`Channel`] instance
//! per XDMAC channel, and therefore there should be at most 1 status reader per XDMAC channel.
//!
//! # MATRIX connections
//!
//! For SAME70/S70/V70/V71 MCUs, XDMAC can access following clients via specified bus interfaces:
//!
//! * Bus interface 0 (MATRIX host 4)
//!     - Internal SRAM (MATRIX client 0)
//!     - External Bus Interface (MATRIX client 5)
//!     - Cortex-M7 AHB Client[¹](#cortex-m7-interconnect-remark) (AHBS) (MATRIX client 8)
//! * Bus interface 1 (MATRIX host 5)
//!     - Internal SRAM (MATRIX client 1)
//!     - Internal Flash (MATRIX client 3)
//!     - External Bus Interface (MATRIX client 5)
//!     - QSPI (MATRIX client 6)
//!     - Peripheral Bridge (MATRIX client 7)
//!
//! Reverse mapping list is provided for convenience:
//!
//! * **Internal SRAM** *(MATRIX client 0 and 1)* can be accessed using **both interfaces**.
//! * **Internal FLASH** *(MATRIX client 3)* must be accessed using **interface 1**.
//! * **External Bus Interface** *(MATRIX client 5)* can be accessed using **both interfaces**,
//!   however, it has only one client connection (unlike SRAM).
//!   EBI provides access to external memories via Static Memory Controller (SMC).
//! * **QSPI** *(MATRIX client 6)* must be accessed using **interface 1**.
//! * **Peripheral Bridge** *(MATRIX client 7)* must be accessed using **interface 1**.
//!   That means most of the peripherals (except those having their own MATRIX client interfaces)
//!   must be accessed using that interface.
//! * **Cortex-M7 AHB Client[¹](#cortex-m7-interconnect-remark) (AHBS)** *(MATRIX client 8)* must be
//!   accessed using **interface 0**
//! * Internal ROM and USB HS Dual Port RAM cannot be accessed using XDMAC.
//!
//! # Cortex-M7 interconnect remark
//!
//! ¹For the connection of Cortex-M7 processor to the SRAM, refer to sections "Interconnect" and
//! "Memories", sub-section "Embedded Memories" in SAME70/S70/V70/V71 datasheet.

use samv71q21_pac::xdmac::xdmac_chid::XDMAC_CHID as ChannelRegisters;
use samv71q21_pac::XDMAC;

use self::channel::Channel;
use self::status::StatusReader;

pub mod channel;
pub mod interrupts;
pub mod status;

/// XDMAC driver.
pub struct Xdmac {
    /// PAC XDMAC instance.
    xdmac: XDMAC,
    /// Array with flags indicating if channel was taken.
    channel_taken: [bool; Self::SUPPORTED_CHANNELS],
    /// Xdmac's status reader.
    status_reader: Option<StatusReader>,
}

impl Xdmac {
    /// Amount of channels supported by this driver.
    /// This value should not be smaller than the value returned from [`Xdmac::available_channels`].
    /// SAMV71 documentation is broken and lies about it having only 7 channels.
    pub const SUPPORTED_CHANNELS: usize = 24;

    /// Creates new [`Xdmac`] instance using PAC XDMAC instance.
    /// PAC XDMAC instance is consumed to prevent creating multiple drivers.
    pub const fn new(xdmac: XDMAC) -> Xdmac {
        Xdmac {
            xdmac,
            channel_taken: [false; Self::SUPPORTED_CHANNELS],
            status_reader: Some(StatusReader {}),
        }
    }

    /// Returns `true` if specified channel is available and can be taken with [`Xdmac::take_channel`].
    /// Returns `false` if the channel is already taken.
    pub fn is_channel_available(&self, id: usize) -> bool {
        id < Self::SUPPORTED_CHANNELS && !self.channel_taken[id]
    }

    /// Tries to get a channel with specified ID.
    ///
    /// Returns it, if it's available. Returns `None` otherwise.
    pub fn take_channel(&mut self, id: usize) -> Option<Channel> {
        if self.is_channel_available(id) {
            self.channel_taken[id] = true;
            // Unwrap: If this fails, it's 100% HAL dev's fault for not having the
            // SUPPORTED_CHANNELS constant be equal to the amount of supported channels.
            Some(Channel::new(id, self.get_channel_registers(id).unwrap()))
        } else {
            None
        }
    }

    /// Looks for next available channel and returns it.
    /// Returns `None` if all channels are taken.
    pub fn take_next_free_channel(&mut self) -> Option<Channel> {
        for id in 0..Self::SUPPORTED_CHANNELS {
            if let Some(channel) = self.take_channel(id) {
                return Some(channel);
            }
        }

        None
    }

    /// Returns previously taken channel, making it possible to take it again.
    pub fn return_channel(&mut self, channel: Channel) {
        // Safety: This is safe, because channel's ownership is returned and it will be dropped at
        // the end of this function.
        unsafe { self.mark_channel_as_free(channel.id()) };
    }

    /// Marks the channel with specified ID as "free", which means that it's instance no longer
    /// exists, and a new instance of this channel can be safely created.
    ///
    /// # Safety
    ///
    /// You can call this function safely only if you can guarantee that the channel it marks as
    /// "free" no longer exists, or will be dropped shortly after marking it "free". Having multiple
    /// instances of a single Channel breaks the safety invariants of XDMAC driver, and may result
    /// in data races or undefined behaviors.
    pub unsafe fn mark_channel_as_free(&mut self, channel_id: usize) {
        self.channel_taken[channel_id] = false;
    }

    /// Returns status reader, if available. Returns `None` if it was already taken and not returned.
    pub fn take_status_reader(&mut self) -> Option<StatusReader> {
        self.status_reader.take()
    }

    /// Returns status reader's ownership to Xdmac.
    pub fn return_status_reader(&mut self, status_reader: StatusReader) {
        self.status_reader.replace(status_reader);
    }

    /// Returns `true` if status reader is currently owned by Xdmac driver.
    pub fn is_status_reader_available(&self) -> bool {
        self.status_reader.is_some()
    }

    /// Returns the number of available peripheral requests.
    pub fn number_of_peripheral_requests(&self) -> usize {
        (self.xdmac.gtype.read().nb_req().bits() as usize) + 1usize
    }

    /// Returns the size of internal FIFO, in bytes.
    pub fn fifo_size(&self) -> usize {
        self.xdmac.gtype.read().fifo_sz().bits() as usize
    }

    /// Returns number of available channels.
    pub fn available_channels(&self) -> usize {
        (self.xdmac.gtype.read().nb_ch().bits() as usize) + 1usize
    }

    /// Returns a reference to the registers of specified channel, or None if the channel
    /// doesn't exist.
    fn get_channel_registers(&self, channel: usize) -> Option<&ChannelRegisters> {
        // This is lots of boilerplate to avoid manual pointer operations.
        // On one hand, this is perfectly safe as it avoid making pointers manually.
        // On the other hand, make sure to port it appropriately if number of XDMAC channels
        // ever changes.
        // I blame `svd2rust` for not putting this in an indexed container, i can understand why
        // it cannot be an array (notice the padding between CHID instances in XDMAC's register
        // block), but c'mon.
        match channel {
            0 => Some(&self.xdmac.xdmac_chid0),
            1 => Some(&self.xdmac.xdmac_chid1),
            2 => Some(&self.xdmac.xdmac_chid2),
            3 => Some(&self.xdmac.xdmac_chid3),
            4 => Some(&self.xdmac.xdmac_chid4),
            5 => Some(&self.xdmac.xdmac_chid5),
            6 => Some(&self.xdmac.xdmac_chid6),
            7 => Some(&self.xdmac.xdmac_chid7),
            8 => Some(&self.xdmac.xdmac_chid8),
            9 => Some(&self.xdmac.xdmac_chid9),
            10 => Some(&self.xdmac.xdmac_chid10),
            11 => Some(&self.xdmac.xdmac_chid11),
            12 => Some(&self.xdmac.xdmac_chid12),
            13 => Some(&self.xdmac.xdmac_chid13),
            14 => Some(&self.xdmac.xdmac_chid14),
            15 => Some(&self.xdmac.xdmac_chid15),
            16 => Some(&self.xdmac.xdmac_chid16),
            17 => Some(&self.xdmac.xdmac_chid17),
            18 => Some(&self.xdmac.xdmac_chid18),
            19 => Some(&self.xdmac.xdmac_chid19),
            20 => Some(&self.xdmac.xdmac_chid20),
            21 => Some(&self.xdmac.xdmac_chid21),
            22 => Some(&self.xdmac.xdmac_chid22),
            23 => Some(&self.xdmac.xdmac_chid23),
            _ => None,
        }
    }
}
