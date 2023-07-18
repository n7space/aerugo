#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Master Mode Register"]
    pub mmr: MMR,
    #[doc = "0x08 - Slave Mode Register"]
    pub smr: SMR,
    #[doc = "0x0c - Internal Address Register"]
    pub iadr: IADR,
    #[doc = "0x10 - Clock Waveform Generator Register"]
    pub cwgr: CWGR,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - Status Register"]
    pub sr: SR,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x34 - Transmit Holding Register"]
    pub thr: THR,
    #[doc = "0x38 - SMBus Timing Register"]
    pub smbtr: SMBTR,
    _reserved12: [u8; 0x08],
    #[doc = "0x44 - Filter Register"]
    pub filtr: FILTR,
    _reserved13: [u8; 0x04],
    #[doc = "0x4c - SleepWalking Matching Register"]
    pub swmr: SWMR,
    _reserved14: [u8; 0x94],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MMR (rw) register accessor: an alias for `Reg<MMR_SPEC>`"]
pub type MMR = crate::Reg<mmr::MMR_SPEC>;
#[doc = "Master Mode Register"]
pub mod mmr;
#[doc = "SMR (rw) register accessor: an alias for `Reg<SMR_SPEC>`"]
pub type SMR = crate::Reg<smr::SMR_SPEC>;
#[doc = "Slave Mode Register"]
pub mod smr;
#[doc = "IADR (rw) register accessor: an alias for `Reg<IADR_SPEC>`"]
pub type IADR = crate::Reg<iadr::IADR_SPEC>;
#[doc = "Internal Address Register"]
pub mod iadr;
#[doc = "CWGR (rw) register accessor: an alias for `Reg<CWGR_SPEC>`"]
pub type CWGR = crate::Reg<cwgr::CWGR_SPEC>;
#[doc = "Clock Waveform Generator Register"]
pub mod cwgr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
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
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "THR (w) register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "SMBTR (rw) register accessor: an alias for `Reg<SMBTR_SPEC>`"]
pub type SMBTR = crate::Reg<smbtr::SMBTR_SPEC>;
#[doc = "SMBus Timing Register"]
pub mod smbtr;
#[doc = "FILTR (rw) register accessor: an alias for `Reg<FILTR_SPEC>`"]
pub type FILTR = crate::Reg<filtr::FILTR_SPEC>;
#[doc = "Filter Register"]
pub mod filtr;
#[doc = "SWMR (rw) register accessor: an alias for `Reg<SWMR_SPEC>`"]
pub type SWMR = crate::Reg<swmr::SWMR_SPEC>;
#[doc = "SleepWalking Matching Register"]
pub mod swmr;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
