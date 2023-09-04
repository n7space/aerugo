#[doc = r"Register block"]
#[repr(C)]
pub struct MATRIX_PR {
    #[doc = "0x00 - Priority Register A for Slave 0"]
    pub pras: PRAS,
    #[doc = "0x04 - Priority Register B for Slave 0"]
    pub prbs: PRBS,
}
#[doc = "PRAS (rw) register accessor: Priority Register A for Slave 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pras::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pras::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pras`]
module"]
pub type PRAS = crate::Reg<pras::PRAS_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod pras;
#[doc = "PRBS (rw) register accessor: Priority Register B for Slave 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prbs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prbs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`prbs`]
module"]
pub type PRBS = crate::Reg<prbs::PRBS_SPEC>;
#[doc = "Priority Register B for Slave 0"]
pub mod prbs;
