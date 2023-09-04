#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - SMC Setup Register"]
    pub smc_cs_number: [SMC_CS_NUMBER; 4],
    _reserved1: [u8; 0x40],
    #[doc = "0x80 - SMC Off-Chip Memory Scrambling Register"]
    pub ocms: OCMS,
    #[doc = "0x84 - SMC Off-Chip Memory Scrambling KEY1 Register"]
    pub key1: KEY1,
    #[doc = "0x88 - SMC Off-Chip Memory Scrambling KEY2 Register"]
    pub key2: KEY2,
    _reserved4: [u8; 0x58],
    #[doc = "0xe4 - SMC Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - SMC Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = "SMC Setup Register"]
pub use self::smc_cs_number::SMC_CS_NUMBER;
#[doc = r"Cluster"]
#[doc = "SMC Setup Register"]
pub mod smc_cs_number;
#[doc = "OCMS (rw) register accessor: SMC Off-Chip Memory Scrambling Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocms::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocms::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ocms`]
module"]
pub type OCMS = crate::Reg<ocms::OCMS_SPEC>;
#[doc = "SMC Off-Chip Memory Scrambling Register"]
pub mod ocms;
#[doc = "KEY1 (w) register accessor: SMC Off-Chip Memory Scrambling KEY1 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key1`]
module"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "SMC Off-Chip Memory Scrambling KEY1 Register"]
pub mod key1;
#[doc = "KEY2 (w) register accessor: SMC Off-Chip Memory Scrambling KEY2 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`key2`]
module"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "SMC Off-Chip Memory Scrambling KEY2 Register"]
pub mod key2;
#[doc = "WPMR (rw) register accessor: SMC Write Protection Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "SMC Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: SMC Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "SMC Write Protection Status Register"]
pub mod wpsr;
