//! Module with embedded-hal tests.

use aerugo::hal::{
    drivers::spi::{
        chip_config::{
            BitsPerTransfer, ChipConfig, ChipSelectBehavior, ClockPhase, ClockPolarity,
            SerialClockDivider,
        },
        config::{MasterConfig, SelectedChip},
        embedded_hal::SpiBus,
        interrupts::Interrupts,
        NotConfigured, Spi,
    },
    user_peripherals::SPI0,
};
use calldwell::write_str;

pub fn perform_test(spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, NotConfigured> {
    write_str("Beginning SPI communication tests via embedded-hal traits...");

    let mut spi = spi.into_master(MasterConfig::new(SelectedChip::Chip0));
    spi.configure_chip(
        SelectedChip::Chip0,
        ChipConfig {
            clock_polarity: ClockPolarity::LowWhenInactive,
            clock_phase: ClockPhase::DataCapturedOnLeadingEdge,
            chip_select_behavior: ChipSelectBehavior::DeactivateAfterLastTransfer,
            bits_per_transfer: BitsPerTransfer::Bits8,
            clock_divider: SerialClockDivider::new_saturated(0),
            delay_before_first_clock: 0,
            delay_between_consecutive_transfers: 0,
        },
    );
    spi.set_interrupts_state(Interrupts {
        rx_data_register_full: true,
        tx_data_register_empty: true,
        mode_fault_error: true,
        overrun_error: true,
        nss_rising: false,
        tx_registers_empty: true,
        underrun_error: false,
    });
    spi.enable_loopback();

    // Only `transfer` can be tested via loopback.
    const TEST_DATA_LENGTH: usize = 1024;
    let mut write_buffer: [u8; TEST_DATA_LENGTH] = [0; TEST_DATA_LENGTH];
    let mut read_buffer: [u8; TEST_DATA_LENGTH] = [0; TEST_DATA_LENGTH];

    const TEST_OFFSET: usize = 0x10;
    for (i, word) in write_buffer.iter_mut().enumerate() {
        *word = (TEST_OFFSET + i) as u8;
    }

    assert_ne!(read_buffer, write_buffer);
    assert_eq!(spi.transfer(&mut read_buffer, &write_buffer), Ok(()));
    assert_eq!(read_buffer, write_buffer);

    write_str("All SPI communication tests via embedded-hal traits done!");

    spi.reset()
}
