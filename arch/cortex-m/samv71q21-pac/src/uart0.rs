#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x0c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x10 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x14 - Status Register"]
    pub sr: SR,
    #[doc = "0x18 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x1c - Transmit Holding Register"]
    pub thr: THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub brgr: BRGR,
    #[doc = "0x24 - Comparison Register"]
    pub cmpr: CMPR,
    _reserved10: [u8; 0xbc],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
}
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "RHR (r) register accessor: an alias for `Reg<RHR_SPEC>`"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "THR (w) register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "BRGR (rw) register accessor: an alias for `Reg<BRGR_SPEC>`"]
pub type BRGR = crate::Reg<brgr::BRGR_SPEC>;
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "CMPR (rw) register accessor: an alias for `Reg<CMPR_SPEC>`"]
pub type CMPR = crate::Reg<cmpr::CMPR_SPEC>;
#[doc = "Comparison Register"]
pub mod cmpr;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
