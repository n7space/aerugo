//! Floating Point Unit HAL driver implementation.

use cortex_m::asm::{dsb, isb};
use cortex_m::register::fpscr::{read as read_fpscr, write as write_fpscr};
use samv71q21_pac::{FPU, SCB};

use self::registers::{
    Config, ContextConfig, ContextStateFlags, FlushToZeroMode, HalfPrecisionMode, NaNMode,
    RoundingMode, Status, FPU_FPCAR_ADDRESS_MASK, FPU_FPCAR_ADDRESS_OFFSET, FPU_FPCCR_ASPEN_MASK,
    FPU_FPCCR_BFRDY_MASK, FPU_FPCCR_HFRDY_MASK, FPU_FPCCR_LSPACT_MASK, FPU_FPCCR_LSPEN_MASK,
    FPU_FPCCR_MMRDY_MASK, FPU_FPCCR_MONRDY_MASK, FPU_FPCCR_THREAD_MASK, FPU_FPCCR_USER_MASK,
    FPU_FPDSCR_AHP_MASK, FPU_FPDSCR_AHP_OFFSET, FPU_FPDSCR_DN_MASK, FPU_FPDSCR_DN_OFFSET,
    FPU_FPDSCR_FZ_MASK, FPU_FPDSCR_FZ_OFFSET, FPU_FPDSCR_RMODE_MASK, FPU_FPDSCR_RMODE_OFFSET,
};

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

    /// Enables the FPU and it's lazy stacking feature.
    pub fn enable(&mut self, scb: &mut SCB) {
        scb.enable_fpu();
        dsb();
        isb();
        unsafe { self.fpu.fpccr.modify(|reg| reg | FPU_FPCCR_LSPEN_MASK) };
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

    /// Sets FPU configuration.
    pub fn set_config(&mut self, config: Config) {
        unsafe {
            self.fpu.fpccr.modify(|reg| {
                if config.is_context_preserved_on_exception {
                    reg | FPU_FPCCR_ASPEN_MASK
                } else {
                    reg & !FPU_FPCCR_ASPEN_MASK
                }
            })
        };

        unsafe {
            self.fpu.fpcar.modify(|reg| {
                let clean_reg = reg & !FPU_FPCAR_ADDRESS_MASK;
                let address_bits = (config.exception_register_space_address
                    << FPU_FPCAR_ADDRESS_OFFSET)
                    & FPU_FPCAR_ADDRESS_MASK;
                clean_reg | address_bits
            })
        };

        unsafe {
            self.fpu.fpdscr.modify(|reg| {
                let mask = FPU_FPDSCR_AHP_MASK
                    | FPU_FPDSCR_DN_MASK
                    | FPU_FPDSCR_FZ_MASK
                    | FPU_FPDSCR_RMODE_MASK;
                let clean_reg = reg & !mask;
                let ahp_bits = (config.default_context_config.half_precision_mode as u32)
                    << FPU_FPDSCR_AHP_OFFSET;
                let dn_bits =
                    (config.default_context_config.nan_mode as u32) << FPU_FPDSCR_DN_OFFSET;
                let fz_bits = (config.default_context_config.flush_to_zero_mode as u32)
                    << FPU_FPDSCR_FZ_OFFSET;
                let rmode_bits =
                    (config.default_context_config.rounding_mode as u32) << FPU_FPDSCR_RMODE_OFFSET;
                clean_reg | ahp_bits | dn_bits | fz_bits | rmode_bits
            })
        };

        dsb();
        isb();
    }

    /// Returns current FPU configuration.
    pub fn get_config(&mut self) -> Config {
        let fpccr = self.fpu.fpccr.read();
        let fpcar = self.fpu.fpcar.read();
        let fpdscr = self.fpu.fpdscr.read();

        let is_context_preserved_on_exception = (fpccr & FPU_FPCCR_ASPEN_MASK) != 0;
        let exception_register_space_address =
            (fpcar & FPU_FPCAR_ADDRESS_MASK) >> FPU_FPCAR_ADDRESS_OFFSET;
        let half_precision_mode =
            HalfPrecisionMode::try_from((fpdscr & FPU_FPDSCR_AHP_MASK) >> FPU_FPDSCR_AHP_OFFSET)
                .unwrap();
        let nan_mode =
            NaNMode::try_from((fpdscr & FPU_FPDSCR_DN_MASK) >> FPU_FPDSCR_DN_OFFSET).unwrap();
        let flush_to_zero_mode =
            FlushToZeroMode::try_from((fpdscr & FPU_FPDSCR_FZ_MASK) >> FPU_FPDSCR_FZ_OFFSET)
                .unwrap();
        let rounding_mode =
            RoundingMode::try_from((fpdscr * FPU_FPDSCR_RMODE_MASK) >> FPU_FPDSCR_RMODE_OFFSET)
                .unwrap();

        Config {
            is_context_preserved_on_exception,
            exception_register_space_address,
            default_context_config: ContextConfig {
                half_precision_mode,
                nan_mode,
                flush_to_zero_mode,
                rounding_mode,
            },
        }
    }

    /// Returns current status of FPU
    pub fn get_status(&self) -> Status {
        let fpccr = self.fpu.fpccr.read();

        Status {
            could_debug_monitor_exception_pending_be_set: (fpccr & FPU_FPCCR_MONRDY_MASK) != 0,
            could_bus_fault_exception_pending_be_set: (fpccr & FPU_FPCCR_BFRDY_MASK) != 0,
            could_mem_manage_exception_pending_be_set: (fpccr & FPU_FPCCR_MMRDY_MASK) != 0,
            could_hard_fault_exception_pending_be_set: (fpccr & FPU_FPCCR_HFRDY_MASK) != 0,
            was_processor_in_thread_mode: (fpccr & FPU_FPCCR_THREAD_MASK) != 0,
            was_processor_in_user_mode: (fpccr & FPU_FPCCR_USER_MASK) != 0,
            is_lazy_fp_state_preservation_active: (fpccr & FPU_FPCCR_LSPACT_MASK) != 0,
        }
    }

    /// Sets FPU context configuration
    pub fn set_context_config(&mut self, config: ContextConfig) {
        let mut fpscr = read_fpscr();
        fpscr.set_ahp(config.half_precision_mode as u32 != 0);
        fpscr.set_dn(config.nan_mode as u32 != 0);
        fpscr.set_fz(config.flush_to_zero_mode as u32 != 0);
        fpscr.set_rmode(config.rounding_mode.into());

        unsafe { write_fpscr(fpscr) };
        dsb();
        isb();
    }

    /// Returns FPU context configuration
    pub fn context_config(&self) -> ContextConfig {
        let fpscr = read_fpscr();

        ContextConfig {
            half_precision_mode: fpscr.ahp().into(),
            nan_mode: fpscr.dn().into(),
            flush_to_zero_mode: fpscr.fz().into(),
            rounding_mode: fpscr.rmode().into(),
        }
    }

    /// Returns FPU context state
    pub fn context_state(&self) -> ContextStateFlags {
        let fpscr = read_fpscr();

        ContextStateFlags {
            negative_condition: fpscr.n(),
            zero_condition: fpscr.z(),
            carry_condition: fpscr.c(),
            overflow_condition: fpscr.v(),
            input_denormal_exception: fpscr.idc(),
            inexact_exception: fpscr.ixc(),
            underflow_exception: fpscr.ufc(),
            overflow_exception: fpscr.ofc(),
            division_by_zero_exception: fpscr.dzc(),
            invalid_operation_exception: fpscr.ioc(),
        }
    }

    /// Clears FPU exceptions
    pub fn clear_exceptions(&mut self) {
        let mut fpscr = read_fpscr();
        fpscr.set_idc(false);
        fpscr.set_ixc(false);
        fpscr.set_ufc(false);
        fpscr.set_ofc(false);
        fpscr.set_dzc(false);
        fpscr.set_ioc(false);
        unsafe { write_fpscr(fpscr) };
    }
}
