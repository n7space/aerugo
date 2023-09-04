#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - OHCI Interrupt Configuration Register"]
    pub ohciicr: OHCIICR,
    _reserved1: [u8; 0x1c],
    #[doc = "0x30 - UTMI Clock Trimming Register"]
    pub cktrim: CKTRIM,
}
#[doc = "OHCIICR (rw) register accessor: OHCI Interrupt Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ohciicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ohciicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ohciicr`]
module"]
pub type OHCIICR = crate::Reg<ohciicr::OHCIICR_SPEC>;
#[doc = "OHCI Interrupt Configuration Register"]
pub mod ohciicr;
#[doc = "CKTRIM (rw) register accessor: UTMI Clock Trimming Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cktrim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cktrim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cktrim`]
module"]
pub type CKTRIM = crate::Reg<cktrim::CKTRIM_SPEC>;
#[doc = "UTMI Clock Trimming Register"]
pub mod cktrim;
