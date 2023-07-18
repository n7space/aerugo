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
#[doc = "OCMS (rw) register accessor: an alias for `Reg<OCMS_SPEC>`"]
pub type OCMS = crate::Reg<ocms::OCMS_SPEC>;
#[doc = "SMC Off-Chip Memory Scrambling Register"]
pub mod ocms;
#[doc = "KEY1 (w) register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "SMC Off-Chip Memory Scrambling KEY1 Register"]
pub mod key1;
#[doc = "KEY2 (w) register accessor: an alias for `Reg<KEY2_SPEC>`"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "SMC Off-Chip Memory Scrambling KEY2 Register"]
pub mod key2;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "SMC Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "SMC Write Protection Status Register"]
pub mod wpsr;
