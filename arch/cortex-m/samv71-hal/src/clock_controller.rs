//! Implementation of HAL Clock Controller driver.
//! This driver provides an abstraction over Power Management Controller (PMC).
//!
//! Difference in the name of this module is due to the fact that "Power" Management
//! Controller original name reflects it's actual usage quite inaccurately.
//!
//! Power Management Controller (PMC) controls all the SAMV71 MCU clocks.
//! You can use it to configure system clocks and PLLs, and to configure/disable/enable
//! peripheral's clocks.
//!
//! This module can also calculate some of the clock's frequencies.
pub mod interrupt;
pub mod status;

use crate::pac;
pub use interrupt::Interrupts;
pub use status::Status;

/// Structure representing Power Management Controller (PMC).
///
/// It's instance can be used to
/// * Change internal RC oscillator frequency
/// * Configure Main Clock (MAINCK) source
/// * Configure Master Clock (MCK)
/// * Enable/disable/configure peripheral clocks
/// * Enable/disable/configure independent programmable clocks
///
/// This structure is not thread/interrupt-safe, as it uses shared state (registers).
/// If you need to share it, wrap it in a proper container that implements [`Sync`].
pub struct ClockController {
    /// PMC instance.
    pmc: pac::PMC,
}

impl ClockController {
    /// Create new Clock controller instance
    pub const fn new(pmc: pac::PMC) -> Self {
        ClockController { pmc }
    }

    /// Returns current PMC status.
    ///
    /// # Safety
    /// Calling this function will clear PMC status register flags, therefore it's marked as mutable.
    /// Be very careful with this function if you're using it both in PMC interrupt context and outside of it.
    pub fn status(&mut self) -> Status {
        let sr = self.pmc.sr.read();

        Status {
            mco_stabilized: sr.moscxts().bit_is_set(),
            plla_locked: sr.locka().bit_is_set(),
            master_clock_ready: sr.mckrdy().bit_is_set(),
            utmi_pll_locked: sr.locku().bit_is_set(),
            slow_clock_source: sr.oscsels().bit().into(),
            pck_ready: [
                sr.pckrdy0().bit_is_set(),
                sr.pckrdy1().bit_is_set(),
                sr.pckrdy2().bit_is_set(),
                sr.pckrdy3().bit_is_set(),
                sr.pckrdy4().bit_is_set(),
                sr.pckrdy5().bit_is_set(),
                sr.pckrdy6().bit_is_set(),
                sr.pckrdy7().bit_is_set(),
            ],
            main_clock_selected: sr.moscsels().bit_is_set(),
            main_rc_stabilized: sr.moscrcs().bit_is_set(),
            mco_status: status::MCOStatus {
                clock_failure_event_detected: sr.cfdev().bit_is_set(),
                clock_failure_currently_detected: sr.cfds().bit_is_set(),
                clock_failure_fault_active: sr.fos().bit_is_set(),
            },
            slow_oscillator_error: sr.xt32kerr().bit_is_set(),
        }
    }

    /// Enables selected interrupts.
    ///
    /// State of other interrupts will not be changed.
    ///
    /// # Parameters
    /// * `interrupts` - Structure with interrupts to be enabled. All interrupts set
    ///                  to `true` will be enabled.
    pub fn enable_interrupts(&mut self, interrupts: Interrupts) {
        self.pmc.ier.write(|w| {
            w.moscxts()
                .variant(interrupts.mco_stabilized)
                .locka()
                .variant(interrupts.plla_locked)
                .mckrdy()
                .variant(interrupts.master_clock_ready)
                .locku()
                .variant(interrupts.utmi_pll_locked)
                .pckrdy0()
                .variant(interrupts.pck_ready[0])
                .pckrdy1()
                .variant(interrupts.pck_ready[1])
                .pckrdy2()
                .variant(interrupts.pck_ready[2])
                .pckrdy3()
                .variant(interrupts.pck_ready[3])
                .pckrdy4()
                .variant(interrupts.pck_ready[4])
                .pckrdy5()
                .variant(interrupts.pck_ready[5])
                .pckrdy6()
                .variant(interrupts.pck_ready[6])
                .pckrdy7()
                .variant(interrupts.pck_ready[7])
                .moscsels()
                .variant(interrupts.main_clock_selected)
                .moscrcs()
                .variant(interrupts.main_rc_stabilized)
                .cfdev()
                .variant(interrupts.mco_failure_event_detected)
                .xt32kerr()
                .variant(interrupts.slow_oscillator_error)
        })
    }

    /// Disables selected interrupts.
    ///
    /// State of other interrupts will not be changed.
    ///
    /// # Parameters
    /// * `interrupts` - Structure with interrupts to be disabled. All interrupts set
    ///                  to `true` will be disabled.
    pub fn disable_interrupts(&mut self, interrupts: Interrupts) {
        self.pmc.idr.write(|w| {
            w.moscxts()
                .variant(interrupts.mco_stabilized)
                .locka()
                .variant(interrupts.plla_locked)
                .mckrdy()
                .variant(interrupts.master_clock_ready)
                .locku()
                .variant(interrupts.utmi_pll_locked)
                .pckrdy0()
                .variant(interrupts.pck_ready[0])
                .pckrdy1()
                .variant(interrupts.pck_ready[1])
                .pckrdy2()
                .variant(interrupts.pck_ready[2])
                .pckrdy3()
                .variant(interrupts.pck_ready[3])
                .pckrdy4()
                .variant(interrupts.pck_ready[4])
                .pckrdy5()
                .variant(interrupts.pck_ready[5])
                .pckrdy6()
                .variant(interrupts.pck_ready[6])
                .pckrdy7()
                .variant(interrupts.pck_ready[7])
                .moscsels()
                .variant(interrupts.main_clock_selected)
                .moscrcs()
                .variant(interrupts.main_rc_stabilized)
                .cfdev()
                .variant(interrupts.mco_failure_event_detected)
                .xt32kerr()
                .variant(interrupts.slow_oscillator_error)
        })
    }

    /// Returns the status (enabled/disabled) of channel's interrupts.
    pub fn interrupts_masks(&self) -> Interrupts {
        let masks = self.pmc.imr.read();

        Interrupts {
            mco_stabilized: masks.moscxts().bit_is_set(),
            plla_locked: masks.locka().bit_is_set(),
            master_clock_ready: masks.mckrdy().bit_is_set(),
            utmi_pll_locked: masks.locku().bit_is_set(),
            pck_ready: [
                masks.pckrdy0().bit_is_set(),
                masks.pckrdy1().bit_is_set(),
                masks.pckrdy2().bit_is_set(),
                masks.pckrdy3().bit_is_set(),
                masks.pckrdy4().bit_is_set(),
                masks.pckrdy5().bit_is_set(),
                masks.pckrdy6().bit_is_set(),
                masks.pckrdy7().bit_is_set(),
            ],
            main_clock_selected: masks.moscsels().bit_is_set(),
            main_rc_stabilized: masks.moscrcs().bit_is_set(),
            mco_failure_event_detected: masks.cfdev().bit_is_set(),
            slow_oscillator_error: masks.xt32kerr().bit_is_set(),
        }
    }
}
