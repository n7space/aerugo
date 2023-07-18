#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AFEC Control Register"]
    pub cr: CR,
    #[doc = "0x04 - AFEC Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - AFEC Extended Mode Register"]
    pub emr: EMR,
    #[doc = "0x0c - AFEC Channel Sequence 1 Register"]
    pub seq1r: SEQ1R,
    #[doc = "0x10 - AFEC Channel Sequence 2 Register"]
    pub seq2r: SEQ2R,
    #[doc = "0x14 - AFEC Channel Enable Register"]
    pub cher: CHER,
    #[doc = "0x18 - AFEC Channel Disable Register"]
    pub chdr: CHDR,
    #[doc = "0x1c - AFEC Channel Status Register"]
    pub chsr: CHSR,
    #[doc = "0x20 - AFEC Last Converted Data Register"]
    pub lcdr: LCDR,
    #[doc = "0x24 - AFEC Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x28 - AFEC Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x2c - AFEC Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - AFEC Interrupt Status Register"]
    pub isr: ISR,
    _reserved13: [u8; 0x18],
    #[doc = "0x4c - AFEC Overrun Status Register"]
    pub over: OVER,
    #[doc = "0x50 - AFEC Compare Window Register"]
    pub cwr: CWR,
    #[doc = "0x54 - AFEC Channel Gain Register"]
    pub cgr: CGR,
    _reserved16: [u8; 0x08],
    #[doc = "0x60 - AFEC Channel Differential Register"]
    pub diffr: DIFFR,
    #[doc = "0x64 - AFEC Channel Selection Register"]
    pub cselr: CSELR,
    #[doc = "0x68 - AFEC Channel Data Register"]
    pub cdr: CDR,
    #[doc = "0x6c - AFEC Channel Offset Compensation Register"]
    pub cocr: COCR,
    #[doc = "0x70 - AFEC Temperature Sensor Mode Register"]
    pub tempmr: TEMPMR,
    #[doc = "0x74 - AFEC Temperature Compare Window Register"]
    pub tempcwr: TEMPCWR,
    _reserved22: [u8; 0x1c],
    #[doc = "0x94 - AFEC Analog Control Register"]
    pub acr: ACR,
    _reserved23: [u8; 0x08],
    #[doc = "0xa0 - AFEC Sample &amp; Hold Mode Register"]
    pub shmr: SHMR,
    _reserved24: [u8; 0x2c],
    #[doc = "0xd0 - AFEC Correction Select Register"]
    pub cosr: COSR,
    #[doc = "0xd4 - AFEC Correction Values Register"]
    pub cvr: CVR,
    #[doc = "0xd8 - AFEC Channel Error Correction Register"]
    pub cecr: CECR,
    _reserved27: [u8; 0x08],
    #[doc = "0xe4 - AFEC Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - AFEC Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "AFEC Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "AFEC Mode Register"]
