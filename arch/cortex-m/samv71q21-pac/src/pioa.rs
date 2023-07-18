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
#[doc = "PER (w) register accessor: an alias for `Reg<PER_SPEC>`"]
pub type PER = crate::Reg<per::PER_SPEC>;
#[doc = "PIO Enable Register"]
pub mod per;
#[doc = "PDR (w) register accessor: an alias for `Reg<PDR_SPEC>`"]
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
#[doc = "PIO Disable Register"]
pub mod pdr;
#[doc = "PSR (r) register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "PIO Status Register"]
pub mod psr;
#[doc = "OER (w) register accessor: an alias for `Reg<OER_SPEC>`"]
pub type OER = crate::Reg<oer::OER_SPEC>;
#[doc = "Output Enable Register"]
pub mod oer;
#[doc = "ODR (w) register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "Output Disable Register"]
pub mod odr;
#[doc = "OSR (r) register accessor: an alias for `Reg<OSR_SPEC>`"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Output Status Register"]
pub mod osr;
#[doc = "IFER (w) register accessor: an alias for `Reg<IFER_SPEC>`"]
pub type IFER = crate::Reg<ifer::IFER_SPEC>;
#[doc = "Glitch Input Filter Enable Register"]
pub mod ifer;
#[doc = "IFDR (w) register accessor: an alias for `Reg<IFDR_SPEC>`"]
pub type IFDR = crate::Reg<ifdr::IFDR_SPEC>;
#[doc = "Glitch Input Filter Disable Register"]
pub mod ifdr;
#[doc = "IFSR (r) register accessor: an alias for `Reg<IFSR_SPEC>`"]
pub type IFSR = crate::Reg<ifsr::IFSR_SPEC>;
#[doc = "Glitch Input Filter Status Register"]
pub mod ifsr;
#[doc = "SODR (w) register accessor: an alias for `Reg<SODR_SPEC>`"]
pub type SODR = crate::Reg<sodr::SODR_SPEC>;
#[doc = "Set Output Data Register"]
pub mod sodr;
#[doc = "CODR (w) register accessor: an alias for `Reg<CODR_SPEC>`"]
pub type CODR = crate::Reg<codr::CODR_SPEC>;
#[doc = "Clear Output Data Register"]
pub mod codr;
#[doc = "ODSR (rw) register accessor: an alias for `Reg<ODSR_SPEC>`"]
pub type ODSR = crate::Reg<odsr::ODSR_SPEC>;
#[doc = "Output Data Status Register"]
pub mod odsr;
#[doc = "PDSR (r) register accessor: an alias for `Reg<PDSR_SPEC>`"]
pub type PDSR = crate::Reg<pdsr::PDSR_SPEC>;
#[doc = "Pin Data Status Register"]
pub mod pdsr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "MDER (w) register accessor: an alias for `Reg<MDER_SPEC>`"]
pub type MDER = crate::Reg<mder::MDER_SPEC>;
#[doc = "Multi-driver Enable Register"]
pub mod mder;
#[doc = "MDDR (w) register accessor: an alias for `Reg<MDDR_SPEC>`"]
pub type MDDR = crate::Reg<mddr::MDDR_SPEC>;
#[doc = "Multi-driver Disable Register"]
pub mod mddr;
#[doc = "MDSR (r) register accessor: an alias for `Reg<MDSR_SPEC>`"]
pub type MDSR = crate::Reg<mdsr::MDSR_SPEC>;
#[doc = "Multi-driver Status Register"]
pub mod mdsr;
#[doc = "PUDR (w) register accessor: an alias for `Reg<PUDR_SPEC>`"]
pub type PUDR = crate::Reg<pudr::PUDR_SPEC>;
#[doc = "Pull-up Disable Register"]
pub mod pudr;
#[doc = "PUER (w) register accessor: an alias for `Reg<PUER_SPEC>`"]
pub type PUER = crate::Reg<puer::PUER_SPEC>;
#[doc = "Pull-up Enable Register"]
pub mod puer;
#[doc = "PUSR (r) register accessor: an alias for `Reg<PUSR_SPEC>`"]
pub type PUSR = crate::Reg<pusr::PUSR_SPEC>;
#[doc = "Pad Pull-up Status Register"]
pub mod pusr;
#[doc = "ABCDSR (rw) register accessor: an alias for `Reg<ABCDSR_SPEC>`"]
pub type ABCDSR = crate::Reg<abcdsr::ABCDSR_SPEC>;
#[doc = "Peripheral ABCD Select Register 0"]
pub mod abcdsr;
#[doc = "IFSCDR (w) register accessor: an alias for `Reg<IFSCDR_SPEC>`"]
pub type IFSCDR = crate::Reg<ifscdr::IFSCDR_SPEC>;
#[doc = "Input Filter Slow Clock Disable Register"]
pub mod ifscdr;
#[doc = "IFSCER (w) register accessor: an alias for `Reg<IFSCER_SPEC>`"]
pub type IFSCER = crate::Reg<ifscer::IFSCER_SPEC>;
#[doc = "Input Filter Slow Clock Enable Register"]
pub mod ifscer;
#[doc = "IFSCSR (r) register accessor: an alias for `Reg<IFSCSR_SPEC>`"]
pub type IFSCSR = crate::Reg<ifscsr::IFSCSR_SPEC>;
#[doc = "Input Filter Slow Clock Status Register"]
pub mod ifscsr;
#[doc = "SCDR (rw) register accessor: an alias for `Reg<SCDR_SPEC>`"]
pub type SCDR = crate::Reg<scdr::SCDR_SPEC>;
#[doc = "Slow Clock Divider Debouncing Register"]
pub mod scdr;
#[doc = "PPDDR (w) register accessor: an alias for `Reg<PPDDR_SPEC>`"]
pub type PPDDR = crate::Reg<ppddr::PPDDR_SPEC>;
#[doc = "Pad Pull-down Disable Register"]
pub mod ppddr;
#[doc = "PPDER (w) register accessor: an alias for `Reg<PPDER_SPEC>`"]
pub type PPDER = crate::Reg<ppder::PPDER_SPEC>;
#[doc = "Pad Pull-down Enable Register"]
pub mod ppder;
#[doc = "PPDSR (r) register accessor: an alias for `Reg<PPDSR_SPEC>`"]
pub type PPDSR = crate::Reg<ppdsr::PPDSR_SPEC>;
#[doc = "Pad Pull-down Status Register"]
pub mod ppdsr;
#[doc = "OWER (w) register accessor: an alias for `Reg<OWER_SPEC>`"]
pub type OWER = crate::Reg<ower::OWER_SPEC>;
#[doc = "Output Write Enable"]
pub mod ower;
#[doc = "OWDR (w) register accessor: an alias for `Reg<OWDR_SPEC>`"]
pub type OWDR = crate::Reg<owdr::OWDR_SPEC>;
#[doc = "Output Write Disable"]
pub mod owdr;
#[doc = "OWSR (r) register accessor: an alias for `Reg<OWSR_SPEC>`"]
pub type OWSR = crate::Reg<owsr::OWSR_SPEC>;
#[doc = "Output Write Status Register"]
pub mod owsr;
#[doc = "AIMER (w) register accessor: an alias for `Reg<AIMER_SPEC>`"]
pub type AIMER = crate::Reg<aimer::AIMER_SPEC>;
#[doc = "Additional Interrupt Modes Enable Register"]
pub mod aimer;
#[doc = "AIMDR (w) register accessor: an alias for `Reg<AIMDR_SPEC>`"]
pub type AIMDR = crate::Reg<aimdr::AIMDR_SPEC>;
#[doc = "Additional Interrupt Modes Disable Register"]
pub mod aimdr;
#[doc = "AIMMR (r) register accessor: an alias for `Reg<AIMMR_SPEC>`"]
pub type AIMMR = crate::Reg<aimmr::AIMMR_SPEC>;
#[doc = "Additional Interrupt Modes Mask Register"]
pub mod aimmr;
#[doc = "ESR (w) register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "Edge Select Register"]
pub mod esr;
#[doc = "LSR (w) register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Level Select Register"]
pub mod lsr;
#[doc = "ELSR (r) register accessor: an alias for `Reg<ELSR_SPEC>`"]
pub type ELSR = crate::Reg<elsr::ELSR_SPEC>;
#[doc = "Edge/Level Status Register"]
pub mod elsr;
#[doc = "FELLSR (w) register accessor: an alias for `Reg<FELLSR_SPEC>`"]
pub type FELLSR = crate::Reg<fellsr::FELLSR_SPEC>;
#[doc = "Falling Edge/Low-Level Select Register"]
pub mod fellsr;
#[doc = "REHLSR (w) register accessor: an alias for `Reg<REHLSR_SPEC>`"]
pub type REHLSR = crate::Reg<rehlsr::REHLSR_SPEC>;
#[doc = "Rising Edge/High-Level Select Register"]
pub mod rehlsr;
#[doc = "FRLHSR (r) register accessor: an alias for `Reg<FRLHSR_SPEC>`"]
pub type FRLHSR = crate::Reg<frlhsr::FRLHSR_SPEC>;
#[doc = "Fall/Rise - Low/High Status Register"]
pub mod frlhsr;
#[doc = "LOCKSR (r) register accessor: an alias for `Reg<LOCKSR_SPEC>`"]
pub type LOCKSR = crate::Reg<locksr::LOCKSR_SPEC>;
#[doc = "Lock Status"]
pub mod locksr;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
#[doc = "SCHMITT (rw) register accessor: an alias for `Reg<SCHMITT_SPEC>`"]
pub type SCHMITT = crate::Reg<schmitt::SCHMITT_SPEC>;
#[doc = "Schmitt Trigger Register"]
pub mod schmitt;
#[doc = "DRIVER (rw) register accessor: an alias for `Reg<DRIVER_SPEC>`"]
pub type DRIVER = crate::Reg<driver::DRIVER_SPEC>;
#[doc = "I/O Drive Register"]
pub mod driver;
#[doc = "PCMR (rw) register accessor: an alias for `Reg<PCMR_SPEC>`"]
pub type PCMR = crate::Reg<pcmr::PCMR_SPEC>;
#[doc = "Parallel Capture Mode Register"]
pub mod pcmr;
#[doc = "PCIER (w) register accessor: an alias for `Reg<PCIER_SPEC>`"]
pub type PCIER = crate::Reg<pcier::PCIER_SPEC>;
#[doc = "Parallel Capture Interrupt Enable Register"]
pub mod pcier;
#[doc = "PCIDR (w) register accessor: an alias for `Reg<PCIDR_SPEC>`"]
pub type PCIDR = crate::Reg<pcidr::PCIDR_SPEC>;
#[doc = "Parallel Capture Interrupt Disable Register"]
pub mod pcidr;
#[doc = "PCIMR (r) register accessor: an alias for `Reg<PCIMR_SPEC>`"]
pub type PCIMR = crate::Reg<pcimr::PCIMR_SPEC>;
#[doc = "Parallel Capture Interrupt Mask Register"]
pub mod pcimr;
#[doc = "PCISR (r) register accessor: an alias for `Reg<PCISR_SPEC>`"]
pub type PCISR = crate::Reg<pcisr::PCISR_SPEC>;
#[doc = "Parallel Capture Interrupt Status Register"]
pub mod pcisr;
#[doc = "PCRHR (r) register accessor: an alias for `Reg<PCRHR_SPEC>`"]
pub type PCRHR = crate::Reg<pcrhr::PCRHR_SPEC>;
#[doc = "Parallel Capture Reception Holding Register"]
pub mod pcrhr;
