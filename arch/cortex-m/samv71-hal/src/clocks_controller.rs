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

pub mod config;
pub mod interrupt;
pub mod status;

pub use interrupt::Interrupts;
pub use status::Status;

use self::config::main_rc::*;
use self::config::master_clock::*;
use self::config::pck::*;
use crate::pac;
use cortex_m::asm;

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
pub struct ClocksController {
    /// PMC instance.
    pmc: pac::PMC,
}

impl ClocksController {
    /// Create new Clock controller instance
    pub const fn new(pmc: pac::PMC) -> Self {
        ClocksController { pmc }
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

    /// Changes main internal RC oscillator frequency.
    /// Will block until the frequency is changed by watching main RC status bit in
    /// PMC status register (effectively clearing some of it's flags, read "Safety" for more details).
    ///
    /// # Parameters
    /// * `frequency` - Target frequency of internal RC oscillator.
    ///                 Can be created from fugit's Megahertz type.
    ///
    /// # Safety
    /// If you'll call this function while crystal oscillator's clock failure detection is active,
    /// and you're not using interrupts to catch clock failure events, and if clock failure happens
    /// during this function, then it's possible that this function will clear it's bit in status register
    /// by reading it, and your code will never know about it. To prevent that issue, use interrupts to
    /// catch clock failure events, and you'll always read status register first.
    pub fn set_main_rc_frequency(&mut self, frequency: MainRcFrequency) {
        // Wait until main RC oscillator is stabilized
        // to make sure it's safe to modify the frequency
        while !self.status().main_rc_stabilized {
            asm::nop();
        }

        self.pmc
            .ckgr_mor
            .modify(|_, w| w.moscrcf().variant(frequency.into()));

        // Wait until main RC oscillator is stabilized
        // to make sure the frequency is stabilized
        while !self.status().main_rc_stabilized {
            asm::nop();
        }
    }

    /// Returns configured internal RC oscillator frequency.
    /// This function does not perform actual clock measurement, it returns the value from the register.
    ///
    /// # Returns
    /// Main RC frequency value. This type can be converted to fugit's Megahertz type
    pub fn main_rc_frequency(&self) -> MainRcFrequency {
        todo!()
    }

    /// Configures master clock (MCK) source, frequency and divider.
    ///
    /// Master clock changes affects:
    /// * Free running clock (FCLK), which has frequency equal to undivided MCK frequency,
    /// * Processor clock (HCLK), which has frequency equal to FCLK,
    /// * SysTick external clock, which has frequency equal to FCLK divided by 8,
    /// * Peripheral clocks, which have frequency equal to MCK.
    ///
    /// For details, consult SAMV71 datasheet, chapter 31 - Power Management Controller.
    ///
    /// # Safety
    /// You **should** follow Recommended Programming Sequence of PMC, described in section 31.17
    /// of SAMV71 datasheet.
    ///
    /// In summary:
    /// * If MAINCK will be used as MCK source, it must be correctly configured.
    ///   If you intend to use crystal oscillator, you must switch MAINCK source to it before calling this function.
    /// * If PLLA will be used as MCK source, it must be correctly configured before calling this function.
    pub fn configure_master_clock(&mut self, _config: MasterClockConfig) {
        todo!()
    }

    /// Returns current master clock configuration.
    pub fn master_clock_config(&self) -> MasterClockConfig {
        todo!()
    }

    /// Returns `true` if processor clock (HCLK) is currently enabled, `false` otherwise.
    pub fn processor_clock_enabled(&self) -> bool {
        todo!()
    }

    /// Returns programmable clocks (PCKs) status in form of a [list](PCKList).
    /// If `self.programmable_clocks_status()[x]` is `true`, then PCKx is enabled.
    pub fn programmable_clocks_status(&self) -> PCKList {
        todo!()
    }

    /// Enables provided programmable clocks (PCKs).
    /// Clocks set to `true` will be enabled, the rest will not be modified.
    pub fn enable_programmable_clocks(&mut self, _clocks: PCKList) {
        todo!()
    }

    /// Disables provided programmable clocks (PCKs).
    /// Clocks set to `true` will be disabled, the rest will not be modified.
    pub fn disable_programmable_clocks(&mut self, _clocks: PCKList) {
        todo!()
    }

    /// Configures a programmable clock (PCK).
    ///
    /// # Parameters
    /// * `clock` - Clock to be configured, only integers in inclusive range [0, 7] are valid.
    /// * `config` - Programmable clock's configuration.
    pub fn configure_programmable_clock(&mut self, _clock: PCK, _config: PCKConfig) {
        todo!()
    }
}
