#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Clock Register"]
    pub clk: CLK,
    #[doc = "0x04 - PWM Enable Register"]
    pub ena: ENA,
    #[doc = "0x08 - PWM Disable Register"]
    pub dis: DIS,
    #[doc = "0x0c - PWM Status Register"]
    pub sr: SR,
    #[doc = "0x10 - PWM Interrupt Enable Register 1"]
    pub ier1: IER1,
    #[doc = "0x14 - PWM Interrupt Disable Register 1"]
    pub idr1: IDR1,
    #[doc = "0x18 - PWM Interrupt Mask Register 1"]
    pub imr1: IMR1,
    #[doc = "0x1c - PWM Interrupt Status Register 1"]
    pub isr1: ISR1,
    #[doc = "0x20 - PWM Sync Channels Mode Register"]
    pub scm: SCM,
    #[doc = "0x24 - PWM DMA Register"]
    pub dmar: DMAR,
    #[doc = "0x28 - PWM Sync Channels Update Control Register"]
    pub scuc: SCUC,
    #[doc = "0x2c - PWM Sync Channels Update Period Register"]
    pub scup: SCUP,
    #[doc = "0x30 - PWM Sync Channels Update Period Update Register"]
    pub scupupd: SCUPUPD,
    #[doc = "0x34 - PWM Interrupt Enable Register 2"]
    pub ier2: IER2,
    #[doc = "0x38 - PWM Interrupt Disable Register 2"]
    pub idr2: IDR2,
    #[doc = "0x3c - PWM Interrupt Mask Register 2"]
    pub imr2: IMR2,
    #[doc = "0x40 - PWM Interrupt Status Register 2"]
    pub isr2: ISR2,
    #[doc = "0x44 - PWM Output Override Value Register"]
    pub oov: OOV,
    #[doc = "0x48 - PWM Output Selection Register"]
    pub os: OS,
    #[doc = "0x4c - PWM Output Selection Set Register"]
    pub oss: OSS,
    #[doc = "0x50 - PWM Output Selection Clear Register"]
    pub osc: OSC,
    #[doc = "0x54 - PWM Output Selection Set Update Register"]
    pub ossupd: OSSUPD,
    #[doc = "0x58 - PWM Output Selection Clear Update Register"]
    pub oscupd: OSCUPD,
    #[doc = "0x5c - PWM Fault Mode Register"]
    pub fmr: FMR,
    #[doc = "0x60 - PWM Fault Status Register"]
    pub fsr: FSR,
    #[doc = "0x64 - PWM Fault Clear Register"]
    pub fcr: FCR,
    #[doc = "0x68 - PWM Fault Protection Value Register 1"]
    pub fpv1: FPV1,
    #[doc = "0x6c - PWM Fault Protection Enable Register"]
    pub fpe: FPE,
    _reserved28: [u8; 0x0c],
    #[doc = "0x7c..0x84 - PWM Event Line 0 Mode Register 0"]
    pub elmr: [ELMR; 2],
    _reserved29: [u8; 0x1c],
    #[doc = "0xa0 - PWM Spread Spectrum Register"]
    pub sspr: SSPR,
    #[doc = "0xa4 - PWM Spread Spectrum Update Register"]
    pub sspup: SSPUP,
    _reserved31: [u8; 0x08],
    #[doc = "0xb0 - PWM Stepper Motor Mode Register"]
    pub smmr: SMMR,
    _reserved32: [u8; 0x0c],
    #[doc = "0xc0 - PWM Fault Protection Value 2 Register"]
    pub fpv2: FPV2,
    _reserved33: [u8; 0x20],
    #[doc = "0xe4 - PWM Write Protection Control Register"]
    pub wpcr: WPCR,
    #[doc = "0xe8 - PWM Write Protection Status Register"]
    pub wpsr: WPSR,
    _reserved35: [u8; 0x44],
    #[doc = "0x130..0x1b0 - PWM Comparison 0 Value Register"]
    pub pwm_cmp: [PWM_CMP; 8],
    _reserved36: [u8; 0x50],
    #[doc = "0x200..0x280 - PWM Channel Mode Register"]
    pub pwm_ch_num: [PWM_CH_NUM; 4],
    _reserved37: [u8; 0x0180],
    #[doc = "0x400 - PWM Channel Mode Update Register (ch_num = 0)"]
    pub cmupd0: CMUPD0,
    _reserved38: [u8; 0x1c],
    #[doc = "0x420 - PWM Channel Mode Update Register (ch_num = 1)"]
    pub cmupd1: CMUPD1,
    _reserved39: [u8; 0x08],
    #[doc = "0x42c - PWM External Trigger Register (trg_num = 1)"]
    pub etrg1: ETRG1,
    #[doc = "0x430 - PWM Leading-Edge Blanking Register (trg_num = 1)"]
    pub lebr1: LEBR1,
    _reserved41: [u8; 0x0c],
    #[doc = "0x440 - PWM Channel Mode Update Register (ch_num = 2)"]
    pub cmupd2: CMUPD2,
    _reserved42: [u8; 0x08],
    #[doc = "0x44c - PWM External Trigger Register (trg_num = 2)"]
    pub etrg2: ETRG2,
    #[doc = "0x450 - PWM Leading-Edge Blanking Register (trg_num = 2)"]
    pub lebr2: LEBR2,
    _reserved44: [u8; 0x0c],
    #[doc = "0x460 - PWM Channel Mode Update Register (ch_num = 3)"]
    pub cmupd3: CMUPD3,
}
#[doc = "CLK (rw) register accessor: PWM Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk`]
module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "PWM Clock Register"]
pub mod clk;
#[doc = "ENA (w) register accessor: PWM Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ena::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ena`]
module"]
pub type ENA = crate::Reg<ena::ENA_SPEC>;
#[doc = "PWM Enable Register"]
pub mod ena;
#[doc = "DIS (w) register accessor: PWM Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dis`]
module"]
pub type DIS = crate::Reg<dis::DIS_SPEC>;
#[doc = "PWM Disable Register"]
pub mod dis;
#[doc = "SR (r) register accessor: PWM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "PWM Status Register"]
pub mod sr;
#[doc = "IER1 (w) register accessor: PWM Interrupt Enable Register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ier1`]
module"]
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
#[doc = "PWM Interrupt Enable Register 1"]
pub mod ier1;
#[doc = "IDR1 (w) register accessor: PWM Interrupt Disable Register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idr1`]
module"]
pub type IDR1 = crate::Reg<idr1::IDR1_SPEC>;
#[doc = "PWM Interrupt Disable Register 1"]
pub mod idr1;
#[doc = "IMR1 (r) register accessor: PWM Interrupt Mask Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`imr1`]
module"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "PWM Interrupt Mask Register 1"]
pub mod imr1;
#[doc = "ISR1 (r) register accessor: PWM Interrupt Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isr1`]
module"]
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
#[doc = "PWM Interrupt Status Register 1"]
pub mod isr1;
#[doc = "SCM (rw) register accessor: PWM Sync Channels Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scm`]
module"]
pub type SCM = crate::Reg<scm::SCM_SPEC>;
#[doc = "PWM Sync Channels Mode Register"]
pub mod scm;
#[doc = "DMAR (w) register accessor: PWM DMA Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmar`]
module"]
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
#[doc = "PWM DMA Register"]
pub mod dmar;
#[doc = "SCUC (rw) register accessor: PWM Sync Channels Update Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scuc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scuc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scuc`]
module"]
pub type SCUC = crate::Reg<scuc::SCUC_SPEC>;
#[doc = "PWM Sync Channels Update Control Register"]
pub mod scuc;
#[doc = "SCUP (rw) register accessor: PWM Sync Channels Update Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scup`]
module"]
pub type SCUP = crate::Reg<scup::SCUP_SPEC>;
#[doc = "PWM Sync Channels Update Period Register"]
pub mod scup;
#[doc = "SCUPUPD (w) register accessor: PWM Sync Channels Update Period Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scupupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scupupd`]
module"]
pub type SCUPUPD = crate::Reg<scupupd::SCUPUPD_SPEC>;
#[doc = "PWM Sync Channels Update Period Update Register"]
pub mod scupupd;
#[doc = "IER2 (w) register accessor: PWM Interrupt Enable Register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ier2`]
module"]
pub type IER2 = crate::Reg<ier2::IER2_SPEC>;
#[doc = "PWM Interrupt Enable Register 2"]
pub mod ier2;
#[doc = "IDR2 (w) register accessor: PWM Interrupt Disable Register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idr2`]
module"]
pub type IDR2 = crate::Reg<idr2::IDR2_SPEC>;
#[doc = "PWM Interrupt Disable Register 2"]
pub mod idr2;
#[doc = "IMR2 (r) register accessor: PWM Interrupt Mask Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`imr2`]
module"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "PWM Interrupt Mask Register 2"]
pub mod imr2;
#[doc = "ISR2 (r) register accessor: PWM Interrupt Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`isr2`]
module"]
pub type ISR2 = crate::Reg<isr2::ISR2_SPEC>;
#[doc = "PWM Interrupt Status Register 2"]
pub mod isr2;
#[doc = "OOV (rw) register accessor: PWM Output Override Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oov::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oov::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`oov`]
module"]
pub type OOV = crate::Reg<oov::OOV_SPEC>;
#[doc = "PWM Output Override Value Register"]
pub mod oov;
#[doc = "OS (rw) register accessor: PWM Output Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`os::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`os::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`os`]
module"]
pub type OS = crate::Reg<os::OS_SPEC>;
#[doc = "PWM Output Selection Register"]
pub mod os;
#[doc = "OSS (w) register accessor: PWM Output Selection Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oss::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`oss`]
module"]
pub type OSS = crate::Reg<oss::OSS_SPEC>;
#[doc = "PWM Output Selection Set Register"]
pub mod oss;
#[doc = "OSC (w) register accessor: PWM Output Selection Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`osc`]
module"]
pub type OSC = crate::Reg<osc::OSC_SPEC>;
#[doc = "PWM Output Selection Clear Register"]
pub mod osc;
#[doc = "OSSUPD (w) register accessor: PWM Output Selection Set Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ossupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ossupd`]
module"]
pub type OSSUPD = crate::Reg<ossupd::OSSUPD_SPEC>;
#[doc = "PWM Output Selection Set Update Register"]
pub mod ossupd;
#[doc = "OSCUPD (w) register accessor: PWM Output Selection Clear Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`oscupd`]
module"]
pub type OSCUPD = crate::Reg<oscupd::OSCUPD_SPEC>;
#[doc = "PWM Output Selection Clear Update Register"]
pub mod oscupd;
#[doc = "FMR (rw) register accessor: PWM Fault Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fmr`]
module"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "PWM Fault Mode Register"]
pub mod fmr;
#[doc = "FSR (r) register accessor: PWM Fault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fsr`]
module"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "PWM Fault Status Register"]
pub mod fsr;
#[doc = "FCR (w) register accessor: PWM Fault Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "PWM Fault Clear Register"]
pub mod fcr;
#[doc = "FPV1 (rw) register accessor: PWM Fault Protection Value Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fpv1`]
module"]
pub type FPV1 = crate::Reg<fpv1::FPV1_SPEC>;
#[doc = "PWM Fault Protection Value Register 1"]
pub mod fpv1;
#[doc = "FPE (rw) register accessor: PWM Fault Protection Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fpe`]
module"]
pub type FPE = crate::Reg<fpe::FPE_SPEC>;
#[doc = "PWM Fault Protection Enable Register"]
pub mod fpe;
#[doc = "ELMR (rw) register accessor: PWM Event Line 0 Mode Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`elmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`elmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`elmr`]
module"]
pub type ELMR = crate::Reg<elmr::ELMR_SPEC>;
#[doc = "PWM Event Line 0 Mode Register 0"]
pub mod elmr;
#[doc = "SSPR (rw) register accessor: PWM Spread Spectrum Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sspr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sspr`]
module"]
pub type SSPR = crate::Reg<sspr::SSPR_SPEC>;
#[doc = "PWM Spread Spectrum Register"]
pub mod sspr;
#[doc = "SSPUP (w) register accessor: PWM Spread Spectrum Update Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspup::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sspup`]
module"]
pub type SSPUP = crate::Reg<sspup::SSPUP_SPEC>;
#[doc = "PWM Spread Spectrum Update Register"]
pub mod sspup;
#[doc = "SMMR (rw) register accessor: PWM Stepper Motor Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`smmr`]
module"]
pub type SMMR = crate::Reg<smmr::SMMR_SPEC>;
#[doc = "PWM Stepper Motor Mode Register"]
pub mod smmr;
#[doc = "FPV2 (rw) register accessor: PWM Fault Protection Value 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpv2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpv2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fpv2`]
module"]
pub type FPV2 = crate::Reg<fpv2::FPV2_SPEC>;
#[doc = "PWM Fault Protection Value 2 Register"]
pub mod fpv2;
#[doc = "WPCR (w) register accessor: PWM Write Protection Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpcr`]
module"]
pub type WPCR = crate::Reg<wpcr::WPCR_SPEC>;
#[doc = "PWM Write Protection Control Register"]
pub mod wpcr;
#[doc = "WPSR (r) register accessor: PWM Write Protection Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpsr`]
module"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "PWM Write Protection Status Register"]
pub mod wpsr;
#[doc = "PWM Comparison 0 Value Register"]
pub use self::pwm_cmp::PWM_CMP;
#[doc = r"Cluster"]
#[doc = "PWM Comparison 0 Value Register"]
pub mod pwm_cmp;
#[doc = "PWM Channel Mode Register"]
pub use self::pwm_ch_num::PWM_CH_NUM;
#[doc = r"Cluster"]
#[doc = "PWM Channel Mode Register"]
pub mod pwm_ch_num;
#[doc = "CMUPD0 (w) register accessor: PWM Channel Mode Update Register (ch_num = 0)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmupd0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmupd0`]
module"]
pub type CMUPD0 = crate::Reg<cmupd0::CMUPD0_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 0)"]
pub mod cmupd0;
#[doc = "CMUPD1 (w) register accessor: PWM Channel Mode Update Register (ch_num = 1)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmupd1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmupd1`]
module"]
pub type CMUPD1 = crate::Reg<cmupd1::CMUPD1_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 1)"]
pub mod cmupd1;
#[doc = "ETRG1 (rw) register accessor: PWM External Trigger Register (trg_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etrg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etrg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`etrg1`]
module"]
pub type ETRG1 = crate::Reg<etrg1::ETRG1_SPEC>;
#[doc = "PWM External Trigger Register (trg_num = 1)"]
pub mod etrg1;
#[doc = "LEBR1 (rw) register accessor: PWM Leading-Edge Blanking Register (trg_num = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lebr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lebr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lebr1`]
module"]
pub type LEBR1 = crate::Reg<lebr1::LEBR1_SPEC>;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 1)"]
pub mod lebr1;
#[doc = "CMUPD2 (w) register accessor: PWM Channel Mode Update Register (ch_num = 2)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmupd2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmupd2`]
module"]
pub type CMUPD2 = crate::Reg<cmupd2::CMUPD2_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 2)"]
pub mod cmupd2;
#[doc = "ETRG2 (rw) register accessor: PWM External Trigger Register (trg_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etrg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etrg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`etrg2`]
module"]
pub type ETRG2 = crate::Reg<etrg2::ETRG2_SPEC>;
#[doc = "PWM External Trigger Register (trg_num = 2)"]
pub mod etrg2;
#[doc = "LEBR2 (rw) register accessor: PWM Leading-Edge Blanking Register (trg_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lebr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lebr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lebr2`]
module"]
pub type LEBR2 = crate::Reg<lebr2::LEBR2_SPEC>;
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)"]
pub mod lebr2;
#[doc = "CMUPD3 (w) register accessor: PWM Channel Mode Update Register (ch_num = 3)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmupd3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmupd3`]
module"]
pub type CMUPD3 = crate::Reg<cmupd3::CMUPD3_SPEC>;
#[doc = "PWM Channel Mode Update Register (ch_num = 3)"]
pub mod cmupd3;
