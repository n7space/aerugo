#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xf0],
    #[doc = "0xf0 - Debug Halting Control and Status Register"]
    pub dhcsr: DHCSR,
    #[doc = "0xf4 - Debug Core Register Selector Register"]
    pub dcrsr: DCRSR,
    #[doc = "0xf8 - Debug Core Register Data Register"]
    pub dcrdr: DCRDR,
    #[doc = "0xfc - Debug Exception and Monitor Control Register"]
    pub demcr: DEMCR,
}
#[doc = "DHCSR (rw) register accessor: an alias for `Reg<DHCSR_SPEC>`"]
pub type DHCSR = crate::Reg<dhcsr::DHCSR_SPEC>;
#[doc = "Debug Halting Control and Status Register"]
pub mod dhcsr;
#[doc = "DCRSR (w) register accessor: an alias for `Reg<DCRSR_SPEC>`"]
pub type DCRSR = crate::Reg<dcrsr::DCRSR_SPEC>;
#[doc = "Debug Core Register Selector Register"]
pub mod dcrsr;
#[doc = "DCRDR (rw) register accessor: an alias for `Reg<DCRDR_SPEC>`"]
pub type DCRDR = crate::Reg<dcrdr::DCRDR_SPEC>;
#[doc = "Debug Core Register Data Register"]
pub mod dcrdr;
#[doc = "DEMCR (rw) register accessor: an alias for `Reg<DEMCR_SPEC>`"]
pub type DEMCR = crate::Reg<demcr::DEMCR_SPEC>;
#[doc = "Debug Exception and Monitor Control Register"]
pub mod demcr;
