use aerugo::hal::{
    drivers::{
        nvic::{Interrupt, NVIC},
        spi::{
            chip_config::{
                BitsPerTransfer, ChipConfig, ChipSelectBehavior, ClockPhase, ClockPolarity,
                SerialClockDivider,
            },
            config::{MasterConfig, SelectedChip},
            metadata::SPIMetadata,
            Master, NotConfigured, Spi,
        },
        xdmac::{
            channel::ChannelStatusReader,
            status::StatusReader as XdmacStatusReader,
            transfer::{
                AddressingMode, DataWidth, MicroblockLength, SystemBus, TransferBlock,
                TransferLocation, TransferType, TriggerSource,
            },
            Xdmac,
        },
    },
    interrupt,
    user_peripherals::SPI0,
};
use calldwell::write_str;

/// LSM6DSO chip select signal.
const LSM6DSO_CHIP: SelectedChip = SelectedChip::Chip1;
/// LSM6DSO SPI clock divider. Default peripheral clock is 12MHz, LSM6DSO can work with up to 10MHz,
/// but for safety a 480kHz clock will be used instead (12MHz/25).
const LSM6DSO_SPI_CLOCK_DIVIDER: u8 = 25;
/// LSM6DSO SPI interface.
#[allow(non_camel_case_types)]
type LSM6DSO_SPI = SPI0;

static mut XDMAC_STATUS_READER: Option<XdmacStatusReader> = None;
static mut XDMAC_CHANNEL_STATUS_READER: Option<ChannelStatusReader> = None;

pub fn perform_test(spi: Spi<LSM6DSO_SPI, NotConfigured>, xdmac: &mut Xdmac, nvic: &mut NVIC) {
    write_str("Beginning LSM6DSO communication tests w/ XDMAC...");

    let mut spi = spi.into_master(MasterConfig::new(LSM6DSO_CHIP));
    spi.configure_chip(
        LSM6DSO_CHIP,
        ChipConfig {
            clock_polarity: ClockPolarity::HighWhenInactive,
            clock_phase: ClockPhase::DataChangedOnLeadingEdge,
            chip_select_behavior: ChipSelectBehavior::DeactivateAfterLastTransfer,
            bits_per_transfer: BitsPerTransfer::Bits8,
            clock_divider: SerialClockDivider::new(LSM6DSO_SPI_CLOCK_DIVIDER).unwrap(),
            delay_before_first_clock: 0,
            delay_between_consecutive_transfers: 0,
        },
    );

    unsafe { XDMAC_STATUS_READER = xdmac.take_status_reader() };
    let _rx_channel = xdmac.take_next_free_channel().unwrap();
    let _tx_channel = xdmac.take_next_free_channel().unwrap();

    nvic.enable(Interrupt::XDMAC);

    // TODO: Test code here

    nvic.disable(Interrupt::XDMAC);

    write_str("All LSM6DSO communication tests w/ XDMAC done!");
}

#[interrupt]
fn XDMAC() {
    let status_reader = unsafe { XDMAC_STATUS_READER.as_mut().unwrap() };
    let channel_status_reader = unsafe { XDMAC_CHANNEL_STATUS_READER.as_mut().unwrap() };

    if status_reader.get_pending_channels()[channel_status_reader.id()] {
        let status = channel_status_reader.get_pending_events();

        if status.read_bus_error {
            panic!("XDMAC read bus error");
        }

        if status.write_bus_error {
            panic!("XDMAC write bus error");
        }

        if status.request_overflow_error {
            panic!("XDMAC request overflow error");
        }

        if status.end_of_block {}
    }
}

fn _get_rx_transfer_block(
    spi: &Spi<LSM6DSO_SPI, Master>,
    destination: *mut (),
    data_length: u32,
) -> TransferBlock {
    let source = TransferLocation {
        address: spi.xdmac_rx_address(),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Fixed,
    };

    let destination = TransferLocation {
        address: destination as *const (),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    TransferBlock::new(
        source,
        destination,
        TransferType::PeripheralToMemory(LSM6DSO_SPI::DMA_RX_PERIPHERAL, TriggerSource::Hardware),
        DataWidth::Byte,
    )
    .unwrap()
    .with_microblock_length(MicroblockLength::new(data_length).unwrap())
}

fn _get_tx_transfer_block(
    spi: &Spi<LSM6DSO_SPI, Master>,
    source: *const (),
    data_length: u32,
) -> TransferBlock {
    let source = TransferLocation {
        address: source,
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Incremented,
    };

    let destination = TransferLocation {
        address: spi.xdmac_tx_address(),
        interface: SystemBus::Interface1,
        addressing_mode: AddressingMode::Fixed,
    };

    TransferBlock::new(
        source,
        destination,
        TransferType::MemoryToPeripheral(LSM6DSO_SPI::DMA_TX_PERIPHERAL, TriggerSource::Hardware),
        DataWidth::Byte,
    )
    .unwrap()
    .with_microblock_length(MicroblockLength::new(data_length).unwrap())
}
