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
#[doc = "CMPV (rw) register accessor: PWM Comparison 0 Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmpv`]
module"]
pub type CMPV = crate::Reg<cmpv::CMPV_SPEC>;
#[doc = "PWM Comparison 0 Value Register"]
pub mod cmpv;
#[doc = "CMPVUPD (w) register accessor: PWM Comparison 0 Value Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmpvupd`]
module"]
pub type CMPVUPD = crate::Reg<cmpvupd::CMPVUPD_SPEC>;
#[doc = "PWM Comparison 0 Value Update Register"]
pub mod cmpvupd;
#[doc = "CMPM (rw) register accessor: PWM Comparison 0 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmpm`]
module"]
pub type CMPM = crate::Reg<cmpm::CMPM_SPEC>;
#[doc = "PWM Comparison 0 Mode Register"]
pub mod cmpm;
#[doc = "CMPMUPD (w) register accessor: PWM Comparison 0 Mode Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpmupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmpmupd`]
module"]
pub type CMPMUPD = crate::Reg<cmpmupd::CMPMUPD_SPEC>;
#[doc = "PWM Comparison 0 Mode Update Register"]
pub mod cmpmupd;
