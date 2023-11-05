use aerugo::hal::drivers::xdmac::transfer::{
    AddressingMode, BlockLength, ChunkSize, DataWidth, MemoryBurstSize, MicroblockLength,
    Peripheral, SystemBus, TransferBlock, TransferLocation, TransferType, TriggerSource,
};
use calldwell::write_str;

pub fn perform_test() {
    write_str("Performing transfer config tests...");

    check_block_length_creation();
    check_microblock_length_creation();
    check_transfer_block_creation();

    write_str("Transfer config tests finished successfully!");
}

fn check_block_length_creation() {
    assert!(
        BlockLength::new(BlockLength::LOW).is_some(),
        "Block length with minimum value cannot be created!"
    );

    assert!(
        BlockLength::new(BlockLength::LOW + 100).is_some(),
        "Block length with arbitrary valid value cannot be created!"
    );

    assert!(
        BlockLength::new(BlockLength::HIGH).is_some(),
        "Block length with maximum value cannot be created!"
    );

    assert!(
        BlockLength::new(BlockLength::HIGH + 1).is_none(),
        "Block length with value greater than maximum can be created!"
    );

    assert!(
        BlockLength::new(BlockLength::LOW - 1).is_none(),
        "Block length with value lesser than minimum can be created!"
    );

    let block_length = BlockLength::new(BlockLength::LOW + 100).unwrap();
    assert_eq!(*block_length, BlockLength::LOW + 100);
    assert_eq!(block_length.get(), BlockLength::LOW + 100);

    write_str("Block length creation test finished successfully.");
}

fn check_microblock_length_creation() {
    assert!(
        MicroblockLength::new(MicroblockLength::LOW).is_some(),
        "Microblock length with minimum value cannot be created!"
    );

    assert!(
        MicroblockLength::new(MicroblockLength::LOW + 100).is_some(),
        "Microblock length with arbitrary valid value cannot be created!"
    );

    assert!(
        MicroblockLength::new(MicroblockLength::HIGH).is_some(),
        "Microblock length with maximum value cannot be created!"
    );

    assert!(
        MicroblockLength::new(MicroblockLength::HIGH + 1).is_none(),
        "Microblock length with value greater than maximum can be created!"
    );

    // Microblock length less than LOW cannot be created, as it would result in overflow.
    // This is guarded by the compiler, so it cannot be tested at runtime as trying to do it results
    // in compilation error.

    let block_length = MicroblockLength::new(MicroblockLength::LOW + 100).unwrap();
    assert_eq!(*block_length, MicroblockLength::LOW + 100);
    assert_eq!(block_length.get(), MicroblockLength::LOW + 100);

    write_str("Microblock length creation test finished successfully.");
}

fn check_transfer_block_creation() {
    let u8_aligned_location = TransferLocation {
        address: 0xDEADC0D1 as *const (),
        interface: SystemBus::Interface0,
        addressing_mode: AddressingMode::Fixed,
    };

    let u16_aligned_location = TransferLocation {
        address: 0xDEADC0D2 as *const (),
        interface: SystemBus::Interface0,
        addressing_mode: AddressingMode::Fixed,
    };

    let u32_aligned_location = TransferLocation {
        address: 0xDEADC0D4 as *const (),
        interface: SystemBus::Interface0,
        addressing_mode: AddressingMode::Fixed,
    };

    assert!(
        TransferBlock::new(
            u8_aligned_location,
            u16_aligned_location,
            TransferType::MemoryToMemory,
            DataWidth::TwoBytes,
        )
        .is_none(),
        "TransferBlock with unaligned address (8bit source with 16bit data width) can be created!"
    );

    assert!(
        TransferBlock::new(
            u16_aligned_location,
            u8_aligned_location,
            TransferType::MemoryToMemory,
            DataWidth::TwoBytes,
        )
        .is_none(),
        "TransferBlock with unaligned address (8bit destination with 16bit data width) can be created!"
    );

    assert!(
        TransferBlock::new(
            u8_aligned_location,
            u16_aligned_location,
            TransferType::MemoryToMemory,
            DataWidth::FourBytes,
        )
        .is_none(),
        "TransferBlock with unaligned address (8bit source and 16bit destination with 32bit data width) can be created!"
    );

    let block = TransferBlock::new(
        u32_aligned_location,
        u8_aligned_location,
        TransferType::MemoryToPeripheral(Peripheral::UART0_RX, TriggerSource::Hardware),
        DataWidth::Byte,
    )
    .unwrap();

    assert_eq!(block.source(), u32_aligned_location);
    assert_eq!(block.destination(), u8_aligned_location);
    assert_eq!(*block.microblock_length(), 1);
    assert_eq!(*block.block_length(), 1);
    assert_eq!(
        block.transfer_type(),
        TransferType::MemoryToPeripheral(Peripheral::UART0_RX, TriggerSource::Hardware)
    );
    assert_eq!(block.memory_burst_size(), MemoryBurstSize::Single);
    assert_eq!(block.chunk_size(), ChunkSize::OneData);
    assert_eq!(block.data_width(), DataWidth::Byte);

    let block = block.with_microblock_length(MicroblockLength::new(10).unwrap());
    assert_eq!(*block.microblock_length(), 10);

    let block = block.with_block_length(BlockLength::new(123).unwrap());
    assert_eq!(*block.block_length(), 123);

    let block = block.with_chunk_size(ChunkSize::EightData);
    assert_eq!(block.chunk_size(), ChunkSize::EightData);

    let block = block.with_memory_burst_size(MemoryBurstSize::Sixteen);
    assert_eq!(block.memory_burst_size(), MemoryBurstSize::Sixteen);

    write_str("Transfer block creation test finished successfully.");
}
