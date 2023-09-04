#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x34 - Channel Control Register (channel = 0)"]
    pub tc_channel0: TC_CHANNEL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x40..0x74 - Channel Control Register (channel = 0)"]
    pub tc_channel1: TC_CHANNEL,
    _reserved2: [u8; 0x0c],
    #[doc = "0x80..0xb4 - Channel Control Register (channel = 0)"]
    pub tc_channel2: TC_CHANNEL,
    _reserved3: [u8; 0x0c],
    #[doc = "0xc0 - Block Control Register"]
    pub bcr: BCR,
    #[doc = "0xc4 - Block Mode Register"]
    pub bmr: BMR,
    #[doc = "0xc8 - QDEC Interrupt Enable Register"]
    pub qier: QIER,
    #[doc = "0xcc - QDEC Interrupt Disable Register"]
    pub qidr: QIDR,
    #[doc = "0xd0 - QDEC Interrupt Mask Register"]
    pub qimr: QIMR,
    #[doc = "0xd4 - QDEC Interrupt Status Register"]
    pub qisr: QISR,
    #[doc = "0xd8 - Fault Mode Register"]
    pub fmr: FMR,
    _reserved10: [u8; 0x08],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
}
#[doc = "Channel Control Register (channel = 0)"]
pub use self::tc_channel::TC_CHANNEL;
#[doc = r"Cluster"]
#[doc = "Channel Control Register (channel = 0)"]
pub mod tc_channel;
#[doc = "BCR (w) register accessor: Block Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bcr`]
module"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "Block Control Register"]
pub mod bcr;
#[doc = "BMR (rw) register accessor: Block Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bmr`]
module"]
pub type BMR = crate::Reg<bmr::BMR_SPEC>;
#[doc = "Block Mode Register"]
pub mod bmr;
#[doc = "QIER (w) register accessor: QDEC Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`qier`]
module"]
pub type QIER = crate::Reg<qier::QIER_SPEC>;
#[doc = "QDEC Interrupt Enable Register"]
pub mod qier;
#[doc = "QIDR (w) register accessor: QDEC Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`qidr`]
module"]
pub type QIDR = crate::Reg<qidr::QIDR_SPEC>;
#[doc = "QDEC Interrupt Disable Register"]
pub mod qidr;
#[doc = "QIMR (r) register accessor: QDEC Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`qimr`]
module"]
pub type QIMR = crate::Reg<qimr::QIMR_SPEC>;
#[doc = "QDEC Interrupt Mask Register"]
pub mod qimr;
#[doc = "QISR (r) register accessor: QDEC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`qisr`]
module"]
pub type QISR = crate::Reg<qisr::QISR_SPEC>;
#[doc = "QDEC Interrupt Status Register"]
pub mod qisr;
#[doc = "FMR (rw) register accessor: Fault Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fmr`]
module"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "Fault Mode Register"]
pub mod fmr;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
