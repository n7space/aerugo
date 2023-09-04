#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip ID Register"]
    pub cidr: CIDR,
    #[doc = "0x04 - Chip ID Extension Register"]
    pub exid: EXID,
}
#[doc = "CIDR (r) register accessor: Chip ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cidr`]
module"]
pub type CIDR = crate::Reg<cidr::CIDR_SPEC>;
#[doc = "Chip ID Register"]
pub mod cidr;
#[doc = "EXID (r) register accessor: Chip ID Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exid`]
module"]
pub type EXID = crate::Reg<exid::EXID_SPEC>;
#[doc = "Chip ID Extension Register"]
pub mod exid;
