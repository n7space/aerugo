#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub mr: MR,
    #[doc = "0x04 - Alarm Register"]
    pub ar: AR,
    #[doc = "0x08 - Value Register"]
    pub vr: VR,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
}
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "AR (rw) register accessor: an alias for `Reg<AR_SPEC>`"]
pub type AR = crate::Reg<ar::AR_SPEC>;
#[doc = "Alarm Register"]
pub mod ar;
#[doc = "VR (r) register accessor: an alias for `Reg<VR_SPEC>`"]
pub type VR = crate::Reg<vr::VR_SPEC>;
#[doc = "Value Register"]
pub mod vr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
