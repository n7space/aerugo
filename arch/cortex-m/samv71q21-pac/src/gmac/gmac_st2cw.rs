#[doc = r"Register block"]
#[repr(C)]
pub struct GMAC_ST2CW {
    #[doc = "0x00 - Screening Type 2 Compare Word 0 Register"]
    pub st2cw0: ST2CW0,
    #[doc = "0x04 - Screening Type 2 Compare Word 1 Register"]
    pub st2cw1: ST2CW1,
}
#[doc = "ST2CW0 (rw) register accessor: an alias for `Reg<ST2CW0_SPEC>`"]
pub type ST2CW0 = crate::Reg<st2cw0::ST2CW0_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register"]
pub mod st2cw0;
#[doc = "ST2CW1 (rw) register accessor: an alias for `Reg<ST2CW1_SPEC>`"]
pub type ST2CW1 = crate::Reg<st2cw1::ST2CW1_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register"]
pub mod st2cw1;
