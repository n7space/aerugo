#[doc = r"Register block"]
#[repr(C)]
pub struct SMC_CS_NUMBER {
    #[doc = "0x00 - SMC Setup Register"]
    pub setup: SETUP,
    #[doc = "0x04 - SMC Pulse Register"]
    pub pulse: PULSE,
    #[doc = "0x08 - SMC Cycle Register"]
    pub cycle: CYCLE,
    #[doc = "0x0c - SMC Mode Register"]
    pub mode: MODE,
}
#[doc = "SETUP (rw) register accessor: SMC Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`setup`]
module"]
pub type SETUP = crate::Reg<setup::SETUP_SPEC>;
#[doc = "SMC Setup Register"]
pub mod setup;
#[doc = "PULSE (rw) register accessor: SMC Pulse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pulse`]
module"]
pub type PULSE = crate::Reg<pulse::PULSE_SPEC>;
#[doc = "SMC Pulse Register"]
pub mod pulse;
#[doc = "CYCLE (rw) register accessor: SMC Cycle Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cycle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cycle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cycle`]
module"]
pub type CYCLE = crate::Reg<cycle::CYCLE_SPEC>;
#[doc = "SMC Cycle Register"]
pub mod cycle;
#[doc = "MODE (rw) register accessor: SMC Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mode`]
module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "SMC Mode Register"]
pub mod mode;
