#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Interrupt Controller Type Register"]
    pub ictr: ICTR,
    #[doc = "0x08 - Auxiliary Control Register"]
    pub actlr: ACTLR,
}
#[doc = "ICTR (r) register accessor: Interrupt Controller Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ictr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ictr`]
module"]
pub type ICTR = crate::Reg<ictr::ICTR_SPEC>;
#[doc = "Interrupt Controller Type Register"]
pub mod ictr;
#[doc = "ACTLR (rw) register accessor: Auxiliary Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`actlr`]
module"]
pub type ACTLR = crate::Reg<actlr::ACTLR_SPEC>;
#[doc = "Auxiliary Control Register"]
pub mod actlr;
