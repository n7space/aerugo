#[doc = r"Register block"]
#[repr(C)]
pub struct PWM_CMP {
    #[doc = "0x00 - PWM Comparison 0 Value Register"]
    pub cmpv: CMPV,
    #[doc = "0x04 - PWM Comparison 0 Value Update Register"]
    pub cmpvupd: CMPVUPD,
    #[doc = "0x08 - PWM Comparison 0 Mode Register"]
    pub cmpm: CMPM,
    #[doc = "0x0c - PWM Comparison 0 Mode Update Register"]
    pub cmpmupd: CMPMUPD,
}
#[doc = "CMPV (rw) register accessor: an alias for `Reg<CMPV_SPEC>`"]
pub type CMPV = crate::Reg<cmpv::CMPV_SPEC>;
#[doc = "PWM Comparison 0 Value Register"]
pub mod cmpv;
#[doc = "CMPVUPD (w) register accessor: an alias for `Reg<CMPVUPD_SPEC>`"]
pub type CMPVUPD = crate::Reg<cmpvupd::CMPVUPD_SPEC>;
#[doc = "PWM Comparison 0 Value Update Register"]
pub mod cmpvupd;
#[doc = "CMPM (rw) register accessor: an alias for `Reg<CMPM_SPEC>`"]
pub type CMPM = crate::Reg<cmpm::CMPM_SPEC>;
#[doc = "PWM Comparison 0 Mode Register"]
pub mod cmpm;
#[doc = "CMPMUPD (w) register accessor: an alias for `Reg<CMPMUPD_SPEC>`"]
pub type CMPMUPD = crate::Reg<cmpmupd::CMPMUPD_SPEC>;
#[doc = "PWM Comparison 0 Mode Update Register"]
pub mod cmpmupd;
