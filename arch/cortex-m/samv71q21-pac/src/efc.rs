#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEFC Flash Mode Register"]
    pub eefc_fmr: EEFC_FMR,
    #[doc = "0x04 - EEFC Flash Command Register"]
    pub eefc_fcr: EEFC_FCR,
    #[doc = "0x08 - EEFC Flash Status Register"]
    pub eefc_fsr: EEFC_FSR,
    #[doc = "0x0c - EEFC Flash Result Register"]
    pub eefc_frr: EEFC_FRR,
    _reserved4: [u8; 0xd4],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub eefc_wpmr: EEFC_WPMR,
}
#[doc = "EEFC_FMR (rw) register accessor: EEFC Flash Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefc_fmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefc_fmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefc_fmr`]
module"]
pub type EEFC_FMR = crate::Reg<eefc_fmr::EEFC_FMR_SPEC>;
#[doc = "EEFC Flash Mode Register"]
pub mod eefc_fmr;
#[doc = "EEFC_FCR (w) register accessor: EEFC Flash Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefc_fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefc_fcr`]
module"]
pub type EEFC_FCR = crate::Reg<eefc_fcr::EEFC_FCR_SPEC>;
#[doc = "EEFC Flash Command Register"]
pub mod eefc_fcr;
#[doc = "EEFC_FSR (r) register accessor: EEFC Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefc_fsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefc_fsr`]
module"]
pub type EEFC_FSR = crate::Reg<eefc_fsr::EEFC_FSR_SPEC>;
#[doc = "EEFC Flash Status Register"]
pub mod eefc_fsr;
#[doc = "EEFC_FRR (r) register accessor: EEFC Flash Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefc_frr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefc_frr`]
module"]
pub type EEFC_FRR = crate::Reg<eefc_frr::EEFC_FRR_SPEC>;
#[doc = "EEFC Flash Result Register"]
pub mod eefc_frr;
#[doc = "EEFC_WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefc_wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefc_wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`eefc_wpmr`]
module"]
pub type EEFC_WPMR = crate::Reg<eefc_wpmr::EEFC_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod eefc_wpmr;
