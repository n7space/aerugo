#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MediaLB Control 0 Register"]
    pub mlbc0: MLBC0,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - MediaLB Channel Status 0 Register"]
    pub ms0: MS0,
    _reserved2: [u8; 0x04],
    #[doc = "0x14 - MediaLB Channel Status1 Register"]
    pub ms1: MS1,
    _reserved3: [u8; 0x08],
    #[doc = "0x20 - MediaLB System Status Register"]
    pub mss: MSS,
    #[doc = "0x24 - MediaLB System Data Register"]
    pub msd: MSD,
    _reserved5: [u8; 0x04],
    #[doc = "0x2c - MediaLB Interrupt Enable Register"]
    pub mien: MIEN,
    _reserved6: [u8; 0x0c],
    #[doc = "0x3c - MediaLB Control 1 Register"]
    pub mlbc1: MLBC1,
    _reserved7: [u8; 0x40],
    #[doc = "0x80 - HBI Control Register"]
    pub hctl: HCTL,
    _reserved8: [u8; 0x04],
    #[doc = "0x88..0x90 - HBI Channel Mask 0 Register 0"]
    pub hcmr: [HCMR; 2],
    #[doc = "0x90..0x98 - HBI Channel Error 0 Register 0"]
    pub hcer: [HCER; 2],
    #[doc = "0x98..0xa0 - HBI Channel Busy 0 Register 0"]
    pub hcbr: [HCBR; 2],
    _reserved11: [u8; 0x20],
    #[doc = "0xc0..0xd0 - MIF Data 0 Register 0"]
    pub mdat: [MDAT; 4],
    #[doc = "0xd0..0xe0 - MIF Data Write Enable 0 Register 0"]
    pub mdwe: [MDWE; 4],
    #[doc = "0xe0 - MIF Control Register"]
    pub mctl: MCTL,
    #[doc = "0xe4 - MIF Address Register"]
    pub madr: MADR,
    _reserved15: [u8; 0x02d8],
    #[doc = "0x3c0 - AHB Control Register"]
    pub actl: ACTL,
    _reserved16: [u8; 0x0c],
    #[doc = "0x3d0..0x3d8 - AHB Channel Status 0 Register 0"]
    pub acsr: [ACSR; 2],
    #[doc = "0x3d8..0x3e0 - AHB Channel Mask 0 Register 0"]
    pub acmr: [ACMR; 2],
}
#[doc = "MLBC0 (rw) register accessor: an alias for `Reg<MLBC0_SPEC>`"]
pub type MLBC0 = crate::Reg<mlbc0::MLBC0_SPEC>;
#[doc = "MediaLB Control 0 Register"]
pub mod mlbc0;
#[doc = "MS0 (rw) register accessor: an alias for `Reg<MS0_SPEC>`"]
pub type MS0 = crate::Reg<ms0::MS0_SPEC>;
#[doc = "MediaLB Channel Status 0 Register"]
pub mod ms0;
#[doc = "MS1 (rw) register accessor: an alias for `Reg<MS1_SPEC>`"]
pub type MS1 = crate::Reg<ms1::MS1_SPEC>;
#[doc = "MediaLB Channel Status1 Register"]
pub mod ms1;
#[doc = "MSS (rw) register accessor: an alias for `Reg<MSS_SPEC>`"]
pub type MSS = crate::Reg<mss::MSS_SPEC>;
#[doc = "MediaLB System Status Register"]
pub mod mss;
#[doc = "MSD (r) register accessor: an alias for `Reg<MSD_SPEC>`"]
pub type MSD = crate::Reg<msd::MSD_SPEC>;
#[doc = "MediaLB System Data Register"]
pub mod msd;
#[doc = "MIEN (rw) register accessor: an alias for `Reg<MIEN_SPEC>`"]
pub type MIEN = crate::Reg<mien::MIEN_SPEC>;
#[doc = "MediaLB Interrupt Enable Register"]
pub mod mien;
#[doc = "MLBC1 (rw) register accessor: an alias for `Reg<MLBC1_SPEC>`"]
pub type MLBC1 = crate::Reg<mlbc1::MLBC1_SPEC>;
#[doc = "MediaLB Control 1 Register"]
pub mod mlbc1;
#[doc = "HCTL (rw) register accessor: an alias for `Reg<HCTL_SPEC>`"]
pub type HCTL = crate::Reg<hctl::HCTL_SPEC>;
#[doc = "HBI Control Register"]
pub mod hctl;
#[doc = "HCMR (rw) register accessor: an alias for `Reg<HCMR_SPEC>`"]
pub type HCMR = crate::Reg<hcmr::HCMR_SPEC>;
#[doc = "HBI Channel Mask 0 Register 0"]
pub mod hcmr;
#[doc = "HCER (r) register accessor: an alias for `Reg<HCER_SPEC>`"]
pub type HCER = crate::Reg<hcer::HCER_SPEC>;
#[doc = "HBI Channel Error 0 Register 0"]
pub mod hcer;
#[doc = "HCBR (r) register accessor: an alias for `Reg<HCBR_SPEC>`"]
pub type HCBR = crate::Reg<hcbr::HCBR_SPEC>;
#[doc = "HBI Channel Busy 0 Register 0"]
pub mod hcbr;
#[doc = "MDAT (rw) register accessor: an alias for `Reg<MDAT_SPEC>`"]
pub type MDAT = crate::Reg<mdat::MDAT_SPEC>;
#[doc = "MIF Data 0 Register 0"]
pub mod mdat;
#[doc = "MDWE (rw) register accessor: an alias for `Reg<MDWE_SPEC>`"]
pub type MDWE = crate::Reg<mdwe::MDWE_SPEC>;
#[doc = "MIF Data Write Enable 0 Register 0"]
pub mod mdwe;
#[doc = "MCTL (rw) register accessor: an alias for `Reg<MCTL_SPEC>`"]
pub type MCTL = crate::Reg<mctl::MCTL_SPEC>;
#[doc = "MIF Control Register"]
pub mod mctl;
#[doc = "MADR (rw) register accessor: an alias for `Reg<MADR_SPEC>`"]
pub type MADR = crate::Reg<madr::MADR_SPEC>;
#[doc = "MIF Address Register"]
pub mod madr;
#[doc = "ACTL (rw) register accessor: an alias for `Reg<ACTL_SPEC>`"]
pub type ACTL = crate::Reg<actl::ACTL_SPEC>;
#[doc = "AHB Control Register"]
pub mod actl;
#[doc = "ACSR (rw) register accessor: an alias for `Reg<ACSR_SPEC>`"]
pub type ACSR = crate::Reg<acsr::ACSR_SPEC>;
#[doc = "AHB Channel Status 0 Register 0"]
pub mod acsr;
#[doc = "ACMR (rw) register accessor: an alias for `Reg<ACMR_SPEC>`"]
pub type ACMR = crate::Reg<acmr::ACMR_SPEC>;
#[doc = "AHB Channel Mask 0 Register 0"]
pub mod acmr;
