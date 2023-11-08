//! Module with transfer-related items.

use crate::utils::{BoundedU16, BoundedU32};
use samv71q21_pac::xdmac::{
    self,
    xdmac_chid::cc::{
        CSIZESELECT_A, DAMSELECT_A, DIFSELECT_A, DSYNCSELECT_A, DWIDTHSELECT_A, MBSIZESELECT_A,
        SAMSELECT_A, SIFSELECT_A, SWREQSELECT_A, TYPESELECT_A,
    },
};

/// A single block of XDMAC transfer.
///
/// # Safety
///
/// **XDMAC transfers are inherently unsafe, as they operate directly on raw memory.**
///
/// In order to make sure that an XDMAC transfer will be performed safely, **you** must guarantee
/// that the transfer configuration is valid, and will be valid during XDMAC transfer.
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

/// This structure contains XDMAC channel settings that are specific to errata issues.
pub(super) struct ErrataTransferBlockConfig {
    /// Source and destination data striding. According to errata, section 2.5.2, this must be
    /// set to -1 (in 24-bit two's complement) when transferring 8-bit or 16-bit data when
    /// either destination or source is in fixed addressing mode. Otherwise **both** destination
    /// and source addresses will increment by 8/16-bits.
    /// Note: This is an u16, as PAC requires an u16 value. However, it's set manually to a valid
    /// constant.
    pub data_striding: u16,
    /// Source addressing mode, when errata section 2.5.2 does not apply it's the value configured
    /// by the user. Otherwise, it's set to microblock and data striding.
    pub source_addressing_mode: SAMSELECT_A,
    /// Destination addressing mode, when errata section 2.5.2 does not apply it's the value
    /// configured by the user. Otherwise, it's set to microblock and data striding.
    pub destination_addressing_mode: DAMSELECT_A,
    /// XDMAC Peripheral ID. According to errata, section 2.5.3, this must be set to an unused
    /// peripheral ID when mem2mem transfer is performed.
    pub peripheral_id: u8,
}

