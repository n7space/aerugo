#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock Bits Word 0"]
    pub word0: WORD0,
    #[doc = "0x04 - Lock Bits Word 1"]
    pub word1: WORD1,
    #[doc = "0x08 - Lock Bits Word 2"]
    pub word2: WORD2,
    #[doc = "0x0c - Lock Bits Word 3"]
    pub word3: WORD3,
}
#[doc = "WORD0 (rw) register accessor: Lock Bits Word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`word0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`word0`]
module"]
pub type WORD0 = crate::Reg<word0::WORD0_SPEC>;
#[doc = "Lock Bits Word 0"]
pub mod word0;
#[doc = "WORD1 (rw) register accessor: Lock Bits Word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`word1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`word1`]
module"]
pub type WORD1 = crate::Reg<word1::WORD1_SPEC>;
#[doc = "Lock Bits Word 1"]
pub mod word1;
#[doc = "WORD2 (rw) register accessor: Lock Bits Word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`word2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`word2`]
module"]
pub type WORD2 = crate::Reg<word2::WORD2_SPEC>;
#[doc = "Lock Bits Word 2"]
pub mod word2;
#[doc = "WORD3 (rw) register accessor: Lock Bits Word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`word3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`word3`]
module"]
pub type WORD3 = crate::Reg<word3::WORD3_SPEC>;
#[doc = "Lock Bits Word 3"]
pub mod word3;
