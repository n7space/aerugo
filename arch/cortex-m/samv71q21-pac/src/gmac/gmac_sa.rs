#[doc = r"Register block"]
#[repr(C)]
pub struct GMAC_SA {
    #[doc = "0x00 - Specific Address 1 Bottom Register"]
    pub sab: SAB,
    #[doc = "0x04 - Specific Address 1 Top Register"]
    pub sat: SAT,
}
#[doc = "SAB (rw) register accessor: Specific Address 1 Bottom Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sab::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sab::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sab`]
module"]
pub type SAB = crate::Reg<sab::SAB_SPEC>;
#[doc = "Specific Address 1 Bottom Register"]
pub mod sab;
#[doc = "SAT (rw) register accessor: Specific Address 1 Top Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sat`]
module"]
pub type SAT = crate::Reg<sat::SAT_SPEC>;
#[doc = "Specific Address 1 Top Register"]
pub mod sat;
