//! Module representing user-accessible peripherals.

use pac;

/// Peripherals structure.
/// These peripherals can be used to create HAL drivers in user code.
pub struct UserPeripherals {
    /// Chip ID.
    pub chip_id: Option<pac::CHIPID>,
}