impl ErrataTransferBlockConfig {
    /// Creates an errata-specific config from standard transfer block configuration.
    pub(super) fn from_transfer_block(block: &TransferBlock) -> Self {
        let peripheral_id = match block.transfer_type() {
            // Per errata section 2.5.3, it must be set to an unused peripheral's ID when
            // mem2mem transfer is performed. Therefore, it's set to maximum supported value (this
            // field is 7-bit long).
            TransferType::MemoryToMemory => 0b1111111u8,
            TransferType::PeripheralToMemory(id, _) => id.into(),
            TransferType::MemoryToPeripheral(id, _) => id.into(),
        };

        // Per errata section 2.5.2, if transfer is 8 or 16-bit, and either source or destination
        // is in fixed addressing mode, to prevent either address from being incremented,
        // addressing mode must be set to microblock and data stride with microblock stride = 0
        // and data stride = -1 (in 24-bit two's complement format)
        let data_width_less_than_word = block.data_width() != DataWidth::FourBytes;
        let source_addressing_fixed = block.source().addressing_mode == AddressingMode::Fixed;
        let destination_addressing_fixed =
            block.destination().addressing_mode == AddressingMode::Fixed;

        if data_width_less_than_word && (source_addressing_fixed || destination_addressing_fixed) {
            Self {
                data_striding: 0xFFFFu16, // This is -1 in two's complement format.
                source_addressing_mode: if source_addressing_fixed {
                    SAMSELECT_A::UBS_DS_AM
                } else {
                    block.source().addressing_mode.into()
                },
                destination_addressing_mode: if destination_addressing_fixed {
                    DAMSELECT_A::UBS_DS_AM
                } else {
                    block.destination().addressing_mode.into()
                },
                peripheral_id,
            }
        } else {
            Self {
                data_striding: 0,
                source_addressing_mode: block.source().addressing_mode.into(),
                destination_addressing_mode: block.destination().addressing_mode.into(),
                peripheral_id,
            }
        }
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

impl From<TransferType> for TYPESELECT_A {
    fn from(value: TransferType) -> Self {
        match value {
            TransferType::MemoryToMemory => TYPESELECT_A::MEM_TRAN,
            TransferType::PeripheralToMemory(_, _) => TYPESELECT_A::PER_TRAN,
            TransferType::MemoryToPeripheral(_, _) => TYPESELECT_A::PER_TRAN,
        }
    }
}

impl From<TransferType> for DSYNCSELECT_A {
    fn from(value: TransferType) -> Self {
        match value {
            // Documentation does not specify what value this field should have
            // for mem2mem transfers. Therefore, we choose the "default" 0 value.
            TransferType::MemoryToMemory => DSYNCSELECT_A::PER2MEM,
            TransferType::PeripheralToMemory(_, _) => DSYNCSELECT_A::PER2MEM,
            TransferType::MemoryToPeripheral(_, _) => DSYNCSELECT_A::MEM2PER,
        }
    }
}

impl From<TransferType> for SWREQSELECT_A {
    fn from(value: TransferType) -> Self {
        match value {
            // mem2mem is always software-triggered
            TransferType::MemoryToMemory => SWREQSELECT_A::SWR_CONNECTED,
            TransferType::PeripheralToMemory(_, trigger_source) => trigger_source.into(),
            TransferType::MemoryToPeripheral(_, trigger_source) => trigger_source.into(),
        }
    }
}

impl From<TriggerSource> for SWREQSELECT_A {
    fn from(value: TriggerSource) -> Self {
        match value {
            TriggerSource::Software => SWREQSELECT_A::SWR_CONNECTED,
            TriggerSource::Hardware => SWREQSELECT_A::HWR_CONNECTED,
        }
    }
}

impl From<MemoryBurstSize> for MBSIZESELECT_A {
    fn from(value: MemoryBurstSize) -> Self {
        match value {
            MemoryBurstSize::Single => MBSIZESELECT_A::SINGLE,
            MemoryBurstSize::Four => MBSIZESELECT_A::FOUR,
            MemoryBurstSize::Eight => MBSIZESELECT_A::EIGHT,
            MemoryBurstSize::Sixteen => MBSIZESELECT_A::SIXTEEN,
        }
    }
}

impl From<ChunkSize> for CSIZESELECT_A {
    fn from(value: ChunkSize) -> Self {
        match value {
            ChunkSize::OneData => CSIZESELECT_A::CHK_1,
            ChunkSize::TwoData => CSIZESELECT_A::CHK_2,
            ChunkSize::FourData => CSIZESELECT_A::CHK_4,
            ChunkSize::EightData => CSIZESELECT_A::CHK_8,
            ChunkSize::SixteenData => CSIZESELECT_A::CHK_16,
        }
    }
}

impl From<DataWidth> for DWIDTHSELECT_A {
    fn from(value: DataWidth) -> Self {
        match value {
            DataWidth::Byte => DWIDTHSELECT_A::BYTE,
            DataWidth::TwoBytes => DWIDTHSELECT_A::HALFWORD,
            DataWidth::FourBytes => DWIDTHSELECT_A::WORD,
        }
    }
}

impl From<SystemBus> for SIFSELECT_A {
    fn from(value: SystemBus) -> Self {
        match value {
            SystemBus::Interface0 => SIFSELECT_A::AHB_IF0,
            SystemBus::Interface1 => SIFSELECT_A::AHB_IF1,
        }
    }
}

impl From<SystemBus> for DIFSELECT_A {
    fn from(value: SystemBus) -> Self {
        match value {
            SystemBus::Interface0 => DIFSELECT_A::AHB_IF0,
            SystemBus::Interface1 => DIFSELECT_A::AHB_IF1,
        }
    }
}

impl From<AddressingMode> for SAMSELECT_A {
    fn from(value: AddressingMode) -> Self {
        match value {
            AddressingMode::Fixed => SAMSELECT_A::FIXED_AM,
            AddressingMode::Incremented => SAMSELECT_A::INCREMENTED_AM,
        }
    }
}

impl From<AddressingMode> for DAMSELECT_A {
    fn from(value: AddressingMode) -> Self {
        match value {
            AddressingMode::Fixed => DAMSELECT_A::FIXED_AM,
            AddressingMode::Incremented => DAMSELECT_A::INCREMENTED_AM,
        }
    }
}
