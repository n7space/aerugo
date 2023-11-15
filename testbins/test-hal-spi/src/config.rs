//! Module with SPI configuration tests
use aerugo::hal::{
    drivers::spi::{
        config::{ChipSelectionDelay, MasterConfig, SelectedChip},
        NotConfigured, Spi,
    },
    user_peripherals::SPI0,
};
use calldwell::write_str;

pub fn perform_test(spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, NotConfigured> {
    write_str("Beginning SPI configuration tests...");

    let spi = perform_master_mode_config_test(spi);
    let spi = perform_status_reader_test(spi);

    write_str("All SPI configuration tests done!");
    spi
}

fn perform_master_mode_config_test(spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, NotConfigured> {
    write_str("SPI master mode config test started");

    let config_a = MasterConfig {
        chip_selection_delay: ChipSelectionDelay::new(100).unwrap(),
        selected_chip: SelectedChip::Chip1,
        enable_overrun_detection: true,
    };

    let config_b = MasterConfig {
        chip_selection_delay: ChipSelectionDelay::new(42).unwrap(),
        selected_chip: SelectedChip::Chip3,
        enable_overrun_detection: false,
    };

    let spi = spi.into_master(config_a);
    assert_eq!(spi.master_config(), config_a);

    let spi = spi.reset().into_master(config_b);
    assert_eq!(spi.master_config(), config_b);

    write_str("SPI master mode config test finished successfully");
    spi.reset()
}

fn perform_status_reader_test(mut spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, NotConfigured> {
    write_str("SPI status reader test started");

    assert!(spi.is_status_reader_available());
    let mut status_reader = spi.take_status_reader().unwrap();
    assert!(!spi.is_status_reader_available());

    let status = status_reader.status();
    assert!(!status.is_enabled);

    let mut spi = spi.into_master(MasterConfig {
        chip_selection_delay: ChipSelectionDelay::new_saturated(0),
        selected_chip: SelectedChip::Chip0,
        enable_overrun_detection: true,
    });

    let status = status_reader.status();
    assert!(status.is_enabled);

    spi.return_status_reader(status_reader);
    assert!(spi.is_status_reader_available());

    write_str("SPI status reader test finished successfully");
    spi.reset()
}
