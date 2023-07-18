#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x34 - Master Configuration Register 0"]
    pub mcfg: [MCFG; 13],
    _reserved1: [u8; 0x0c],
    #[doc = "0x40..0x64 - Slave Configuration Register 0"]
    pub scfg: [SCFG; 9],
    _reserved2: [u8; 0x1c],
    #[doc = "0x80..0xc8 - Priority Register A for Slave 0"]
    pub matrix_pr: [MATRIX_PR; 9],
    _reserved3: [u8; 0x38],
    #[doc = "0x100 - Master Remap Control Register"]
    pub mrcr: MRCR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x110 - CAN0 Configuration Register"]
    pub ccfg_can0: CCFG_CAN0,
    #[doc = "0x114 - System I/O and CAN1 Configuration Register"]
    pub ccfg_sysio: CCFG_SYSIO,
    #[doc = "0x118 - Peripheral Clock Configuration Register"]
    pub ccfg_pccr: CCFG_PCCR,
    #[doc = "0x11c - Dynamic Clock Gating Register"]
    pub ccfg_dynckg: CCFG_DYNCKG,
    _reserved8: [u8; 0x04],
    #[doc = "0x124 - SMC NAND Flash Chip Select Configuration Register"]
    pub ccfg_smcnfcs: CCFG_SMCNFCS,
    _reserved9: [u8; 0xbc],
    #[doc = "0x1e4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = "MCFG (rw) register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Master Configuration Register 0"]
pub mod mcfg;
#[doc = "SCFG (rw) register accessor: an alias for `Reg<SCFG_SPEC>`"]
pub type SCFG = crate::Reg<scfg::SCFG_SPEC>;
#[doc = "Slave Configuration Register 0"]
pub mod scfg;
#[doc = "Priority Register A for Slave 0"]
pub use self::matrix_pr::MATRIX_PR;
#[doc = r"Cluster"]
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pr;
#[doc = "MRCR (rw) register accessor: an alias for `Reg<MRCR_SPEC>`"]
pub type MRCR = crate::Reg<mrcr::MRCR_SPEC>;
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "CCFG_CAN0 (rw) register accessor: an alias for `Reg<CCFG_CAN0_SPEC>`"]
pub type CCFG_CAN0 = crate::Reg<ccfg_can0::CCFG_CAN0_SPEC>;
#[doc = "CAN0 Configuration Register"]
pub mod ccfg_can0;
#[doc = "CCFG_SYSIO (rw) register accessor: an alias for `Reg<CCFG_SYSIO_SPEC>`"]
pub type CCFG_SYSIO = crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>;
#[doc = "System I/O and CAN1 Configuration Register"]
pub mod ccfg_sysio;
#[doc = "CCFG_PCCR (rw) register accessor: an alias for `Reg<CCFG_PCCR_SPEC>`"]
pub type CCFG_PCCR = crate::Reg<ccfg_pccr::CCFG_PCCR_SPEC>;
#[doc = "Peripheral Clock Configuration Register"]
pub mod ccfg_pccr;
#[doc = "CCFG_DYNCKG (rw) register accessor: an alias for `Reg<CCFG_DYNCKG_SPEC>`"]
pub type CCFG_DYNCKG = crate::Reg<ccfg_dynckg::CCFG_DYNCKG_SPEC>;
#[doc = "Dynamic Clock Gating Register"]
pub mod ccfg_dynckg;
#[doc = "CCFG_SMCNFCS (rw) register accessor: an alias for `Reg<CCFG_SMCNFCS_SPEC>`"]
pub type CCFG_SMCNFCS = crate::Reg<ccfg_smcnfcs::CCFG_SMCNFCS_SPEC>;
#[doc = "SMC NAND Flash Chip Select Configuration Register"]
pub mod ccfg_smcnfcs;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
