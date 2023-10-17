#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

mod config_tests;
mod uart_tests;

use aerugo::{
    hal::{
        drivers::{
            nvic::NVIC,
            pio::{pin::Peripheral, Port},
            pmc::config::{
                pck::{PCKConfig, PCKPrescaler, PCKSource, PCK},
                PeripheralId,
            },
            uart::UART,
        },
        user_peripherals::{PIOD, PMC},
    },
    Aerugo, InitApi, SystemHardwareConfig,
};
use calldwell::write_str;
use rt::entry;

fn configure_pmc(pmc: &mut PMC) {
    // Enable peripheral clocks
    pmc.enable_peripheral_clock(PeripheralId::PIOD);
    pmc.enable_peripheral_clock(PeripheralId::UART4);

    // Configure and enable PCK4 for UART.
    // Per datasheet, *peripheral clock frequency must be at least three times
    // higher than PCK*. Therefore, PCK frequency cannot be larger than 4MHz,
    // as out peripheral clock is at 12MHz.
    // Source: MAINCK, 12MHz from internal RC oscillator
    // Prescaler: /3
    // Output frequency: 4MHz
    pmc.configure_programmable_clock(
        PCK::PCK4,
        PCKConfig {
            source: PCKSource::MainClock,
            prescaler: PCKPrescaler::new(3).unwrap(),
        },
    );
    pmc.enable_programmable_clock(PCK::PCK4);
}

fn configure_pio(port: Port<PIOD>) {
    let mut pins = port.into_pins();
    pins[18].take().unwrap().into_peripheral_pin(Peripheral::C);
    pins[19].take().unwrap().into_peripheral_pin(Peripheral::C);
}

#[entry]
fn main() -> ! {
    calldwell::start_session();
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());

    // Initialize peripheral clocks.
    let mut pmc = peripherals.pmc.take().unwrap();
    configure_pmc(&mut pmc);

    let nvic = NVIC::new(peripherals.nvic.take().unwrap());

    let port_d = Port::new(peripherals.pio_d.take().unwrap());
    configure_pio(port_d);

    let uart = UART::new(peripherals.uart_4.take().unwrap());
    config_tests::test_uart_config();
    uart_tests::test_uart(uart, nvic);

    write_str("All UART tests finished successfully.");

    aerugo.start();
}
