use aerugo::hal::{
    drivers::{
        nvic::{Interrupt, NVIC},
        spi::{NotConfigured, Spi},
        xdmac::Xdmac,
    },
    interrupt,
    user_peripherals::SPI0,
};
use calldwell::write_str;

pub fn perform_test(_spi: Spi<SPI0, NotConfigured>, _xdmac: &mut Xdmac, nvic: &mut NVIC) {
    write_str("Beginning LSM6DSO communication tests w/ XDMAC...");

    nvic.enable(Interrupt::XDMAC);

    // TODO: Test code here

    nvic.disable(Interrupt::XDMAC);

    write_str("All LSM6DSO communication tests w/ XDMAC done!");
}

#[interrupt]
fn XDMAC() {}
