//! Module with SPI configuration tests
use aerugo::hal::{
    drivers::spi::{
        chip_config::{
            BitsPerTransfer, ChipConfig, ChipSelectBehavior, ClockPhase, ClockPolarity,
            SerialClockDivider,
        },
        config::{ChipSelectionDelay, MasterConfig, SelectedChip},
        interrupts::Interrupts,
        NotConfigured, Spi,
    },
    user_peripherals::SPI0,
};
use calldwell::write_str;

pub fn perform_test(spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, NotConfigured> {
    write_str("Beginning SPI configuration tests...");

    let spi = perform_master_mode_config_test(spi);
    let spi = perform_status_reader_test(spi);
    let spi = perform_irq_config_test(spi);
    let spi = perform_chip_config_test(spi);
    let spi = perform_loopback_config_test(spi);
    let spi = perform_reader_writer_taking_test(spi);

    write_str("All SPI configuration tests done!");
    spi
}

fn perform_master_mode_config_test(spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, NotConfigured> {
    write_str("SPI master mode config test started.");

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

    write_str("SPI master mode config test finished successfully!");
    spi.reset()
}

fn perform_status_reader_test(mut spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, NotConfigured> {
    write_str("SPI status reader test started.");

    assert!(spi.is_status_reader_available());
    let status_reader = spi.take_status_reader().unwrap();
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

    write_str("SPI status reader test finished successfully!");
    spi.reset()
}

fn perform_irq_config_test(spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, NotConfigured> {
    write_str("SPI interrupt config test started.");
    let mut spi = spi.into_master(MasterConfig {
        chip_selection_delay: ChipSelectionDelay::new_saturated(0),
        selected_chip: SelectedChip::Chip0,
        enable_overrun_detection: true,
    });

    let irq_a = Interrupts {
        rx_data_register_full: false,
        tx_data_register_empty: true,
        mode_fault_error: false,
        overrun_error: true,
        nss_rising: false,
        tx_registers_empty: true,
        underrun_error: false,
    };

    let irq_b = Interrupts {
        rx_data_register_full: true,
        tx_data_register_empty: false,
        mode_fault_error: true,
        overrun_error: false,
        nss_rising: true,
        tx_registers_empty: false,
        underrun_error: true,
    };

    let irq_all = Interrupts {
        rx_data_register_full: true,
        tx_data_register_empty: true,
        mode_fault_error: true,
        overrun_error: true,
        nss_rising: true,
        tx_registers_empty: true,
        underrun_error: true,
    };

    let irq_none = Interrupts {
        rx_data_register_full: false,
        tx_data_register_empty: false,
        mode_fault_error: false,
        overrun_error: false,
        nss_rising: false,
        tx_registers_empty: false,
        underrun_error: false,
    };

    spi.set_interrupts_state(irq_a);
    assert_eq!(spi.interrupts_state(), irq_a);
    spi.set_interrupts_state(irq_b);
    assert_eq!(spi.interrupts_state(), irq_b);
    spi.set_interrupts_state(irq_all);
    assert_eq!(spi.interrupts_state(), irq_all);
    spi.set_interrupts_state(irq_none);
    assert_eq!(spi.interrupts_state(), irq_none);

    write_str("SPI interrupt config test finished successfully!");
    spi.reset()
}

fn perform_chip_config_test(spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, NotConfigured> {
    write_str("SPI chip config test started.");

    let mut spi = spi.into_master(MasterConfig {
        chip_selection_delay: ChipSelectionDelay::new_saturated(0),
        selected_chip: SelectedChip::Chip0,
        enable_overrun_detection: true,
    });

    let chip_config_a = ChipConfig {
        clock_polarity: ClockPolarity::HighWhenInactive,
        clock_phase: ClockPhase::DataCapturedOnLeadingEdge,
        chip_select_behavior: ChipSelectBehavior::DeactivateAfterLastTransfer,
        bits_per_transfer: BitsPerTransfer::Bits8,
        clock_divider: SerialClockDivider::new(100).unwrap(),
        delay_before_first_clock: 10,
        delay_between_consecutive_transfers: 20,
    };

    let chip_config_b = ChipConfig {
        clock_polarity: ClockPolarity::LowWhenInactive,
        clock_phase: ClockPhase::DataChangedOnLeadingEdge,
        chip_select_behavior: ChipSelectBehavior::KeepActive,
        bits_per_transfer: BitsPerTransfer::Bits13,
        clock_divider: SerialClockDivider::new(123).unwrap(),
        delay_before_first_clock: 42,
        delay_between_consecutive_transfers: 234,
    };

    assert!(!spi.is_chip_configured(SelectedChip::Chip0));
    assert!(!spi.is_chip_configured(SelectedChip::Chip1));
    assert!(!spi.is_chip_configured(SelectedChip::Chip2));
    assert!(!spi.is_chip_configured(SelectedChip::Chip3));
    assert!(!spi.is_current_chip_configured());

    spi.configure_chip(SelectedChip::Chip0, chip_config_a);
    spi.configure_chip(SelectedChip::Chip3, chip_config_b);

    assert!(spi.is_chip_configured(SelectedChip::Chip0));
    assert!(!spi.is_chip_configured(SelectedChip::Chip1));
    assert!(!spi.is_chip_configured(SelectedChip::Chip2));
    assert!(spi.is_chip_configured(SelectedChip::Chip3));

    assert!(spi.is_current_chip_configured());
    spi.change_chip(SelectedChip::Chip1);
    assert!(!spi.is_current_chip_configured());
    spi.change_chip(SelectedChip::Chip2);
    assert!(!spi.is_current_chip_configured());
    spi.change_chip(SelectedChip::Chip3);
    assert!(spi.is_current_chip_configured());
    spi.change_chip(SelectedChip::Chip0);
    assert!(spi.is_current_chip_configured());

    assert_eq!(spi.chip_config(SelectedChip::Chip0).unwrap(), chip_config_a);
    assert!(spi.chip_config(SelectedChip::Chip1).is_none());
    assert!(spi.chip_config(SelectedChip::Chip2).is_none());
    assert_eq!(spi.chip_config(SelectedChip::Chip3).unwrap(), chip_config_b);

    write_str("SPI chip config test finished successfully!");
    spi.reset()
}

fn perform_loopback_config_test(spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, NotConfigured> {
    write_str("SPI loopback config test started.");

    let mut spi = spi.into_master(MasterConfig {
        chip_selection_delay: ChipSelectionDelay::new_saturated(0),
        selected_chip: SelectedChip::Chip0,
        enable_overrun_detection: true,
    });

    spi.disable_loopback();
    assert!(!spi.is_loopback_enabled());
    spi.enable_loopback();
    assert!(spi.is_loopback_enabled());
    spi.disable_loopback();
    assert!(!spi.is_loopback_enabled());

    write_str("SPI loopback config test finished successfully!");
    spi.reset()
}

fn perform_reader_writer_taking_test(spi: Spi<SPI0, NotConfigured>) -> Spi<SPI0, NotConfigured> {
    write_str("SPI reader/writer taking test started.");

    let mut spi = spi.into_master(MasterConfig {
        chip_selection_delay: ChipSelectionDelay::new_saturated(0),
        selected_chip: SelectedChip::Chip0,
        enable_overrun_detection: true,
    });

    assert!(spi.is_reader_available());
    let reader = spi.take_reader().unwrap();
    assert!(!spi.is_reader_available());
    spi.return_reader(reader);
    assert!(spi.is_reader_available());

    assert!(spi.is_writer_available());
    let writer = spi.take_writer().unwrap();
    assert!(!spi.is_writer_available());
    spi.return_writer(writer);
    assert!(spi.is_writer_available());

    write_str("SPI reader/writer taking test finished successfully!");
    spi.reset()
}
