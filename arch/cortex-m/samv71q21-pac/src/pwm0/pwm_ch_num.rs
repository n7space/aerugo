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
#[doc = "CMR (rw) register accessor: PWM Channel Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmr`]
module"]
pub type CMR = crate::Reg<cmr::CMR_SPEC>;
#[doc = "PWM Channel Mode Register"]
pub mod cmr;
#[doc = "CDTY (rw) register accessor: PWM Channel Duty Cycle Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cdty`]
module"]
pub type CDTY = crate::Reg<cdty::CDTY_SPEC>;
#[doc = "PWM Channel Duty Cycle Register"]
pub mod cdty;
#[doc = "CDTYUPD (w) register accessor: PWM Channel Duty Cycle Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdtyupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cdtyupd`]
module"]
pub type CDTYUPD = crate::Reg<cdtyupd::CDTYUPD_SPEC>;
#[doc = "PWM Channel Duty Cycle Update Register"]
pub mod cdtyupd;
#[doc = "CPRD (rw) register accessor: PWM Channel Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cprd`]
module"]
pub type CPRD = crate::Reg<cprd::CPRD_SPEC>;
#[doc = "PWM Channel Period Register"]
pub mod cprd;
#[doc = "CPRDUPD (w) register accessor: PWM Channel Period Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprdupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cprdupd`]
module"]
pub type CPRDUPD = crate::Reg<cprdupd::CPRDUPD_SPEC>;
#[doc = "PWM Channel Period Update Register"]
pub mod cprdupd;
#[doc = "CCNT (r) register accessor: PWM Channel Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccnt`]
module"]
pub type CCNT = crate::Reg<ccnt::CCNT_SPEC>;
#[doc = "PWM Channel Counter Register"]
pub mod ccnt;
#[doc = "DT (rw) register accessor: PWM Channel Dead Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt`]
module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "PWM Channel Dead Time Register"]
pub mod dt;
#[doc = "DTUPD (w) register accessor: PWM Channel Dead Time Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtupd`]
module"]
pub type DTUPD = crate::Reg<dtupd::DTUPD_SPEC>;
#[doc = "PWM Channel Dead Time Update Register"]
pub mod dtupd;
