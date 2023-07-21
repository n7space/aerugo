//! Access to the hardware peripherals.

pub mod watchdog;

use aerugo_cortex_m::Mutex;
use samv71q21_pac::CorePeripherals as cortex_peripherals;
use samv71q21_pac::Peripherals as samv71_peripherals;

/// Mutex indicating whether the peripherals instance have already been taken.
static PERIPHERALS_TAKEN: Mutex<bool> = Mutex::new(false);

/// Peripherals structure.
pub struct Peripherals {
    /// Raw MCU peripherals
    pub mcu_pac: samv71_peripherals,
    /// Raw Cortex-M peripherals
    pub cortex_pac: cortex_peripherals,
}

impl Peripherals {
    /// Create new peripherals instance. This function can be called successfully only once.
    pub fn new() -> Option<Peripherals> {
        PERIPHERALS_TAKEN.lock(|peripherals_taken| {
            if *peripherals_taken {
                return None;
            }

            *peripherals_taken = true;

            Some(Peripherals {
                // SAFETY: This is safe, because it's the only place where MCU peripherals will be stored,
                // and this function is guarded from creating the peripherals instance more than once.
                mcu_pac: unsafe { samv71_peripherals::steal() },
                cortex_pac: cortex_peripherals::take()
                    .expect("Cortex peripherals cannot be taken more than once!"),
            })
        })
    }
}

unsafe impl Sync for Peripherals {}
