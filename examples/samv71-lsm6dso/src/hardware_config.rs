use aerugo::hal::{
    drivers::{
        pio::{
            pin::{InputMode, Peripheral, PeripheralMode},
            Pin, Port,
        },
        pmc::{config::PeripheralId, PMC},
        spi::{
            chip_config::{
                BitsPerTransfer, ChipConfig, ChipSelectBehavior, ClockPhase, ClockPolarity,
                SerialClockDivider,
            },
            config::{MasterConfig, SelectedChip},
            interrupts::Interrupts,
            Master, NotConfigured, Spi,
        },
    },
    user_peripherals::{PIOD, SPI0},
};

/// LSM6DSO chip select signal.
const LSM6DSO_CHIP: SelectedChip = SelectedChip::Chip1;
/// LSM6DSO SPI clock divider. Default peripheral clock is 12MHz, LSM6DSO can work with up to 10MHz,
/// but for safety a 480kHz clock will be used instead (12MHz/25).
const LSM6DSO_SPI_CLOCK_DIVIDER: u8 = 25;

pub struct LSM6DSOPins {
    pub miso: Pin<PeripheralMode>,
    pub mosi: Pin<PeripheralMode>,
    pub sck: Pin<PeripheralMode>,
    pub cs: Pin<PeripheralMode>,
    pub int1: Pin<InputMode>,
}

pub fn configure_pio(port: Port<PIOD>) -> LSM6DSOPins {
    let mut pins = port.into_pins();

    let miso = pins[20].take().unwrap().into_peripheral_pin(Peripheral::B);
    let mosi = pins[21].take().unwrap().into_peripheral_pin(Peripheral::B);
    let sck = pins[22].take().unwrap().into_peripheral_pin(Peripheral::B);
    let cs = pins[25].take().unwrap().into_peripheral_pin(Peripheral::B);
    let int1 = pins[28].take().unwrap().into_input_pin();

    LSM6DSOPins {
        miso,
        mosi,
        sck,
        cs,
        int1,
    }
}

pub fn configure_spi(spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, Master> {
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
    spi.set_interrupts_state(Interrupts {
        rx_data_register_full: true,
        tx_data_register_empty: true,
        mode_fault_error: false,
        overrun_error: false,
        nss_rising: false,
        tx_registers_empty: true,
        underrun_error: false,
    });

    spi
}

pub fn configure_pmc(pmc: &mut PMC) {
    pmc.enable_peripheral_clock(PeripheralId::SPI0);
    pmc.enable_peripheral_clock(PeripheralId::PIOD);
}
