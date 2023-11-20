//! Module with communication tests.
use core::hint::spin_loop;
use core::sync::atomic::{AtomicBool, AtomicU16, Ordering};

use aerugo::hal::{
    drivers::{
        nvic::{Interrupt, NVIC},
        spi::{
            chip_config::{
                BitsPerTransfer, ChipConfig, ChipSelectBehavior, ClockPhase, ClockPolarity,
                SerialClockDivider,
            },
            config::{ChipSelectionDelay, MasterConfig, SelectedChip},
            interrupts::Interrupts,
            reader::Reader,
            status_reader::StatusReader,
            NotConfigured, Spi,
        },
    },
    interrupt,
    user_peripherals::SPI0,
};
use calldwell::write_str;

pub fn perform_test(spi: Spi<SPI0, NotConfigured>, nvic: &mut NVIC) -> Spi<SPI0, NotConfigured> {
    write_str("Beginning SPI communication tests...");

    let spi = perform_synchronous_loopback_transfer_test(spi);
    let spi = perform_interrupt_loopback_transfer_test(spi, nvic);

    write_str("All SPI communication tests done!");
    spi
}

fn perform_synchronous_loopback_transfer_test(
    spi: Spi<SPI0, NotConfigured>,
) -> Spi<SPI0, NotConfigured> {
    write_str("Synchronous transfer test started.");

    let mut spi = spi.into_master(MasterConfig {
        chip_selection_delay: ChipSelectionDelay::new_saturated(0),
        selected_chip: SelectedChip::Chip0,
        enable_overrun_detection: true,
    });

    spi.enable_loopback();
    spi.configure_chip(
        SelectedChip::Chip0,
        ChipConfig {
            clock_polarity: ClockPolarity::LowWhenInactive,
            clock_phase: ClockPhase::DataChangedOnLeadingEdge,
            chip_select_behavior: ChipSelectBehavior::default(),
            bits_per_transfer: BitsPerTransfer::Bits8,
            clock_divider: SerialClockDivider::new_saturated(0),
            delay_before_first_clock: 0,
            delay_between_consecutive_transfers: 0,
        },
    );

    spi.configure_chip(
        SelectedChip::Chip1,
        ChipConfig {
            clock_polarity: ClockPolarity::HighWhenInactive,
            clock_phase: ClockPhase::DataCapturedOnLeadingEdge,
            chip_select_behavior: ChipSelectBehavior::default(),
            bits_per_transfer: BitsPerTransfer::Bits12,
            clock_divider: SerialClockDivider::new_saturated(0),
            delay_before_first_clock: 0,
            delay_between_consecutive_transfers: 0,
        },
    );

    assert!(spi.is_loopback_enabled());
    assert!(spi.is_current_chip_configured());

    let reader = spi.take_reader().unwrap();
    let mut writer = spi.take_writer().unwrap();
    let mut status_reader = spi.take_status_reader().unwrap();

    for byte in 0..=u8::MAX {
        writer.transmit_value(byte as u16);
        status_reader.wait_for_status(|status| status.interrupts.rx_data_register_full, 10_000);
        assert_eq!(reader.get_received_data() as u8, byte);
    }

    write_str("Synchronous 8-bit transfer successful!");

    spi.change_chip(SelectedChip::Chip1);
    assert!(spi.is_loopback_enabled());
    assert!(spi.is_current_chip_configured());

    for value in 0..2u16.pow(12) {
        writer.transmit_value(value);
        status_reader.wait_for_status(|status| status.interrupts.rx_data_register_full, 10_000);
        assert_eq!(reader.get_received_data(), value);
    }

    write_str("Synchronous 12-bit transfer successful!");

    spi.return_reader(reader);
    spi.return_writer(writer);
    spi.return_status_reader(status_reader);

    write_str("Synchronous transfer test successfully finished!");
    spi.reset()
}

