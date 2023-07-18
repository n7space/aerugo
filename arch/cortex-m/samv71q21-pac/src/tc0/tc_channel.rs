#[doc = r"Register block"]
#[repr(C)]
pub struct TC_CHANNEL {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub ccr: CCR,
    _reserved_1_cmr: [u8; 0x04],
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    pub smmr: SMMR,
    #[doc = "0x0c - Register AB (channel = 0)"]
    pub rab: RAB,
    #[doc = "0x10 - Counter Value (channel = 0)"]
    pub cv: CV,
    #[doc = "0x14 - Register A (channel = 0)"]
    pub ra: RA,
    #[doc = "0x18 - Register B (channel = 0)"]
    pub rb: RB,
    #[doc = "0x1c - Register C (channel = 0)"]
    pub rc: RC,
    #[doc = "0x20 - Status Register (channel = 0)"]
    pub sr: SR,
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    pub ier: IER,
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    pub idr: IDR,
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    pub imr: IMR,
    #[doc = "0x30 - Extended Mode Register (channel = 0)"]
    pub emr: EMR,
}
impl TC_CHANNEL {
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn cmr_waveform_mode(&self) -> &CMR_WAVEFORM_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn cmr_capture_mode(&self) -> &CMR_CAPTURE_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "CCR (w) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Channel Control Register (channel = 0)"]
pub mod ccr;
#[doc = "CMR_CAPTURE_MODE (rw) register accessor: an alias for `Reg<CMR_CAPTURE_MODE_SPEC>`"]
pub type CMR_CAPTURE_MODE = crate::Reg<cmr_capture_mode::CMR_CAPTURE_MODE_SPEC>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr_capture_mode;
#[doc = "CMR_WAVEFORM_MODE (rw) register accessor: an alias for `Reg<CMR_WAVEFORM_MODE_SPEC>`"]
pub type CMR_WAVEFORM_MODE = crate::Reg<cmr_waveform_mode::CMR_WAVEFORM_MODE_SPEC>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr_waveform_mode;
#[doc = "SMMR (rw) register accessor: an alias for `Reg<SMMR_SPEC>`"]
pub type SMMR = crate::Reg<smmr::SMMR_SPEC>;
#[doc = "Stepper Motor Mode Register (channel = 0)"]
pub mod smmr;
#[doc = "RAB (r) register accessor: an alias for `Reg<RAB_SPEC>`"]
pub type RAB = crate::Reg<rab::RAB_SPEC>;
#[doc = "Register AB (channel = 0)"]
pub mod rab;
#[doc = "CV (r) register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Counter Value (channel = 0)"]
pub mod cv;
#[doc = "RA (rw) register accessor: an alias for `Reg<RA_SPEC>`"]
pub type RA = crate::Reg<ra::RA_SPEC>;
#[doc = "Register A (channel = 0)"]
pub mod ra;
#[doc = "RB (rw) register accessor: an alias for `Reg<RB_SPEC>`"]
pub type RB = crate::Reg<rb::RB_SPEC>;
#[doc = "Register B (channel = 0)"]
pub mod rb;
#[doc = "RC (rw) register accessor: an alias for `Reg<RC_SPEC>`"]
pub type RC = crate::Reg<rc::RC_SPEC>;
#[doc = "Register C (channel = 0)"]
pub mod rc;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register (channel = 0)"]
pub mod sr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod idr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod imr;
#[doc = "EMR (rw) register accessor: an alias for `Reg<EMR_SPEC>`"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "Extended Mode Register (channel = 0)"]
pub mod emr;
