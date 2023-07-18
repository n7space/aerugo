#[doc = r"Register block"]
#[repr(C)]
pub struct PWM_CH_NUM {
    #[doc = "0x00 - PWM Channel Mode Register"]
    pub cmr: CMR,
    #[doc = "0x04 - PWM Channel Duty Cycle Register"]
    pub cdty: CDTY,
    #[doc = "0x08 - PWM Channel Duty Cycle Update Register"]
    pub cdtyupd: CDTYUPD,
    #[doc = "0x0c - PWM Channel Period Register"]
    pub cprd: CPRD,
    #[doc = "0x10 - PWM Channel Period Update Register"]
    pub cprdupd: CPRDUPD,
    #[doc = "0x14 - PWM Channel Counter Register"]
    pub ccnt: CCNT,
    #[doc = "0x18 - PWM Channel Dead Time Register"]
    pub dt: DT,
    #[doc = "0x1c - PWM Channel Dead Time Update Register"]
    pub dtupd: DTUPD,
}
#[doc = "CMR (rw) register accessor: an alias for `Reg<CMR_SPEC>`"]
pub type CMR = crate::Reg<cmr::CMR_SPEC>;
#[doc = "PWM Channel Mode Register"]
pub mod cmr;
#[doc = "CDTY (rw) register accessor: an alias for `Reg<CDTY_SPEC>`"]
pub type CDTY = crate::Reg<cdty::CDTY_SPEC>;
#[doc = "PWM Channel Duty Cycle Register"]
pub mod cdty;
#[doc = "CDTYUPD (w) register accessor: an alias for `Reg<CDTYUPD_SPEC>`"]
pub type CDTYUPD = crate::Reg<cdtyupd::CDTYUPD_SPEC>;
#[doc = "PWM Channel Duty Cycle Update Register"]
pub mod cdtyupd;
#[doc = "CPRD (rw) register accessor: an alias for `Reg<CPRD_SPEC>`"]
pub type CPRD = crate::Reg<cprd::CPRD_SPEC>;
#[doc = "PWM Channel Period Register"]
pub mod cprd;
#[doc = "CPRDUPD (w) register accessor: an alias for `Reg<CPRDUPD_SPEC>`"]
pub type CPRDUPD = crate::Reg<cprdupd::CPRDUPD_SPEC>;
#[doc = "PWM Channel Period Update Register"]
pub mod cprdupd;
#[doc = "CCNT (r) register accessor: an alias for `Reg<CCNT_SPEC>`"]
pub type CCNT = crate::Reg<ccnt::CCNT_SPEC>;
#[doc = "PWM Channel Counter Register"]
pub mod ccnt;
#[doc = "DT (rw) register accessor: an alias for `Reg<DT_SPEC>`"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "PWM Channel Dead Time Register"]
pub mod dt;
#[doc = "DTUPD (w) register accessor: an alias for `Reg<DTUPD_SPEC>`"]
pub type DTUPD = crate::Reg<dtupd::DTUPD_SPEC>;
#[doc = "PWM Channel Dead Time Update Register"]
pub mod dtupd;
