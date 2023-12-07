//! Floating Point Unit HAL driver implementation.

use cortex_m::asm::{dsb, isb};
use cortex_m::register::fpscr::{read as read_fpscr, write as write_fpscr, Fpscr};
use samv71q21_pac::{FPU, SCB};

mod registers;

/*
SRS list:
- Support for default NaN operation mode
- Support for full-compliance IEEE operation mode
- Support for flush-to-zero operation mode
- Support for handling invalid operands in default NaN/flush-to-zero mode
- Lazy stacking enabled by default (LSPEN)
*/

/// Structure representing Floating Point Unit
pub struct Fpu {
    /// PAC FPU driver instance.
    fpu: FPU,
}

impl Fpu {
    /// Creates new instance of FPU driver and consumes PAC FPU instance.
    pub fn new(fpu: FPU) -> Self {
        Fpu { fpu }
    }

    /// Enable the FPU.
    /// You can use SCB for that instead, but you must provide memory barrier manually.
    pub fn enable(&mut self, scb: &mut SCB) {
        scb.enable_fpu();
        dsb();
        isb();
    }

    /// Disable the FPU. In addition to
    /// You can use SCB for that instead, but you must provide memory barrier manually.
    pub fn disable(&mut self, scb: &mut SCB) {
        scb.disable_fpu();
        dsb();
        isb();
    }
}
