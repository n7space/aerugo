#![allow(dead_code)]
//! FPU registers. Cortex-M crate does not provide mapping and lacks some registers

/// Automatically preserve FP context on exception register offset.
pub(crate) const FPU_FPCCR_ASPEN_OFFSET: u32 = 31u32;
/// Automatically preserve FP context on exception register mask.
pub(crate) const FPU_FPCCR_ASPEN_MASK: u32 = 0x80000000u32;
/// Enable lazy context save of FP state register offset.
pub(crate) const FPU_FPCCR_LSPEN_OFFSET: u32 = 30u32;
/// Enable lazy context save of FP state register mask.
pub(crate) const FPU_FPCCR_LSPEN_MASK: u32 = 0x40000000u32;
/// Debug Monitor exception pending set register offset.
pub(crate) const FPU_FPCCR_MONRDY_OFFSET: u32 = 8u32;
/// Debug Monitor exception pending set register mask.
pub(crate) const FPU_FPCCR_MONRDY_MASK: u32 = 0x00000100u32;
/// Bus Fault exception pending set register offset.
pub(crate) const FPU_FPCCR_BFRDY_OFFSET: u32 = 6u32;
/// Bus Fault exception pending set register mask.
pub(crate) const FPU_FPCCR_BFRDY_MASK: u32 = 0x00000040u32;
/// Mem Manage exception pending set register offset.
pub(crate) const FPU_FPCCR_MMRDY_OFFSET: u32 = 5u32;
/// Mem Manage exception pending set register mask.
pub(crate) const FPU_FPCCR_MMRDY_MASK: u32 = 0x00000020u32;
/// Hard Fault exception pending set register offset.
pub(crate) const FPU_FPCCR_HFRDY_OFFSET: u32 = 4u32;
/// Hard Fault exception pending set register mask.
pub(crate) const FPU_FPCCR_HFRDY_MASK: u32 = 0x00000010u32;
/// Is thread mode set register offset.
pub(crate) const FPU_FPCCR_THREAD_OFFSET: u32 = 3u32;
/// Is thread mode set register mask.
pub(crate) const FPU_FPCCR_THREAD_MASK: u32 = 0x00000008u32;
/// Is user mode set register offset.
pub(crate) const FPU_FPCCR_USER_OFFSET: u32 = 1u32;
/// Is user mode set register mask.
pub(crate) const FPU_FPCCR_USER_MASK: u32 = 0x00000002u32;
/// Lazy FP state active register offset.
pub(crate) const FPU_FPCCR_LSPACT_OFFSET: u32 = 0u32;
/// Lazy FP state active register mask.
pub(crate) const FPU_FPCCR_LSPACT_MASK: u32 = 0x00000001u32;

/// FP register space allocated on an exception stack frame address register offset.
pub(crate) const FPU_FPCAR_ADDRESS_OFFSET: u32 = 3u32;
/// FP register space allocated on an exception stack frame address register mask.
pub(crate) const FPU_FPCAR_ADDRESS_MASK: u32 = 0xFFFFFFF8u32;

/// Alternative half-precision control bit default value register offset.
pub(crate) const FPU_FPDSCR_AHP_OFFSET: u32 = 26u32;
/// Alternative half-precision control bit default value register mask.
pub(crate) const FPU_FPDSCR_AHP_MASK: u32 = 0x04000000u32;
/// NaN mode control bit default value register offset.
pub(crate) const FPU_FPDSCR_DN_OFFSET: u32 = 25u32;
/// NaN mode control bit default value register mask.
pub(crate) const FPU_FPDSCR_DN_MASK: u32 = 0x02000000u32;
/// Flush-to-zero mode control bit default value register offset.
pub(crate) const FPU_FPDSCR_FZ_OFFSET: u32 = 24u32;
/// Flush-to-zero mode control bit default value register mask.
pub(crate) const FPU_FPDSCR_FZ_MASK: u32 = 0x01000000u32;
/// Rounding mode control field default value register offset.
pub(crate) const FPU_FPDSCR_RMODE_OFFSET: u32 = 22u32;
/// Rounding mode control field default value register mask.
pub(crate) const FPU_FPDSCR_RMODE_MASK: u32 = 0x00C00000u32;

/// Feature not supported indicator.
pub(crate) const FPU_FEATURE_NOT_SUPPORTED: u32 = 0x00u32;
/// Feature supported indicator.
pub(crate) const FPU_FEATURE_SUPPORTED: u32 = 0xFFu32;

