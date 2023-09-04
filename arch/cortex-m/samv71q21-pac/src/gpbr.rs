#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - General Purpose Backup Register 0"]
    pub sys_gpbr: [SYS_GPBR; 8],
}
#[doc = "SYS_GPBR (rw) register accessor: General Purpose Backup Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_gpbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_gpbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sys_gpbr`]
module"]
pub type SYS_GPBR = crate::Reg<sys_gpbr::SYS_GPBR_SPEC>;
#[doc = "General Purpose Backup Register 0"]
pub mod sys_gpbr;
