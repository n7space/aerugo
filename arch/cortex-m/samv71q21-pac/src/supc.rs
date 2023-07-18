#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supply Controller Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Supply Controller Supply Monitor Mode Register"]
    pub smmr: SMMR,
    #[doc = "0x08 - Supply Controller Mode Register"]
    pub mr: MR,
    #[doc = "0x0c - Supply Controller Wake-up Mode Register"]
    pub wumr: WUMR,
    #[doc = "0x10 - Supply Controller Wake-up Inputs Register"]
    pub wuir: WUIR,
    #[doc = "0x14 - Supply Controller Status Register"]
    pub sr: SR,
    _reserved6: [u8; 0xbc],
    #[doc = "0xd4 - Write Protection Mode Register"]
    pub sysc_wpmr: SYSC_WPMR,
}
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Supply Controller Control Register"]
pub mod cr;
#[doc = "SMMR (rw) register accessor: an alias for `Reg<SMMR_SPEC>`"]
pub type SMMR = crate::Reg<smmr::SMMR_SPEC>;
#[doc = "Supply Controller Supply Monitor Mode Register"]
pub mod smmr;
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Supply Controller Mode Register"]
pub mod mr;
#[doc = "WUMR (rw) register accessor: an alias for `Reg<WUMR_SPEC>`"]
pub type WUMR = crate::Reg<wumr::WUMR_SPEC>;
#[doc = "Supply Controller Wake-up Mode Register"]
pub mod wumr;
#[doc = "WUIR (rw) register accessor: an alias for `Reg<WUIR_SPEC>`"]
pub type WUIR = crate::Reg<wuir::WUIR_SPEC>;
#[doc = "Supply Controller Wake-up Inputs Register"]
pub mod wuir;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Supply Controller Status Register"]
pub mod sr;
#[doc = "SYSC_WPMR (rw) register accessor: an alias for `Reg<SYSC_WPMR_SPEC>`"]
pub type SYSC_WPMR = crate::Reg<sysc_wpmr::SYSC_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod sysc_wpmr;