/// FP rounding modes register offset.
pub(crate) const FPU_MVFR0_FP_ROUNDING_MODES_OFFSET: u32 = 28u32;
/// FP rounding modes register mask.
pub(crate) const FPU_MVFR0_FP_ROUNDING_MODES_MASK: u32 = 0xF0000000u32;
/// Short vectors feature register offset.
pub(crate) const FPU_MVFR0_SHORT_VECTORS_OFFSET: u32 = 24u32;
/// Short vectors feature register mask.
pub(crate) const FPU_MVFR0_SHORT_VECTORS_MASK: u32 = 0x0F000000u32;
/// Square root feature register offset.
pub(crate) const FPU_MVFR0_SQUARE_ROOT_OFFSET: u32 = 20u32;
/// Square root feature register mask.
pub(crate) const FPU_MVFR0_SQUARE_ROOT_MASK: u32 = 0x00F00000u32;
/// Divide feature register offset.
pub(crate) const FPU_MVFR0_DIVIDE_OFFSET: u32 = 16u32;
/// Divide feature register mask.
pub(crate) const FPU_MVFR0_DIVIDE_MASK: u32 = 0x000F0000u32;
/// FP exception trapping feature register offset.
pub(crate) const FPU_MVFR0_FP_EXCEPTION_TRAPPING_OFFSET: u32 = 12u32;
/// FP exception trapping feature register mask.
pub(crate) const FPU_MVFR0_FP_EXCEPTION_TRAPPING_MASK: u32 = 0x0000F000u32;
/// Double-precision feature register offset.
pub(crate) const FPU_MVFR0_DOUBLE_PRECISION_OFFSET: u32 = 8u32;
/// Double-precision feature register mask.
pub(crate) const FPU_MVFR0_DOUBLE_PRECISION_MASK: u32 = 0x00000F00u32;
/// Single-precision feature register offset.
pub(crate) const FPU_MVFR0_SINGLE_PRECISION_OFFSET: u32 = 4u32;
/// Single-precision feature register mask.
pub(crate) const FPU_MVFR0_SINGLE_PRECISION_MASK: u32 = 0x000000F0u32;
/// FP register bank size register offset.
pub(crate) const FPU_MVFR0_A_SIMD_OFFSET: u32 = 0u32;
/// FP register bank size register mask.
pub(crate) const FPU_MVFR0_A_SIMD_MASK: u32 = 0x0000000Fu32;

/// FP fused MAC feature register offset.
pub(crate) const FPU_MVFR1_FP_FUSED_MAC_OFFSET: u32 = 28u32;
/// FP fused MAC feature register mask.
pub(crate) const FPU_MVFR1_FP_FUSED_MAC_MASK: u32 = 0xF0000000u32;
/// FP half-precision and double-precision register offset.
pub(crate) const FPU_MVFR1_FP_HPFP_OFFSET: u32 = 24u32;
/// FP half-precision and double-precision register mask.
pub(crate) const FPU_MVFR1_FP_HPFP_MASK: u32 = 0x0F000000u32;
/// Half-precision <-> single-precision conversion supported.
pub(crate) const FPU_MVFR1_FP_HPFP_HPSP_SUPPORTED: u32 = 0x01u32;
/// Half-precision <-> single-precision <-> double-precision conversion supported.
pub(crate) const FPU_MVFR1_FP_HPFP_HPDP_SUPPORTED: u32 = 0x02u32;
/// Default NaN mode feature register offset.
pub(crate) const FPU_MVFR1_D_NAN_MODE_OFFSET: u32 = 4u32;
/// Default NaN mode feature register mask.
pub(crate) const FPU_MVFR1_D_NAN_MODE_MASK: u32 = 0x000000F0u32;
/// Flush-to-zero mode feature register offset.
pub(crate) const FPU_MVFR1_FTZ_MODE_OFFSET: u32 = 0u32;
/// Flush-to-zero mode feature register mask.
pub(crate) const FPU_MVFR1_FTZ_MODE_MASK: u32 = 0x0000000Fu32;

/// Miscellaneous features register offset.
pub(crate) const FPU_MVFR2_VFP_MISC_OFFSET: u32 = 4u32;
/// Miscellaneous features register mask.
pub(crate) const FPU_MVFR2_VFP_MISC_MASK: u32 = 0x000000F0u32;

