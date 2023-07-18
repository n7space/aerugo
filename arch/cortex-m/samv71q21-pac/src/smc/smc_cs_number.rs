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
#[doc = "SETUP (rw) register accessor: an alias for `Reg<SETUP_SPEC>`"]
pub type SETUP = crate::Reg<setup::SETUP_SPEC>;
#[doc = "SMC Setup Register"]
pub mod setup;
#[doc = "PULSE (rw) register accessor: an alias for `Reg<PULSE_SPEC>`"]
pub type PULSE = crate::Reg<pulse::PULSE_SPEC>;
#[doc = "SMC Pulse Register"]
pub mod pulse;
#[doc = "CYCLE (rw) register accessor: an alias for `Reg<CYCLE_SPEC>`"]
pub type CYCLE = crate::Reg<cycle::CYCLE_SPEC>;
#[doc = "SMC Cycle Register"]
pub mod cycle;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "SMC Mode Register"]
pub mod mode;
