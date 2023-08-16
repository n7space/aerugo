//! Module representing user-accessible peripherals.

use pac;

/// Peripherals structure.
/// These peripherals can be used to create HAL drivers in user code.
pub struct UserPeripherals {
    /// Chip ID.
    pub chip_id: Option<pac::CHIPID>,
    /// Timer Counter 1.
    pub timer_counter1: Option<pac::TC1>,
    /// Timer Counter 2.
    pub timer_counter2: Option<pac::TC2>,
    /// Timer Counter 3.
    pub timer_counter3: Option<pac::TC3>,
    /// PMC
    pub pmc: Option<pac::PMC>,
}