fn perform_interrupt_loopback_transfer_test(
    spi: Spi<SPI0, NotConfigured>,
    nvic: &mut NVIC,
) -> Spi<SPI0, NotConfigured> {
    write_str("Interrupt transfer test started.");

    let mut spi = spi.into_master(MasterConfig {
        chip_selection_delay: ChipSelectionDelay::new_saturated(0),
        selected_chip: SelectedChip::Chip0,
        enable_overrun_detection: true,
    });

    spi.enable_loopback();
    spi.configure_chip(
        SelectedChip::Chip0,
        ChipConfig {
            clock_polarity: ClockPolarity::LowWhenInactive,
            clock_phase: ClockPhase::DataChangedOnLeadingEdge,
            chip_select_behavior: ChipSelectBehavior::default(),
            bits_per_transfer: BitsPerTransfer::Bits8,
            clock_divider: SerialClockDivider::new_saturated(0),
            delay_before_first_clock: 0,
            delay_between_consecutive_transfers: 0,
        },
    );

    spi.configure_chip(
        SelectedChip::Chip1,
        ChipConfig {
            clock_polarity: ClockPolarity::HighWhenInactive,
            clock_phase: ClockPhase::DataCapturedOnLeadingEdge,
            chip_select_behavior: ChipSelectBehavior::default(),
            bits_per_transfer: BitsPerTransfer::Bits12,
            clock_divider: SerialClockDivider::new_saturated(0),
            delay_before_first_clock: 0,
            delay_between_consecutive_transfers: 0,
        },
    );

    spi.set_interrupts_state(Interrupts {
        rx_data_register_full: true,
        tx_data_register_empty: false,
        mode_fault_error: true,
        overrun_error: true,
        nss_rising: false,
        tx_registers_empty: false,
        underrun_error: false,
    });

    assert!(spi.is_loopback_enabled());
    assert!(spi.is_current_chip_configured());

    let reader = spi.take_reader().unwrap();
    let mut writer = spi.take_writer().unwrap();
    let status_reader = spi.take_status_reader().unwrap();

    unsafe {
        STATUS_READER.replace(status_reader);
        SPI_READER.replace(reader);
    }

    nvic.enable(Interrupt::SPI0);

    for byte in 0..=u8::MAX {
        writer.transmit_value(byte as u16);
        assert_eq!(wait_for_spi_data() as u8, byte);
    }

    write_str("Interrupt 8-bit transfer successful!");

    spi.change_chip(SelectedChip::Chip1);
    assert!(spi.is_loopback_enabled());
    assert!(spi.is_current_chip_configured());

    for value in 0..2u16.pow(12) {
        writer.transmit_value(value);
        assert_eq!(wait_for_spi_data(), value);
    }

    write_str("Interrupt 12-bit transfer successful!");

    nvic.disable(Interrupt::SPI0);

    spi.return_reader(unsafe { SPI_READER.take().unwrap() });
    spi.return_writer(writer);
    spi.return_status_reader(unsafe { STATUS_READER.take().unwrap() });

    write_str("Interrupt transfer test successfully finished!");
    spi.reset()
}

static mut STATUS_READER: Option<StatusReader<SPI0>> = None;
static mut SPI_READER: Option<Reader<SPI0>> = None;

static SPI_DATA: AtomicU16 = AtomicU16::new(0);
static SPI_DATA_READY: AtomicBool = AtomicBool::new(false);

fn wait_for_spi_data() -> u16 {
    while !SPI_DATA_READY.load(Ordering::SeqCst) {
        spin_loop();
    }
    SPI_DATA_READY.store(false, Ordering::SeqCst);
    SPI_DATA.load(Ordering::SeqCst)
}

#[interrupt]
fn SPI0() {
    let status = unsafe { STATUS_READER.as_mut().unwrap().status() };

    if status.interrupts.mode_fault_error {
        panic!("mode fault error detected");
    }

    if status.interrupts.overrun_error {
        panic!("overrun error detected");
    }

    if status.interrupts.rx_data_register_full {
        SPI_DATA.store(
            unsafe { SPI_READER.as_mut().unwrap().get_received_data() },
            Ordering::SeqCst,
        );
        SPI_DATA_READY.store(true, Ordering::SeqCst);
    }
}
