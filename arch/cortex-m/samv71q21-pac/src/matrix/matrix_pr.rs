#[doc = r"Register block"]
#[repr(C)]
pub struct MATRIX_PR {
    #[doc = "0x00 - Priority Register A for Slave 0"]
    pub pras: PRAS,
    #[doc = "0x04 - Priority Register B for Slave 0"]
    pub prbs: PRBS,
}
#[doc = "PRAS (rw) register accessor: an alias for `Reg<PRAS_SPEC>`"]
pub type PRAS = crate::Reg<pras::PRAS_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod pras;
#[doc = "PRBS (rw) register accessor: an alias for `Reg<PRBS_SPEC>`"]
pub type PRBS = crate::Reg<prbs::PRBS_SPEC>;
#[doc = "Priority Register B for Slave 0"]
pub mod prbs;
