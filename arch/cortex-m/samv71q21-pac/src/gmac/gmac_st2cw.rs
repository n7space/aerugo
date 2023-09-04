#[doc = r"Register block"]
#[repr(C)]
pub struct GMAC_ST2CW {
    #[doc = "0x00 - Screening Type 2 Compare Word 0 Register"]
    pub st2cw0: ST2CW0,
    #[doc = "0x04 - Screening Type 2 Compare Word 1 Register"]
    pub st2cw1: ST2CW1,
}
#[doc = "ST2CW0 (rw) register accessor: Screening Type 2 Compare Word 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cw0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cw0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cw0`]
module"]
pub type ST2CW0 = crate::Reg<st2cw0::ST2CW0_SPEC>;
#[doc = "Screening Type 2 Compare Word 0 Register"]
pub mod st2cw0;
#[doc = "ST2CW1 (rw) register accessor: Screening Type 2 Compare Word 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cw1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cw1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`st2cw1`]
module"]
pub type ST2CW1 = crate::Reg<st2cw1::ST2CW1_SPEC>;
#[doc = "Screening Type 2 Compare Word 1 Register"]
pub mod st2cw1;