pub mod mr;
#[doc = "EMR (rw) register accessor: an alias for `Reg<EMR_SPEC>`"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "AFEC Extended Mode Register"]
pub mod emr;
#[doc = "SEQ1R (rw) register accessor: an alias for `Reg<SEQ1R_SPEC>`"]
pub type SEQ1R = crate::Reg<seq1r::SEQ1R_SPEC>;
#[doc = "AFEC Channel Sequence 1 Register"]
pub mod seq1r;
#[doc = "SEQ2R (rw) register accessor: an alias for `Reg<SEQ2R_SPEC>`"]
pub type SEQ2R = crate::Reg<seq2r::SEQ2R_SPEC>;
#[doc = "AFEC Channel Sequence 2 Register"]
pub mod seq2r;
#[doc = "CHER (w) register accessor: an alias for `Reg<CHER_SPEC>`"]
pub type CHER = crate::Reg<cher::CHER_SPEC>;
#[doc = "AFEC Channel Enable Register"]
pub mod cher;
#[doc = "CHDR (w) register accessor: an alias for `Reg<CHDR_SPEC>`"]
pub type CHDR = crate::Reg<chdr::CHDR_SPEC>;
#[doc = "AFEC Channel Disable Register"]
pub mod chdr;
#[doc = "CHSR (r) register accessor: an alias for `Reg<CHSR_SPEC>`"]
pub type CHSR = crate::Reg<chsr::CHSR_SPEC>;
#[doc = "AFEC Channel Status Register"]
pub mod chsr;
#[doc = "LCDR (r) register accessor: an alias for `Reg<LCDR_SPEC>`"]
pub type LCDR = crate::Reg<lcdr::LCDR_SPEC>;
#[doc = "AFEC Last Converted Data Register"]
pub mod lcdr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "AFEC Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "AFEC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "AFEC Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "AFEC Interrupt Status Register"]
pub mod isr;
#[doc = "OVER (r) register accessor: an alias for `Reg<OVER_SPEC>`"]
pub type OVER = crate::Reg<over::OVER_SPEC>;
#[doc = "AFEC Overrun Status Register"]
pub mod over;
#[doc = "CWR (rw) register accessor: an alias for `Reg<CWR_SPEC>`"]
pub type CWR = crate::Reg<cwr::CWR_SPEC>;
#[doc = "AFEC Compare Window Register"]
pub mod cwr;
#[doc = "CGR (rw) register accessor: an alias for `Reg<CGR_SPEC>`"]
pub type CGR = crate::Reg<cgr::CGR_SPEC>;
#[doc = "AFEC Channel Gain Register"]
pub mod cgr;
#[doc = "DIFFR (rw) register accessor: an alias for `Reg<DIFFR_SPEC>`"]
pub type DIFFR = crate::Reg<diffr::DIFFR_SPEC>;
#[doc = "AFEC Channel Differential Register"]
pub mod diffr;
#[doc = "CSELR (rw) register accessor: an alias for `Reg<CSELR_SPEC>`"]
pub type CSELR = crate::Reg<cselr::CSELR_SPEC>;
#[doc = "AFEC Channel Selection Register"]
pub mod cselr;
#[doc = "CDR (r) register accessor: an alias for `Reg<CDR_SPEC>`"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "AFEC Channel Data Register"]
pub mod cdr;
#[doc = "COCR (rw) register accessor: an alias for `Reg<COCR_SPEC>`"]
pub type COCR = crate::Reg<cocr::COCR_SPEC>;
#[doc = "AFEC Channel Offset Compensation Register"]
pub mod cocr;
#[doc = "TEMPMR (rw) register accessor: an alias for `Reg<TEMPMR_SPEC>`"]
pub type TEMPMR = crate::Reg<tempmr::TEMPMR_SPEC>;
#[doc = "AFEC Temperature Sensor Mode Register"]
pub mod tempmr;
#[doc = "TEMPCWR (rw) register accessor: an alias for `Reg<TEMPCWR_SPEC>`"]
pub type TEMPCWR = crate::Reg<tempcwr::TEMPCWR_SPEC>;
#[doc = "AFEC Temperature Compare Window Register"]
pub mod tempcwr;
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "AFEC Analog Control Register"]
pub mod acr;
#[doc = "SHMR (rw) register accessor: an alias for `Reg<SHMR_SPEC>`"]
pub type SHMR = crate::Reg<shmr::SHMR_SPEC>;
#[doc = "AFEC Sample &amp; Hold Mode Register"]
pub mod shmr;
#[doc = "COSR (rw) register accessor: an alias for `Reg<COSR_SPEC>`"]
pub type COSR = crate::Reg<cosr::COSR_SPEC>;
#[doc = "AFEC Correction Select Register"]
pub mod cosr;
#[doc = "CVR (rw) register accessor: an alias for `Reg<CVR_SPEC>`"]
pub type CVR = crate::Reg<cvr::CVR_SPEC>;
#[doc = "AFEC Correction Values Register"]
pub mod cvr;
#[doc = "CECR (rw) register accessor: an alias for `Reg<CECR_SPEC>`"]
pub type CECR = crate::Reg<cecr::CECR_SPEC>;
#[doc = "AFEC Channel Error Correction Register"]
pub mod cecr;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "AFEC Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "AFEC Write Protection Status Register"]
pub mod wpsr;
