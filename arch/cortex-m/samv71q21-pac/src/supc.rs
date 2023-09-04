#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supply Controller Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Supply Controller Supply Monitor Mode Register"]
    pub smmr: SMMR,
    #[doc = "0x08 - Supply Controller Mode Register"]
    pub mr: MR,
    #[doc = "0x0c - Supply Controller Wake-up Mode Register"]
    pub wumr: WUMR,
    #[doc = "0x10 - Supply Controller Wake-up Inputs Register"]
    pub wuir: WUIR,
    #[doc = "0x14 - Supply Controller Status Register"]
    pub sr: SR,
    _reserved6: [u8; 0xbc],
    #[doc = "0xd4 - Write Protection Mode Register"]
    pub sysc_wpmr: SYSC_WPMR,
}
#[doc = "CR (w) register accessor: Supply Controller Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Supply Controller Control Register"]
pub mod cr;
#[doc = "SMMR (rw) register accessor: Supply Controller Supply Monitor Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`smmr`]
module"]
pub type SMMR = crate::Reg<smmr::SMMR_SPEC>;
#[doc = "Supply Controller Supply Monitor Mode Register"]
pub mod smmr;
#[doc = "MR (rw) register accessor: Supply Controller Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Supply Controller Mode Register"]
pub mod mr;
#[doc = "WUMR (rw) register accessor: Supply Controller Wake-up Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wumr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wumr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wumr`]
module"]
pub type WUMR = crate::Reg<wumr::WUMR_SPEC>;
#[doc = "Supply Controller Wake-up Mode Register"]
pub mod wumr;
#[doc = "WUIR (rw) register accessor: Supply Controller Wake-up Inputs Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wuir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wuir`]
module"]
pub type WUIR = crate::Reg<wuir::WUIR_SPEC>;
#[doc = "Supply Controller Wake-up Inputs Register"]
pub mod wuir;
#[doc = "SR (r) register accessor: Supply Controller Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Supply Controller Status Register"]
pub mod sr;
#[doc = "SYSC_WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysc_wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysc_wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sysc_wpmr`]
module"]
pub type SYSC_WPMR = crate::Reg<sysc_wpmr::SYSC_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod sysc_wpmr;
