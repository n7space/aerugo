//! Module with transfer-related items.

use crate::utils::{BoundedU16, BoundedU32};
use samv71q21_pac::xdmac;

/// A single block of XDMAC transfer.
///
/// # Safety
///
/// **XDMAC transfers are inherently unsafe, as they operate directly on raw memory.**
///
/// In order to make sure that an XDMAC transfer will be performed safely, **you** must guarantee
/// that the transfer configuration is valid, and will be valid during XDMAC transaction.
/// This mostly means that **you** must make sure the source and destination addresses point to
/// valid, aligned locations/buffers in memory, and the transfer is configured appropriately for the
/// buffer(s) size.
///
/// **XDMAC will not perform any safety checks. [`TransferBlock`]'s constructor and setters will
/// perform basic safety checks wherever possible, but it's not possible to fully validate the
/// transfer configuration that way. Breaking Rust/XDMAC safety invariants will lead to an
/// undefined behavior. Drive responsibly.**
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TransferBlock {
    /// Transfer's source
    source: TransferLocation,

    /// Transfer's destination
    destination: TransferLocation,

    /// Amount of data units in the microblock.
    ///
    /// Data unit size is configured in `data_width` field.
    microblock_length: MicroblockLength,

    /// Amount of microblocks in a transfer block.
    block_length: BlockLength,

    /// Transfer's type (memory-to-memory, peripheral-to-memory, memory-to-peripheral), and
    /// peripheral ID in case of peripheral transfer.
    transfer_type: TransferType,

    /// Memory burst size.
    memory_burst_size: MemoryBurstSize,

    /// Channel's chunk size, amount of data transferred in a single chunk.
    chunk_size: ChunkSize,

    /// Data width for a single transfer (smallest data unit).
    data_width: DataWidth,
}

/// Transfer location (destination or source) configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TransferLocation {
    /// Transfer's address.
    ///
    /// **This address must point to a valid memory location, aligned to the transfer's data width.**
    ///
    /// If addressing mode is set to "incremented", address must point to a sufficiently large,
    /// linear memory buffer.
    ///
    /// For peripheral transfers, this must point to the source/destination peripheral's data
    /// register.
    pub address: *const (),

    /// Bus interface that allows the access to the memory pointed by `address`.
    /// For details, check [`xdmac`](crate::xdmac#matrix-connections) module documentation.
    pub interface: SystemBus,

    /// Addressing mode - can either be fixed (address does not change during transfer), or
    /// incrementing (address is incremented after reading a data unit).
    pub addressing_mode: AddressingMode,
}

/// `TransferLocation` is only a storage for the pointer and it does not dereference it, so it's safe
/// to send it to other threads as it doesn't use the shared memory.
unsafe impl Send for TransferLocation {}
/// `TransferLocation` is only a storage for the pointer and it does not dereference it, so it's safe
/// to share as it doesn't use the shared memory.
unsafe impl Sync for TransferLocation {}

/// Transfer type.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TransferType {
    /// Memory-to-memory transfer.
    /// Both destination and source addresses must point to valid locations in memory.
    MemoryToMemory,
    /// Peripheral-to-memory transfer.
    /// Source address must point to provided peripheral's data register.
    /// Destination address must point to a valid location in memory.
    /// Trigger source indicates whether the transfer is automatically triggered by peripheral,
    /// or manually via software.
    PeripheralToMemory(Peripheral, TriggerSource),
    /// Memory-to-peripheral transfer.
    /// Source address must point to a valid location in memory.
    /// Destination address must point to provided peripheral's data register.
    /// Trigger source indicates whether the transfer is automatically triggered by peripheral,
    /// or manually via software.
    MemoryToPeripheral(Peripheral, TriggerSource),
}

/// System bus interfaces used by XDMAC.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SystemBus {
    /// System bus interface 0.
    Interface0,
    /// System bus interface 1.
    Interface1,
}

/// XDMAC addressing modes.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AddressingMode {
    /// Fixed addressing mode. Address will not change during block transfer.
    Fixed,
    /// Incremental addressing mode. XDMAC will read continuous segment of memory starting with
    /// specified address, which will be incremented every time data is read.
    Incremented,
}

/// Memory burst size.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MemoryBurstSize {
    /// Single data unit handled in single burst.
    Single,
    /// Up to four data units handled in single burst.
    Four,
    /// Up to eight data units handled in single burst.
    Eight,
    /// Up to sixteen data units handled in single burst.
    Sixteen,
}

/// Trigger source for a transfer.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TriggerSource {
    /// Transfer is software-triggered.
    Software,
    /// Transfer is hardware-triggered by a peripheral.
    Hardware,
}

/// Transfer's chunk size.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChunkSize {
    /// One data unit per chunk.
    OneData,
    /// Two data units per chunk.
    TwoData,
    /// Four data units per chunk.
    FourData,
    /// Eight data units per chunk.
    EightData,
    /// Sixteen data units per chunk.
    SixteenData,
}

