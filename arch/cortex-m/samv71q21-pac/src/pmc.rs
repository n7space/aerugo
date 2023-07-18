#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Clock Enable Register"]
    pub scer: SCER,
    #[doc = "0x04 - System Clock Disable Register"]
    pub scdr: SCDR,
    #[doc = "0x08 - System Clock Status Register"]
    pub scsr: SCSR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Peripheral Clock Enable Register 0"]
    pub pcer0: PCER0,
    #[doc = "0x14 - Peripheral Clock Disable Register 0"]
    pub pcdr0: PCDR0,
    #[doc = "0x18 - Peripheral Clock Status Register 0"]
    pub pcsr0: PCSR0,
    #[doc = "0x1c - UTMI Clock Register"]
    pub ckgr_uckr: CKGR_UCKR,
    #[doc = "0x20 - Main Oscillator Register"]
    pub ckgr_mor: CKGR_MOR,
    #[doc = "0x24 - Main Clock Frequency Register"]
    pub ckgr_mcfr: CKGR_MCFR,
    #[doc = "0x28 - PLLA Register"]
    pub ckgr_pllar: CKGR_PLLAR,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Master Clock Register"]
    pub mckr: MCKR,
    _reserved11: [u8; 0x04],
    #[doc = "0x38 - USB Clock Register"]
    pub usb: USB,
    _reserved12: [u8; 0x04],
    #[doc = "0x40..0x60 - Programmable Clock Register"]
    pub pck: [PCK; 8],
    #[doc = "0x60 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x64 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x68 - Status Register"]
    pub sr: SR,
    #[doc = "0x6c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x70 - Fast Startup Mode Register"]
    pub fsmr: FSMR,
    #[doc = "0x74 - Fast Startup Polarity Register"]
    pub fspr: FSPR,
    #[doc = "0x78 - Fault Output Clear Register"]
    pub focr: FOCR,
    _reserved20: [u8; 0x68],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
    _reserved22: [u8; 0x14],
    #[doc = "0x100 - Peripheral Clock Enable Register 1"]
    pub pcer1: PCER1,
    #[doc = "0x104 - Peripheral Clock Disable Register 1"]
    pub pcdr1: PCDR1,
    #[doc = "0x108 - Peripheral Clock Status Register 1"]
    pub pcsr1: PCSR1,
    #[doc = "0x10c - Peripheral Control Register"]
    pub pcr: PCR,
    #[doc = "0x110 - Oscillator Calibration Register"]
    pub ocr: OCR,
    #[doc = "0x114 - SleepWalking Enable Register 0"]
    pub slpwk_er0: SLPWK_ER0,
    #[doc = "0x118 - SleepWalking Disable Register 0"]
    pub slpwk_dr0: SLPWK_DR0,
    #[doc = "0x11c - SleepWalking Status Register 0"]
    pub slpwk_sr0: SLPWK_SR0,
    #[doc = "0x120 - SleepWalking Activity Status Register 0"]
    pub slpwk_asr0: SLPWK_ASR0,
    _reserved31: [u8; 0x0c],
    #[doc = "0x130 - PLL Maximum Multiplier Value Register"]
    pub pmmr: PMMR,
    #[doc = "0x134 - SleepWalking Enable Register 1"]
    pub slpwk_er1: SLPWK_ER1,
    #[doc = "0x138 - SleepWalking Disable Register 1"]
    pub slpwk_dr1: SLPWK_DR1,
    #[doc = "0x13c - SleepWalking Status Register 1"]
    pub slpwk_sr1: SLPWK_SR1,
    #[doc = "0x140 - SleepWalking Activity Status Register 1"]
    pub slpwk_asr1: SLPWK_ASR1,
    #[doc = "0x144 - SleepWalking Activity In Progress Register"]
    pub slpwk_aipr: SLPWK_AIPR,
}
#[doc = "SCER (w) register accessor: an alias for `Reg<SCER_SPEC>`"]
pub type SCER = crate::Reg<scer::SCER_SPEC>;
#[doc = "System Clock Enable Register"]
pub mod scer;
#[doc = "SCDR (w) register accessor: an alias for `Reg<SCDR_SPEC>`"]
pub type SCDR = crate::Reg<scdr::SCDR_SPEC>;
#[doc = "System Clock Disable Register"]
pub mod scdr;
#[doc = "SCSR (r) register accessor: an alias for `Reg<SCSR_SPEC>`"]
pub type SCSR = crate::Reg<scsr::SCSR_SPEC>;
#[doc = "System Clock Status Register"]
pub mod scsr;
#[doc = "PCER0 (w) register accessor: an alias for `Reg<PCER0_SPEC>`"]
pub type PCER0 = crate::Reg<pcer0::PCER0_SPEC>;
#[doc = "Peripheral Clock Enable Register 0"]
pub mod pcer0;
#[doc = "PCDR0 (w) register accessor: an alias for `Reg<PCDR0_SPEC>`"]
pub type PCDR0 = crate::Reg<pcdr0::PCDR0_SPEC>;
#[doc = "Peripheral Clock Disable Register 0"]
pub mod pcdr0;
#[doc = "PCSR0 (r) register accessor: an alias for `Reg<PCSR0_SPEC>`"]
pub type PCSR0 = crate::Reg<pcsr0::PCSR0_SPEC>;
#[doc = "Peripheral Clock Status Register 0"]
pub mod pcsr0;
#[doc = "CKGR_UCKR (rw) register accessor: an alias for `Reg<CKGR_UCKR_SPEC>`"]
pub type CKGR_UCKR = crate::Reg<ckgr_uckr::CKGR_UCKR_SPEC>;
#[doc = "UTMI Clock Register"]
pub mod ckgr_uckr;
#[doc = "CKGR_MOR (rw) register accessor: an alias for `Reg<CKGR_MOR_SPEC>`"]
pub type CKGR_MOR = crate::Reg<ckgr_mor::CKGR_MOR_SPEC>;
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "CKGR_MCFR (rw) register accessor: an alias for `Reg<CKGR_MCFR_SPEC>`"]
pub type CKGR_MCFR = crate::Reg<ckgr_mcfr::CKGR_MCFR_SPEC>;
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "CKGR_PLLAR (rw) register accessor: an alias for `Reg<CKGR_PLLAR_SPEC>`"]
pub type CKGR_PLLAR = crate::Reg<ckgr_pllar::CKGR_PLLAR_SPEC>;
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "MCKR (rw) register accessor: an alias for `Reg<MCKR_SPEC>`"]
pub type MCKR = crate::Reg<mckr::MCKR_SPEC>;
#[doc = "Master Clock Register"]
pub mod mckr;
#[doc = "USB (rw) register accessor: an alias for `Reg<USB_SPEC>`"]
pub type USB = crate::Reg<usb::USB_SPEC>;
#[doc = "USB Clock Register"]
pub mod usb;
#[doc = "PCK (rw) register accessor: an alias for `Reg<PCK_SPEC>`"]
pub type PCK = crate::Reg<pck::PCK_SPEC>;
#[doc = "Programmable Clock Register"]
pub mod pck;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "FSMR (rw) register accessor: an alias for `Reg<FSMR_SPEC>`"]
pub type FSMR = crate::Reg<fsmr::FSMR_SPEC>;
#[doc = "Fast Startup Mode Register"]
pub mod fsmr;
#[doc = "FSPR (rw) register accessor: an alias for `Reg<FSPR_SPEC>`"]
pub type FSPR = crate::Reg<fspr::FSPR_SPEC>;
#[doc = "Fast Startup Polarity Register"]
pub mod fspr;
#[doc = "FOCR (w) register accessor: an alias for `Reg<FOCR_SPEC>`"]
pub type FOCR = crate::Reg<focr::FOCR_SPEC>;
#[doc = "Fault Output Clear Register"]
pub mod focr;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
#[doc = "PCER1 (w) register accessor: an alias for `Reg<PCER1_SPEC>`"]
pub type PCER1 = crate::Reg<pcer1::PCER1_SPEC>;
#[doc = "Peripheral Clock Enable Register 1"]
pub mod pcer1;
#[doc = "PCDR1 (w) register accessor: an alias for `Reg<PCDR1_SPEC>`"]
pub type PCDR1 = crate::Reg<pcdr1::PCDR1_SPEC>;
#[doc = "Peripheral Clock Disable Register 1"]
pub mod pcdr1;
#[doc = "PCSR1 (r) register accessor: an alias for `Reg<PCSR1_SPEC>`"]
pub type PCSR1 = crate::Reg<pcsr1::PCSR1_SPEC>;
#[doc = "Peripheral Clock Status Register 1"]
pub mod pcsr1;
#[doc = "PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "Peripheral Control Register"]
pub mod pcr;
#[doc = "OCR (rw) register accessor: an alias for `Reg<OCR_SPEC>`"]
pub type OCR = crate::Reg<ocr::OCR_SPEC>;
#[doc = "Oscillator Calibration Register"]
pub mod ocr;
#[doc = "SLPWK_ER0 (w) register accessor: an alias for `Reg<SLPWK_ER0_SPEC>`"]
pub type SLPWK_ER0 = crate::Reg<slpwk_er0::SLPWK_ER0_SPEC>;
#[doc = "SleepWalking Enable Register 0"]
pub mod slpwk_er0;
#[doc = "SLPWK_DR0 (w) register accessor: an alias for `Reg<SLPWK_DR0_SPEC>`"]
pub type SLPWK_DR0 = crate::Reg<slpwk_dr0::SLPWK_DR0_SPEC>;
#[doc = "SleepWalking Disable Register 0"]
pub mod slpwk_dr0;
#[doc = "SLPWK_SR0 (r) register accessor: an alias for `Reg<SLPWK_SR0_SPEC>`"]
pub type SLPWK_SR0 = crate::Reg<slpwk_sr0::SLPWK_SR0_SPEC>;
#[doc = "SleepWalking Status Register 0"]
pub mod slpwk_sr0;
#[doc = "SLPWK_ASR0 (r) register accessor: an alias for `Reg<SLPWK_ASR0_SPEC>`"]
pub type SLPWK_ASR0 = crate::Reg<slpwk_asr0::SLPWK_ASR0_SPEC>;
#[doc = "SleepWalking Activity Status Register 0"]
pub mod slpwk_asr0;
#[doc = "PMMR (rw) register accessor: an alias for `Reg<PMMR_SPEC>`"]
pub type PMMR = crate::Reg<pmmr::PMMR_SPEC>;
#[doc = "PLL Maximum Multiplier Value Register"]
pub mod pmmr;
#[doc = "SLPWK_ER1 (w) register accessor: an alias for `Reg<SLPWK_ER1_SPEC>`"]
pub type SLPWK_ER1 = crate::Reg<slpwk_er1::SLPWK_ER1_SPEC>;
#[doc = "SleepWalking Enable Register 1"]
pub mod slpwk_er1;
#[doc = "SLPWK_DR1 (w) register accessor: an alias for `Reg<SLPWK_DR1_SPEC>`"]
pub type SLPWK_DR1 = crate::Reg<slpwk_dr1::SLPWK_DR1_SPEC>;
#[doc = "SleepWalking Disable Register 1"]
pub mod slpwk_dr1;
#[doc = "SLPWK_SR1 (r) register accessor: an alias for `Reg<SLPWK_SR1_SPEC>`"]
pub type SLPWK_SR1 = crate::Reg<slpwk_sr1::SLPWK_SR1_SPEC>;
#[doc = "SleepWalking Status Register 1"]
pub mod slpwk_sr1;
#[doc = "SLPWK_ASR1 (r) register accessor: an alias for `Reg<SLPWK_ASR1_SPEC>`"]
pub type SLPWK_ASR1 = crate::Reg<slpwk_asr1::SLPWK_ASR1_SPEC>;
#[doc = "SleepWalking Activity Status Register 1"]
pub mod slpwk_asr1;
#[doc = "SLPWK_AIPR (r) register accessor: an alias for `Reg<SLPWK_AIPR_SPEC>`"]
pub type SLPWK_AIPR = crate::Reg<slpwk_aipr::SLPWK_AIPR_SPEC>;
#[doc = "SleepWalking Activity In Progress Register"]
pub mod slpwk_aipr;
