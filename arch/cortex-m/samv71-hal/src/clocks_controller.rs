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
use self::config::peripheral::*;
use crate::pac::PMC;
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
    pmc: PMC,
}

impl ClocksController {
    /// Create new Clock controller instance
    pub const fn new(pmc: PMC) -> Self {
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
        // Make sure that main RC oscillator is ready to be changed
        self.wait_until_main_rc_stabilizes();

        self.pmc
            .ckgr_mor
            .modify(|_, w| w.moscrcf().variant(frequency.into()));

        // Wait until the changes take effect
        self.wait_until_main_rc_stabilizes();
    }

    /// Returns configured internal RC oscillator frequency.
    /// This function does not perform actual clock measurement, it returns the value from the register.
    ///
    /// # Returns
    /// Main RC frequency value. This type can be converted to fugit's Megahertz type
    ///
    /// # Remarks
    /// This function will panic if an unexpected value is read from register. It should never
    /// happen under normal circumstances, assuming that user doesn't try to manually write
    /// an invalid value to this register and that there's no unexpected hardware issue.
    pub fn main_rc_frequency(&self) -> MainRcFrequency {
        // If an invalid value is read from here, it's either a very nasty user error or
        // possible hardware bug/issue, so it should crash program as it's not intended to
        // ever happen.
        self.pmc
            .ckgr_mor
            .read()
            .moscrcf()
            .variant()
            .expect("invalid value of main RC oscillator frequency read from PMC main oscillator register")
            .into()
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
    pub fn configure_master_clock(&mut self, config: MasterClockConfig) {
        // Make sure that master clock is ready for changes
        self.wait_until_master_clock_is_ready();

        self.pmc.mckr.modify(|_, w| {
            w.css()
                .variant(config.source.into())
                .pres()
                .variant(config.prescaler.into())
                .mdiv()
                .variant(config.divider.into())
        });

        // Wait until the changes take effect
        self.wait_until_master_clock_is_ready();
    }

    /// Returns current master clock configuration.
    pub fn master_clock_config(&self) -> MasterClockConfig {
        let reg = self.pmc.mckr.read();

        MasterClockConfig {
            source: reg.css().variant().into(),
            prescaler: reg.pres().variant().into(),
            divider: reg.mdiv().variant().into(),
        }
    }

    /// Returns `true` if processor clock (HCLK) is currently enabled, `false` otherwise.
    pub fn processor_clock_enabled(&self) -> bool {
        self.pmc.scsr.read().hclks().bit_is_set()
    }

    /// Returns programmable clocks (PCKs) status in form of a [list](PCKList).
    /// If `self.programmable_clocks_status()[x]` is `true`, then PCKx is enabled.
    pub fn programmable_clocks_status(&self) -> PCKList {
        let reg = self.pmc.scsr.read();

        [
            reg.pck0().bit_is_set(),
            reg.pck1().bit_is_set(),
            reg.pck2().bit_is_set(),
            reg.pck3().bit_is_set(),
            reg.pck4().bit_is_set(),
            reg.pck5().bit_is_set(),
            reg.pck6().bit_is_set(),
            reg.pck7().bit_is_set(),
        ]
    }

    /// Enables provided programmable clock (PCK).
    pub fn enable_programmable_clock(&mut self, clock: PCK) {
        /// PMC bits in System Clock Enable Register start at 8th bit
        const PMC_BITS_START: u32 = 8;
        // Since `clock` has PCK type, it's underlying value is always an index of PCK,
        // so we can safely shift the bit to the left using it as an offset.
        let pck_mask = 1u32 << (PMC_BITS_START + (clock as u32));
        self.pmc.scer.write(|w| unsafe { w.bits(pck_mask) });
    }

    /// Disables provided programmable clock (PCK).
    pub fn disable_programmable_clock(&mut self, clock: PCK) {
        /// PMC bits in System Clock Disable Register start at 8th bit
        const PMC_BITS_START: u32 = 8;
        // Since `clock` has PCK type, it's underlying value is always an index of PCK,
        // so we can safely shift the bit to the left using it as an offset.
        let pck_mask = 1u32 << (PMC_BITS_START + (clock as u32));
        self.pmc.scdr.write(|w| unsafe { w.bits(pck_mask) });
    }

    /// Configures a programmable clock (PCK).
    ///
    /// # Parameters
    /// * `clock` - Clock to be configured.
    /// * `config` - Programmable clock's configuration.
    pub fn configure_programmable_clock(&mut self, clock: PCK, config: PCKConfig) {
        self.pmc.pck[clock as usize].write(|w| {
            w.css()
                .variant(config.source.into())
                .pres()
                .variant(config.prescaler.into_register_value())
        });

        // Wait until it's ready
        while !self.programmable_clocks_status()[clock as usize] {
            asm::nop();
        }
    }

    /// Configures peripheral clock.
    /// Can be used to disable or enable peripheral and generic clocks,
    /// and to configure generic clock's source and divider.
    ///
    /// # Parameters
    /// * `peripheral` - Peripheral which clocks will be configured.
    /// * `config` - Clock configuration.
    pub fn configure_peripheral_clocks(
        &mut self,
        peripheral: PeripheralId,
        config: PeripheralClockConfig,
    ) {
        let clock_id = peripheral as u8;
        let current_config = self.peripheral_clocks_config(peripheral);

        // Since changing generic clock divider along with other generic clock parameters is unsafe,
        // it needs to be done smart.
        let state_changed = current_config.generic_clock.enabled != config.generic_clock.enabled;
        let source_changed = current_config.generic_clock.source != config.generic_clock.source;
        let divider_changed = current_config.generic_clock.divider != config.generic_clock.divider;

        // If either generic clock's state or source changes is changed with the divider,
        // divider must be configured in second write operation according to datasheet.
        let second_write_needed = (state_changed || source_changed) && divider_changed;

        self.pmc.pcr.write(|w| {
            let w = w
                .pid()
                .variant(clock_id)
                .gclkcss()
                .variant(config.generic_clock.source.into())
                .cmd()
                .set_bit()
                .en()
                .variant(config.enabled)
                .gclken()
                .variant(config.generic_clock.enabled);

            // If both generic clock source and state is unchanged, then generic clock
            // divider can be safely set in this operation.
            if !second_write_needed {
                w.gclkdiv()
                    .variant(config.generic_clock.divider.into_register_value())
            } else {
                w.gclkdiv()
                    .variant(current_config.generic_clock.divider.into_register_value())
            }
        });

        // If either generic clock or source has changed, and clock divider is supposed to
        // change too, perform second write.
        //
        // `modify` cannot be used here, because in order to read correct values from this register,
        // it must be written with "CMD" bit equal to 0 first, so an explicit read + conversion
        // would have to be applied, and according to datasheet "All configuration fields must be
        // correctly set". It's shorter to copy the previous call and add the divider setting without check :)
        if second_write_needed {
            self.pmc.pcr.write(|w| {
                w.pid()
                    .variant(clock_id)
                    .gclkcss()
                    .variant(config.generic_clock.source.into())
                    .cmd()
                    .set_bit()
                    .gclkdiv()
                    .variant(config.generic_clock.divider.into_register_value())
                    .en()
                    .variant(config.enabled)
                    .gclken()
                    .variant(config.generic_clock.enabled)
            });
        }
    }

    /// Returns configuration of clocks for specified peripheral.
    ///
    /// # Parameters
    /// * `peripheral` - Peripheral which clocks configuration will be returned.
    ///
    /// # Remarks
    /// This function will panic if an unexpected value is read from register. It should never
    /// happen under normal circumstances, assuming that user doesn't try to manually write
    /// an invalid value to this register and that there's no unexpected hardware issue.
    pub fn peripheral_clocks_config(&self, peripheral: PeripheralId) -> PeripheralClockConfig {
        let clock_id = peripheral as u8;
        // This register needs to be written with peripheral ID and command bit set to 0,
        // to fill it's content with configuration for specified peripheral.
        self.pmc
            .pcr
            .write(|w| w.pid().variant(clock_id).cmd().clear_bit());

        let reg = self.pmc.pcr.read();

        PeripheralClockConfig {
            enabled: reg.en().bit_is_set(),
            generic_clock: GenericClockConfig {
                enabled: reg.gclken().bit_is_set(),
                source: reg.gclkcss().variant().expect("invalid value of generic clock source read from PMC peripheral clocks configuration register").into(),
                divider: GenericClockDivider::from_register_value(reg.gclkdiv().bits()),
            }
        }
    }

    /// Blocks current thread until main RC oscillator is stabilized.
    fn wait_until_main_rc_stabilizes(&mut self) {
        while !self.status().main_rc_stabilized {
            asm::nop();
        }
    }

    /// Blocks current thread until Master Clock becomes ready.
    fn wait_until_master_clock_is_ready(&mut self) {
        while !self.status().master_clock_ready {
            asm::nop();
        }
    }
}