/// Transfer's data width (size).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DataWidth {
    /// One data is a byte
    Byte,
    /// One data is a half-word.
    TwoBytes,
    /// One data is a word.
    FourBytes,
}

/// XDMAC peripheral connection.
pub type Peripheral = xdmac::xdmac_chid::cc::PERIDSELECT_A;

/// Microblock length type, limited to maximum amount of data units in a microblock.
pub type MicroblockLength = BoundedU32<0, 0xFFFFFF>;

/// Block length type, limited to maximum amount of microblocks in a block.
/// Must be at least 1.
pub type BlockLength = BoundedU16<1, 0x1000>;

impl TransferBlock {
    /// Validates provided configuration and creates a new `TransferBlock`, with following
    /// default settings:
    ///
    /// * Microblock length = 1 data unit
    /// * Block length = 1 microblock
    /// * Memory burst size = 1 data unit
    /// * Chunk size = 1 data unit
    ///
    /// These settings can be changed with chained methods (i.e. [`TransferBlock::with_chunk_size`]).
    ///
    /// If provided configuration is not valid (i.e. address is not aligned), `None` is returned
    /// instead.
    ///
    /// # Parameters
    ///
    /// * `source` - Transfer's source location configuration.
    /// * `destination` - Transfer's destination location configuration.
    /// * `transfer_type` - Transfer's type.
    /// * `data_width` - Size of one transferred data unit.
    pub fn new(
        source: TransferLocation,
        destination: TransferLocation,
        transfer_type: TransferType,
        data_width: DataWidth,
    ) -> Option<Self> {
        if source.is_aligned_to(data_width) && destination.is_aligned_to(data_width) {
            Some(Self {
                source,
                destination,
                microblock_length: MicroblockLength::new(1).unwrap(),
                block_length: BlockLength::new(1).unwrap(),
                transfer_type,
                memory_burst_size: MemoryBurstSize::Single,
                chunk_size: ChunkSize::OneData,
                data_width,
            })
        } else {
            None
        }
    }

    /// Returns a new instance of `TransferBlock` with provided microblock length.
    pub fn with_microblock_length(self, microblock_length: MicroblockLength) -> Self {
        Self {
            microblock_length,
            ..self
        }
    }

    /// Returns a new instance of `TransferBlock` with provided block length.
    pub fn with_block_length(self, block_length: BlockLength) -> Self {
        Self {
            block_length,
            ..self
        }
    }

    /// Returns a new instance of `TransferBlock` with provided chunk size.
    pub fn with_chunk_size(self, chunk_size: ChunkSize) -> Self {
        Self { chunk_size, ..self }
    }

    /// Returns a new instance of `TransferBlock` with provided memory burst size.
    pub fn with_memory_burst_size(self, memory_burst_size: MemoryBurstSize) -> Self {
        Self {
            memory_burst_size,
            ..self
        }
    }

    /// Returns transfer's source.
    pub fn source(&self) -> TransferLocation {
        self.source
    }

    /// Returns transfer's destination.
    pub fn destination(&self) -> TransferLocation {
        self.destination
    }

    /// Returns transfer's microblock length.
    pub fn microblock_length(&self) -> MicroblockLength {
        self.microblock_length
    }

    /// Returns transfer's block length.
    pub fn block_length(&self) -> BlockLength {
        self.block_length
    }

    /// Returns transfer's transfer type.
    pub fn transfer_type(&self) -> TransferType {
        self.transfer_type
    }

    /// Returns transfer's memory burst size.
    pub fn memory_burst_size(&self) -> MemoryBurstSize {
        self.memory_burst_size
    }

    /// Returns transfer's chunk size.
    pub fn chunk_size(&self) -> ChunkSize {
        self.chunk_size
    }

    /// Returns transfer's data width.
    pub fn data_width(&self) -> DataWidth {
        self.data_width
    }
}

impl TransferLocation {
    /// Returns `true` if location's address is aligned to specified data width.
    pub fn is_aligned_to(&self, data_width: DataWidth) -> bool {
        self.address.align_offset(data_width.into()) == 0
    }
}

impl From<DataWidth> for usize {
    fn from(value: DataWidth) -> Self {
        match value {
            DataWidth::Byte => 1,
            DataWidth::TwoBytes => 2,
            DataWidth::FourBytes => 4,
        }
    }
}

impl TryFrom<usize> for DataWidth {
    type Error = ();

    fn try_from(value: usize) -> Result<DataWidth, Self::Error> {
        match value {
            1 => Ok(DataWidth::Byte),
            2 => Ok(DataWidth::TwoBytes),
            4 => Ok(DataWidth::FourBytes),
            _ => Err(()),
        }
    }
}
