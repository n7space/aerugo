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
#[doc = "SCER (w) register accessor: System Clock Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scer`]
module"]
pub type SCER = crate::Reg<scer::SCER_SPEC>;
#[doc = "System Clock Enable Register"]
pub mod scer;
#[doc = "SCDR (w) register accessor: System Clock Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scdr`]
module"]
pub type SCDR = crate::Reg<scdr::SCDR_SPEC>;
#[doc = "System Clock Disable Register"]
pub mod scdr;
#[doc = "SCSR (r) register accessor: System Clock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scsr`]
module"]
pub type SCSR = crate::Reg<scsr::SCSR_SPEC>;
#[doc = "System Clock Status Register"]
pub mod scsr;
#[doc = "PCER0 (w) register accessor: Peripheral Clock Enable Register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcer0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcer0`]
module"]
pub type PCER0 = crate::Reg<pcer0::PCER0_SPEC>;
#[doc = "Peripheral Clock Enable Register 0"]
pub mod pcer0;
#[doc = "PCDR0 (w) register accessor: Peripheral Clock Disable Register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcdr0`]
module"]
pub type PCDR0 = crate::Reg<pcdr0::PCDR0_SPEC>;
#[doc = "Peripheral Clock Disable Register 0"]
pub mod pcdr0;
#[doc = "PCSR0 (r) register accessor: Peripheral Clock Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcsr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcsr0`]
module"]
pub type PCSR0 = crate::Reg<pcsr0::PCSR0_SPEC>;
#[doc = "Peripheral Clock Status Register 0"]
pub mod pcsr0;
#[doc = "CKGR_UCKR (rw) register accessor: UTMI Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_uckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_uckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ckgr_uckr`]
module"]
pub type CKGR_UCKR = crate::Reg<ckgr_uckr::CKGR_UCKR_SPEC>;
#[doc = "UTMI Clock Register"]
pub mod ckgr_uckr;
#[doc = "CKGR_MOR (rw) register accessor: Main Oscillator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_mor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_mor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ckgr_mor`]
module"]
pub type CKGR_MOR = crate::Reg<ckgr_mor::CKGR_MOR_SPEC>;
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "CKGR_MCFR (rw) register accessor: Main Clock Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_mcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_mcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ckgr_mcfr`]
module"]
pub type CKGR_MCFR = crate::Reg<ckgr_mcfr::CKGR_MCFR_SPEC>;
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "CKGR_PLLAR (rw) register accessor: PLLA Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_pllar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_pllar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ckgr_pllar`]
module"]
pub type CKGR_PLLAR = crate::Reg<ckgr_pllar::CKGR_PLLAR_SPEC>;
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "MCKR (rw) register accessor: Master Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mckr`]
module"]
pub type MCKR = crate::Reg<mckr::MCKR_SPEC>;
#[doc = "Master Clock Register"]
pub mod mckr;
#[doc = "USB (rw) register accessor: USB Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usb`]
module"]
pub type USB = crate::Reg<usb::USB_SPEC>;
#[doc = "USB Clock Register"]
pub mod usb;
#[doc = "PCK (rw) register accessor: Programmable Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pck::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pck::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pck`]
module"]
pub type PCK = crate::Reg<pck::PCK_SPEC>;
#[doc = "Programmable Clock Register"]
pub mod pck;
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
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "FSMR (rw) register accessor: Fast Startup Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fsmr`]
module"]
pub type FSMR = crate::Reg<fsmr::FSMR_SPEC>;
#[doc = "Fast Startup Mode Register"]
pub mod fsmr;
#[doc = "FSPR (rw) register accessor: Fast Startup Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fspr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fspr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fspr`]
module"]
pub type FSPR = crate::Reg<fspr::FSPR_SPEC>;
#[doc = "Fast Startup Polarity Register"]
pub mod fspr;
#[doc = "FOCR (w) register accessor: Fault Output Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`focr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`focr`]
module"]
pub type FOCR = crate::Reg<focr::FOCR_SPEC>;
#[doc = "Fault Output Clear Register"]
pub mod focr;
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
#[doc = "PCER1 (w) register accessor: Peripheral Clock Enable Register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcer1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcer1`]
module"]
pub type PCER1 = crate::Reg<pcer1::PCER1_SPEC>;
#[doc = "Peripheral Clock Enable Register 1"]
pub mod pcer1;
#[doc = "PCDR1 (w) register accessor: Peripheral Clock Disable Register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcdr1`]
module"]
pub type PCDR1 = crate::Reg<pcdr1::PCDR1_SPEC>;
#[doc = "Peripheral Clock Disable Register 1"]
pub mod pcdr1;
#[doc = "PCSR1 (r) register accessor: Peripheral Clock Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcsr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcsr1`]
module"]
pub type PCSR1 = crate::Reg<pcsr1::PCSR1_SPEC>;
#[doc = "Peripheral Clock Status Register 1"]
pub mod pcsr1;
#[doc = "PCR (rw) register accessor: Peripheral Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcr`]
module"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "Peripheral Control Register"]
pub mod pcr;
#[doc = "OCR (rw) register accessor: Oscillator Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ocr`]
module"]
pub type OCR = crate::Reg<ocr::OCR_SPEC>;
#[doc = "Oscillator Calibration Register"]
pub mod ocr;
#[doc = "SLPWK_ER0 (w) register accessor: SleepWalking Enable Register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slpwk_er0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slpwk_er0`]
module"]
pub type SLPWK_ER0 = crate::Reg<slpwk_er0::SLPWK_ER0_SPEC>;
#[doc = "SleepWalking Enable Register 0"]
pub mod slpwk_er0;
#[doc = "SLPWK_DR0 (w) register accessor: SleepWalking Disable Register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slpwk_dr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slpwk_dr0`]
module"]
pub type SLPWK_DR0 = crate::Reg<slpwk_dr0::SLPWK_DR0_SPEC>;
#[doc = "SleepWalking Disable Register 0"]
pub mod slpwk_dr0;
#[doc = "SLPWK_SR0 (r) register accessor: SleepWalking Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slpwk_sr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slpwk_sr0`]
module"]
pub type SLPWK_SR0 = crate::Reg<slpwk_sr0::SLPWK_SR0_SPEC>;
#[doc = "SleepWalking Status Register 0"]
pub mod slpwk_sr0;
#[doc = "SLPWK_ASR0 (r) register accessor: SleepWalking Activity Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slpwk_asr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slpwk_asr0`]
module"]
pub type SLPWK_ASR0 = crate::Reg<slpwk_asr0::SLPWK_ASR0_SPEC>;
#[doc = "SleepWalking Activity Status Register 0"]
pub mod slpwk_asr0;
#[doc = "PMMR (rw) register accessor: PLL Maximum Multiplier Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pmmr`]
module"]
pub type PMMR = crate::Reg<pmmr::PMMR_SPEC>;
#[doc = "PLL Maximum Multiplier Value Register"]
pub mod pmmr;
#[doc = "SLPWK_ER1 (w) register accessor: SleepWalking Enable Register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slpwk_er1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slpwk_er1`]
module"]
pub type SLPWK_ER1 = crate::Reg<slpwk_er1::SLPWK_ER1_SPEC>;
#[doc = "SleepWalking Enable Register 1"]
pub mod slpwk_er1;
#[doc = "SLPWK_DR1 (w) register accessor: SleepWalking Disable Register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slpwk_dr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slpwk_dr1`]
module"]
pub type SLPWK_DR1 = crate::Reg<slpwk_dr1::SLPWK_DR1_SPEC>;
#[doc = "SleepWalking Disable Register 1"]
pub mod slpwk_dr1;
#[doc = "SLPWK_SR1 (r) register accessor: SleepWalking Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slpwk_sr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slpwk_sr1`]
module"]
pub type SLPWK_SR1 = crate::Reg<slpwk_sr1::SLPWK_SR1_SPEC>;
#[doc = "SleepWalking Status Register 1"]
pub mod slpwk_sr1;
#[doc = "SLPWK_ASR1 (r) register accessor: SleepWalking Activity Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slpwk_asr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slpwk_asr1`]
module"]
pub type SLPWK_ASR1 = crate::Reg<slpwk_asr1::SLPWK_ASR1_SPEC>;
#[doc = "SleepWalking Activity Status Register 1"]
pub mod slpwk_asr1;
#[doc = "SLPWK_AIPR (r) register accessor: SleepWalking Activity In Progress Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slpwk_aipr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slpwk_aipr`]
module"]
pub type SLPWK_AIPR = crate::Reg<slpwk_aipr::SLPWK_AIPR_SPEC>;
#[doc = "SleepWalking Activity In Progress Register"]
pub mod slpwk_aipr;
