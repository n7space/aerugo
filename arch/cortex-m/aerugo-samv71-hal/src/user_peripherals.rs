//! Module representing user-accessible peripherals.

pub use samv71_hal::pac::{CHIPID, CPUID, NVIC, PIOA, PIOB, PIOC, PIOD, PIOE, SCB, TC1, TC2, TC3};
pub use samv71_hal::pmc::PMC;

/// Peripherals structure.
/// These peripherals can be used to create HAL drivers in user code.
///
/// Core peripherals (like PMC) are stored already in form of HAL drivers, instead of
/// PAC instances, as they are core components that most applications will have
/// to create instances of, and use.
pub struct UserPeripherals {
    /// Chip ID.
    pub chip_id: Option<CHIPID>,
    /// CPU ID, required for some SCB-related operations.
    pub cpu_id: Option<CPUID>,
    /// NVIC
    pub nvic: Option<NVIC>,
    /// I/O Port A.
    pub pio_a: Option<PIOA>,
    /// I/O Port B.
    pub pio_b: Option<PIOB>,
    /// I/O Port C.
    pub pio_c: Option<PIOC>,
    /// I/O Port D.
    pub pio_d: Option<PIOD>,
    /// I/O Port E.
    pub pio_e: Option<PIOE>,
    /// Clocks controller.
    /// This is HAL driver instance that provides abstraction over PMC.
    pub pmc: Option<PMC>,
    /// System Control Block
    pub scb: Option<SCB>,
    /// Timer Counter 1.
    pub timer_counter1: Option<TC1>,
    /// Timer Counter 2.
    pub timer_counter2: Option<TC2>,
    /// Timer Counter 3.
    pub timer_counter3: Option<TC3>,
}
