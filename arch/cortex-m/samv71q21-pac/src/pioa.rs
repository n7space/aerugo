#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIO Enable Register"]
    pub per: PER,
    #[doc = "0x04 - PIO Disable Register"]
    pub pdr: PDR,
    #[doc = "0x08 - PIO Status Register"]
    pub psr: PSR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Output Enable Register"]
    pub oer: OER,
    #[doc = "0x14 - Output Disable Register"]
    pub odr: ODR,
    #[doc = "0x18 - Output Status Register"]
    pub osr: OSR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Glitch Input Filter Enable Register"]
    pub ifer: IFER,
    #[doc = "0x24 - Glitch Input Filter Disable Register"]
    pub ifdr: IFDR,
    #[doc = "0x28 - Glitch Input Filter Status Register"]
    pub ifsr: IFSR,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - Set Output Data Register"]
    pub sodr: SODR,
    #[doc = "0x34 - Clear Output Data Register"]
    pub codr: CODR,
    #[doc = "0x38 - Output Data Status Register"]
    pub odsr: ODSR,
    #[doc = "0x3c - Pin Data Status Register"]
    pub pdsr: PDSR,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x44 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x48 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x4c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x50 - Multi-driver Enable Register"]
    pub mder: MDER,
    #[doc = "0x54 - Multi-driver Disable Register"]
    pub mddr: MDDR,
    #[doc = "0x58 - Multi-driver Status Register"]
    pub mdsr: MDSR,
    _reserved20: [u8; 0x04],
    #[doc = "0x60 - Pull-up Disable Register"]
    pub pudr: PUDR,
    #[doc = "0x64 - Pull-up Enable Register"]
    pub puer: PUER,
    #[doc = "0x68 - Pad Pull-up Status Register"]
    pub pusr: PUSR,
    _reserved23: [u8; 0x04],
    #[doc = "0x70..0x78 - Peripheral ABCD Select Register 0"]
    pub abcdsr: [ABCDSR; 2],
    _reserved24: [u8; 0x08],
    #[doc = "0x80 - Input Filter Slow Clock Disable Register"]
    pub ifscdr: IFSCDR,
    #[doc = "0x84 - Input Filter Slow Clock Enable Register"]
    pub ifscer: IFSCER,
    #[doc = "0x88 - Input Filter Slow Clock Status Register"]
    pub ifscsr: IFSCSR,
    #[doc = "0x8c - Slow Clock Divider Debouncing Register"]
    pub scdr: SCDR,
    #[doc = "0x90 - Pad Pull-down Disable Register"]
    pub ppddr: PPDDR,
    #[doc = "0x94 - Pad Pull-down Enable Register"]
    pub ppder: PPDER,
    #[doc = "0x98 - Pad Pull-down Status Register"]
    pub ppdsr: PPDSR,
    _reserved31: [u8; 0x04],
    #[doc = "0xa0 - Output Write Enable"]
    pub ower: OWER,
    #[doc = "0xa4 - Output Write Disable"]
    pub owdr: OWDR,
    #[doc = "0xa8 - Output Write Status Register"]
    pub owsr: OWSR,
    _reserved34: [u8; 0x04],
    #[doc = "0xb0 - Additional Interrupt Modes Enable Register"]
    pub aimer: AIMER,
    #[doc = "0xb4 - Additional Interrupt Modes Disable Register"]
    pub aimdr: AIMDR,
    #[doc = "0xb8 - Additional Interrupt Modes Mask Register"]
    pub aimmr: AIMMR,
    _reserved37: [u8; 0x04],
    #[doc = "0xc0 - Edge Select Register"]
    pub esr: ESR,
    #[doc = "0xc4 - Level Select Register"]
    pub lsr: LSR,
    #[doc = "0xc8 - Edge/Level Status Register"]
    pub elsr: ELSR,
    _reserved40: [u8; 0x04],
    #[doc = "0xd0 - Falling Edge/Low-Level Select Register"]
    pub fellsr: FELLSR,
    #[doc = "0xd4 - Rising Edge/High-Level Select Register"]
    pub rehlsr: REHLSR,
    #[doc = "0xd8 - Fall/Rise - Low/High Status Register"]
    pub frlhsr: FRLHSR,
    _reserved43: [u8; 0x04],
    #[doc = "0xe0 - Lock Status"]
    pub locksr: LOCKSR,
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
    _reserved46: [u8; 0x14],
    #[doc = "0x100 - Schmitt Trigger Register"]
    pub schmitt: SCHMITT,
    _reserved47: [u8; 0x14],
    #[doc = "0x118 - I/O Drive Register"]
    pub driver: DRIVER,
    _reserved48: [u8; 0x34],
    #[doc = "0x150 - Parallel Capture Mode Register"]
    pub pcmr: PCMR,
    #[doc = "0x154 - Parallel Capture Interrupt Enable Register"]
    pub pcier: PCIER,
    #[doc = "0x158 - Parallel Capture Interrupt Disable Register"]
    pub pcidr: PCIDR,
    #[doc = "0x15c - Parallel Capture Interrupt Mask Register"]
    pub pcimr: PCIMR,
    #[doc = "0x160 - Parallel Capture Interrupt Status Register"]
    pub pcisr: PCISR,
    #[doc = "0x164 - Parallel Capture Reception Holding Register"]
    pub pcrhr: PCRHR,
}
#[doc = "PER (w) register accessor: PIO Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`per`]
module"]
pub type PER = crate::Reg<per::PER_SPEC>;
#[doc = "PIO Enable Register"]
pub mod per;
#[doc = "PDR (w) register accessor: PIO Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdr`]
module"]
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
#[doc = "PIO Disable Register"]
pub mod pdr;
#[doc = "PSR (r) register accessor: PIO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`psr`]
module"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "PIO Status Register"]
pub mod psr;
#[doc = "OER (w) register accessor: Output Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`oer`]
module"]
pub type OER = crate::Reg<oer::OER_SPEC>;
#[doc = "Output Enable Register"]
pub mod oer;
#[doc = "ODR (w) register accessor: Output Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`odr`]
module"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "Output Disable Register"]
pub mod odr;
#[doc = "OSR (r) register accessor: Output Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`osr`]
module"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Output Status Register"]
pub mod osr;
#[doc = "IFER (w) register accessor: Glitch Input Filter Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifer`]
module"]
pub type IFER = crate::Reg<ifer::IFER_SPEC>;
#[doc = "Glitch Input Filter Enable Register"]
pub mod ifer;
#[doc = "IFDR (w) register accessor: Glitch Input Filter Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifdr`]
module"]
pub type IFDR = crate::Reg<ifdr::IFDR_SPEC>;
#[doc = "Glitch Input Filter Disable Register"]
pub mod ifdr;
#[doc = "IFSR (r) register accessor: Glitch Input Filter Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifsr`]
module"]
pub type IFSR = crate::Reg<ifsr::IFSR_SPEC>;
#[doc = "Glitch Input Filter Status Register"]
pub mod ifsr;
#[doc = "SODR (w) register accessor: Set Output Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sodr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sodr`]
module"]
pub type SODR = crate::Reg<sodr::SODR_SPEC>;
#[doc = "Set Output Data Register"]
pub mod sodr;
#[doc = "CODR (w) register accessor: Clear Output Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`codr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`codr`]
module"]
pub type CODR = crate::Reg<codr::CODR_SPEC>;
#[doc = "Clear Output Data Register"]
pub mod codr;
#[doc = "ODSR (rw) register accessor: Output Data Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`odsr`]
module"]
pub type ODSR = crate::Reg<odsr::ODSR_SPEC>;
#[doc = "Output Data Status Register"]
pub mod odsr;
#[doc = "PDSR (r) register accessor: Pin Data Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdsr`]
module"]
pub type PDSR = crate::Reg<pdsr::PDSR_SPEC>;
#[doc = "Pin Data Status Register"]
pub mod pdsr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isr`]
module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "MDER (w) register accessor: Multi-driver Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mder::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mder`]
module"]
pub type MDER = crate::Reg<mder::MDER_SPEC>;
#[doc = "Multi-driver Enable Register"]
pub mod mder;
#[doc = "MDDR (w) register accessor: Multi-driver Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mddr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mddr`]
module"]
pub type MDDR = crate::Reg<mddr::MDDR_SPEC>;
#[doc = "Multi-driver Disable Register"]
pub mod mddr;
#[doc = "MDSR (r) register accessor: Multi-driver Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mdsr`]
module"]
pub type MDSR = crate::Reg<mdsr::MDSR_SPEC>;
#[doc = "Multi-driver Status Register"]
pub mod mdsr;
#[doc = "PUDR (w) register accessor: Pull-up Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pudr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pudr`]
module"]
pub type PUDR = crate::Reg<pudr::PUDR_SPEC>;
#[doc = "Pull-up Disable Register"]
pub mod pudr;
#[doc = "PUER (w) register accessor: Pull-up Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`puer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`puer`]
module"]
pub type PUER = crate::Reg<puer::PUER_SPEC>;
#[doc = "Pull-up Enable Register"]
pub mod puer;
#[doc = "PUSR (r) register accessor: Pad Pull-up Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pusr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pusr`]
module"]
pub type PUSR = crate::Reg<pusr::PUSR_SPEC>;
#[doc = "Pad Pull-up Status Register"]
pub mod pusr;
#[doc = "ABCDSR (rw) register accessor: Peripheral ABCD Select Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abcdsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abcdsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`abcdsr`]
module"]
pub type ABCDSR = crate::Reg<abcdsr::ABCDSR_SPEC>;
#[doc = "Peripheral ABCD Select Register 0"]
pub mod abcdsr;
#[doc = "IFSCDR (w) register accessor: Input Filter Slow Clock Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifscdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifscdr`]
module"]
pub type IFSCDR = crate::Reg<ifscdr::IFSCDR_SPEC>;
#[doc = "Input Filter Slow Clock Disable Register"]
pub mod ifscdr;
#[doc = "IFSCER (w) register accessor: Input Filter Slow Clock Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifscer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifscer`]
module"]
pub type IFSCER = crate::Reg<ifscer::IFSCER_SPEC>;
#[doc = "Input Filter Slow Clock Enable Register"]
pub mod ifscer;
#[doc = "IFSCSR (r) register accessor: Input Filter Slow Clock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifscsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ifscsr`]
module"]
pub type IFSCSR = crate::Reg<ifscsr::IFSCSR_SPEC>;
#[doc = "Input Filter Slow Clock Status Register"]
pub mod ifscsr;
#[doc = "SCDR (rw) register accessor: Slow Clock Divider Debouncing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scdr`]
module"]
pub type SCDR = crate::Reg<scdr::SCDR_SPEC>;
#[doc = "Slow Clock Divider Debouncing Register"]
pub mod scdr;
#[doc = "PPDDR (w) register accessor: Pad Pull-down Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppddr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ppddr`]
module"]
pub type PPDDR = crate::Reg<ppddr::PPDDR_SPEC>;
#[doc = "Pad Pull-down Disable Register"]
pub mod ppddr;
#[doc = "PPDER (w) register accessor: Pad Pull-down Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppder::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ppder`]
module"]
pub type PPDER = crate::Reg<ppder::PPDER_SPEC>;
#[doc = "Pad Pull-down Enable Register"]
pub mod ppder;
#[doc = "PPDSR (r) register accessor: Pad Pull-down Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppdsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ppdsr`]
module"]
pub type PPDSR = crate::Reg<ppdsr::PPDSR_SPEC>;
#[doc = "Pad Pull-down Status Register"]
pub mod ppdsr;
#[doc = "OWER (w) register accessor: Output Write Enable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ower::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ower`]
module"]
pub type OWER = crate::Reg<ower::OWER_SPEC>;
#[doc = "Output Write Enable"]
pub mod ower;
#[doc = "OWDR (w) register accessor: Output Write Disable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`owdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`owdr`]
module"]
pub type OWDR = crate::Reg<owdr::OWDR_SPEC>;
#[doc = "Output Write Disable"]
pub mod owdr;
#[doc = "OWSR (r) register accessor: Output Write Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`owsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`owsr`]
module"]
pub type OWSR = crate::Reg<owsr::OWSR_SPEC>;
#[doc = "Output Write Status Register"]
pub mod owsr;
#[doc = "AIMER (w) register accessor: Additional Interrupt Modes Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aimer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aimer`]
module"]
pub type AIMER = crate::Reg<aimer::AIMER_SPEC>;
#[doc = "Additional Interrupt Modes Enable Register"]
pub mod aimer;
#[doc = "AIMDR (w) register accessor: Additional Interrupt Modes Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aimdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aimdr`]
module"]
pub type AIMDR = crate::Reg<aimdr::AIMDR_SPEC>;
#[doc = "Additional Interrupt Modes Disable Register"]
pub mod aimdr;
#[doc = "AIMMR (r) register accessor: Additional Interrupt Modes Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aimmr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aimmr`]
module"]
pub type AIMMR = crate::Reg<aimmr::AIMMR_SPEC>;
#[doc = "Additional Interrupt Modes Mask Register"]
pub mod aimmr;
#[doc = "ESR (w) register accessor: Edge Select Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`esr`]
module"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "Edge Select Register"]
pub mod esr;
#[doc = "LSR (w) register accessor: Level Select Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lsr`]
module"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Level Select Register"]
pub mod lsr;
#[doc = "ELSR (r) register accessor: Edge/Level Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`elsr`]
module"]
pub type ELSR = crate::Reg<elsr::ELSR_SPEC>;
#[doc = "Edge/Level Status Register"]
pub mod elsr;
#[doc = "FELLSR (w) register accessor: Falling Edge/Low-Level Select Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fellsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fellsr`]
module"]
pub type FELLSR = crate::Reg<fellsr::FELLSR_SPEC>;
#[doc = "Falling Edge/Low-Level Select Register"]
pub mod fellsr;
#[doc = "REHLSR (w) register accessor: Rising Edge/High-Level Select Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rehlsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rehlsr`]
module"]
pub type REHLSR = crate::Reg<rehlsr::REHLSR_SPEC>;
#[doc = "Rising Edge/High-Level Select Register"]
pub mod rehlsr;
#[doc = "FRLHSR (r) register accessor: Fall/Rise - Low/High Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frlhsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`frlhsr`]
module"]
pub type FRLHSR = crate::Reg<frlhsr::FRLHSR_SPEC>;
#[doc = "Fall/Rise - Low/High Status Register"]
pub mod frlhsr;
#[doc = "LOCKSR (r) register accessor: Lock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`locksr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`locksr`]
module"]
pub type LOCKSR = crate::Reg<locksr::LOCKSR_SPEC>;
#[doc = "Lock Status"]
pub mod locksr;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpmr`]
module"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
#[doc = "SCHMITT (rw) register accessor: Schmitt Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`schmitt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`schmitt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`schmitt`]
module"]
pub type SCHMITT = crate::Reg<schmitt::SCHMITT_SPEC>;
#[doc = "Schmitt Trigger Register"]
pub mod schmitt;
#[doc = "DRIVER (rw) register accessor: I/O Drive Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`driver::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`driver::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`driver`]
module"]
pub type DRIVER = crate::Reg<driver::DRIVER_SPEC>;
#[doc = "I/O Drive Register"]
pub mod driver;
#[doc = "PCMR (rw) register accessor: Parallel Capture Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcmr`]
module"]
pub type PCMR = crate::Reg<pcmr::PCMR_SPEC>;
#[doc = "Parallel Capture Mode Register"]
pub mod pcmr;
#[doc = "PCIER (w) register accessor: Parallel Capture Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcier`]
module"]
pub type PCIER = crate::Reg<pcier::PCIER_SPEC>;
#[doc = "Parallel Capture Interrupt Enable Register"]
pub mod pcier;
#[doc = "PCIDR (w) register accessor: Parallel Capture Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcidr`]
module"]
pub type PCIDR = crate::Reg<pcidr::PCIDR_SPEC>;
#[doc = "Parallel Capture Interrupt Disable Register"]
pub mod pcidr;
#[doc = "PCIMR (r) register accessor: Parallel Capture Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcimr`]
module"]
pub type PCIMR = crate::Reg<pcimr::PCIMR_SPEC>;
#[doc = "Parallel Capture Interrupt Mask Register"]
pub mod pcimr;
#[doc = "PCISR (r) register accessor: Parallel Capture Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcisr`]
module"]
pub type PCISR = crate::Reg<pcisr::PCISR_SPEC>;
#[doc = "Parallel Capture Interrupt Status Register"]
pub mod pcisr;
#[doc = "PCRHR (r) register accessor: Parallel Capture Reception Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrhr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcrhr`]
module"]
pub type PCRHR = crate::Reg<pcrhr::PCRHR_SPEC>;
#[doc = "Parallel Capture Reception Holding Register"]
pub mod pcrhr;
