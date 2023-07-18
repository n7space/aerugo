#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Status Register"]
    pub sr: SR,
    #[doc = "0x0c - Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x10 - Status Set Register"]
    pub ssr: SSR,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x20 - Receiver Holding Register"]
    pub rhr: RHR,
    #[doc = "0x24 - Transmitter Holding Register"]
    pub thr: THR,
}
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Status Clear Register"]
pub mod scr;
#[doc = "SSR (w) register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "Status Set Register"]
pub mod ssr;
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
#[doc = "RHR (r) register accessor: an alias for `Reg<RHR_SPEC>`"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Receiver Holding Register"]
pub mod rhr;
#[doc = "THR (w) register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmitter Holding Register"]
pub mod thr;