/// Coprocessor 10 register offset.
pub(crate) const FPU_CPACR_CP10_OFFSET: u32 = 20u32;
/// Coprocessor 10 register mask.
pub(crate) const FPU_CPACR_CP10_MASK: u32 = 0x00300000u32;
/// Coprocessor 11 register offset.
pub(crate) const FPU_CPACR_CP11_OFFSET: u32 = 22u32;
/// Coprocessor 11 register mask.
pub(crate) const FPU_CPACR_CP11_MASK: u32 = 0x00C00000u32;

/// Negative condition code flag register offset.
pub(crate) const FPU_FPSCR_N_OFFSET: u32 = 31u32;
/// Negative condition code flag register mask.
pub(crate) const FPU_FPSCR_N_MASK: u32 = 0x80000000u32;
/// Zero condition code flag register offset.
pub(crate) const FPU_FPSCR_Z_OFFSET: u32 = 30u32;
/// Zero condition code flag register mask.
pub(crate) const FPU_FPSCR_Z_MASK: u32 = 0x40000000u32;
/// Carry condition code flag register offset.
pub(crate) const FPU_FPSCR_C_OFFSET: u32 = 29u32;
/// Carry condition code flag register mask.
pub(crate) const FPU_FPSCR_C_MASK: u32 = 0x20000000u32;
/// Overflow condition code flag register offset.
pub(crate) const FPU_FPSCR_V_OFFSET: u32 = 28u32;
/// Overflow condition code flag register mask.
pub(crate) const FPU_FPSCR_V_MASK: u32 = 0x10000000u32;
/// Alternative half-precision control bit register offset.
pub(crate) const FPU_FPSCR_AHP_OFFSET: u32 = 26u32;
/// Alternative half-precision control bit register mask.
pub(crate) const FPU_FPSCR_AHP_MASK: u32 = 0x04000000u32;
/// Default NaN mode control bit register offset.
pub(crate) const FPU_FPSCR_DN_OFFSET: u32 = 25u32;
/// Default NaN mode control bit register mask.
pub(crate) const FPU_FPSCR_DN_MASK: u32 = 0x02000000u32;
/// Flush-to-zero mode control bit register offset.
pub(crate) const FPU_FPSCR_FZ_OFFSET: u32 = 24u32;
/// Flush-to-zero mode control bit register mask.
pub(crate) const FPU_FPSCR_FZ_MASK: u32 = 0x01000000u32;
/// Rounding model control field register offset.
pub(crate) const FPU_FPSCR_RMODE_OFFSET: u32 = 22u32;
/// Rounding model control field register mask.
pub(crate) const FPU_FPSCR_RMODE_MASK: u32 = 0x00C00000u32;
/// Input Denormal cumulative exception bit register offset.
pub(crate) const FPU_FPSCR_IDC_OFFSET: u32 = 7u32;
/// Input Denormal cumulative exception bit register mask.
pub(crate) const FPU_FPSCR_IDC_MASK: u32 = 0x00000080u32;
/// Inexact cumulative exception bit register offset.
pub(crate) const FPU_FPSCR_IXC_OFFSET: u32 = 4u32;
/// Inexact cumulative exception bit register mask.
pub(crate) const FPU_FPSCR_IXC_MASK: u32 = 0x00000010u32;
/// Underflow cumulative exception bit register offset.
pub(crate) const FPU_FPSCR_UFC_OFFSET: u32 = 3u32;
/// Underflow cumulative exception bit register mask.
pub(crate) const FPU_FPSCR_UFC_MASK: u32 = 0x00000008u32;
/// Overflow cumulative exception bit register offset.
pub(crate) const FPU_FPSCR_OFC_OFFSET: u32 = 2u32;
/// Overflow cumulative exception bit register mask.
pub(crate) const FPU_FPSCR_OFC_MASK: u32 = 0x00000004u32;
/// Division by Zero cumulative exception bit register offset.
pub(crate) const FPU_FPSCR_DZC_OFFSET: u32 = 1u32;
/// Division by Zero cumulative exception bit register mask.
pub(crate) const FPU_FPSCR_DZC_MASK: u32 = 0x00000002u32;
/// Invalid Operation cumulative exception bit register offset.
pub(crate) const FPU_FPSCR_IOC_OFFSET: u32 = 0u32;
/// Invalid Operation cumulative exception bit register mask.
pub(crate) const FPU_FPSCR_IOC_MASK: u32 = 0x00000001u32;
