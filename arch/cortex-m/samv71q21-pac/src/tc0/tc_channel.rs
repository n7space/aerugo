#[doc = r"Register block"]
#[repr(C)]
pub struct TC_CHANNEL {
    #[doc = "0x00 - Channel Control Register (channel = 0)"]
    pub ccr: CCR,
    _reserved_1_cmr: [u8; 0x04],
    #[doc = "0x08 - Stepper Motor Mode Register (channel = 0)"]
    pub smmr: SMMR,
    #[doc = "0x0c - Register AB (channel = 0)"]
    pub rab: RAB,
    #[doc = "0x10 - Counter Value (channel = 0)"]
    pub cv: CV,
    #[doc = "0x14 - Register A (channel = 0)"]
    pub ra: RA,
    #[doc = "0x18 - Register B (channel = 0)"]
    pub rb: RB,
    #[doc = "0x1c - Register C (channel = 0)"]
    pub rc: RC,
    #[doc = "0x20 - Status Register (channel = 0)"]
    pub sr: SR,
    #[doc = "0x24 - Interrupt Enable Register (channel = 0)"]
    pub ier: IER,
    #[doc = "0x28 - Interrupt Disable Register (channel = 0)"]
    pub idr: IDR,
    #[doc = "0x2c - Interrupt Mask Register (channel = 0)"]
    pub imr: IMR,
    #[doc = "0x30 - Extended Mode Register (channel = 0)"]
    pub emr: EMR,
}
impl TC_CHANNEL {
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn cmr_waveform_mode(&self) -> &CMR_WAVEFORM_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Channel Mode Register (channel = 0)"]
    #[inline(always)]
    pub const fn cmr_capture_mode(&self) -> &CMR_CAPTURE_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "CCR (w) register accessor: Channel Control Register (channel = 0)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Channel Control Register (channel = 0)"]
pub mod ccr;
#[doc = "CMR_CAPTURE_MODE (rw) register accessor: Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr_capture_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr_capture_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmr_capture_mode`]
module"]
pub type CMR_CAPTURE_MODE = crate::Reg<cmr_capture_mode::CMR_CAPTURE_MODE_SPEC>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr_capture_mode;
#[doc = "CMR_WAVEFORM_MODE (rw) register accessor: Channel Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr_waveform_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr_waveform_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmr_waveform_mode`]
module"]
pub type CMR_WAVEFORM_MODE = crate::Reg<cmr_waveform_mode::CMR_WAVEFORM_MODE_SPEC>;
#[doc = "Channel Mode Register (channel = 0)"]
pub mod cmr_waveform_mode;
#[doc = "SMMR (rw) register accessor: Stepper Motor Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`smmr`]
module"]
pub type SMMR = crate::Reg<smmr::SMMR_SPEC>;
#[doc = "Stepper Motor Mode Register (channel = 0)"]
pub mod smmr;
#[doc = "RAB (r) register accessor: Register AB (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rab`]
module"]
pub type RAB = crate::Reg<rab::RAB_SPEC>;
#[doc = "Register AB (channel = 0)"]
pub mod rab;
#[doc = "CV (r) register accessor: Counter Value (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cv`]
module"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Counter Value (channel = 0)"]
pub mod cv;
#[doc = "RA (rw) register accessor: Register A (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ra`]
module"]
pub type RA = crate::Reg<ra::RA_SPEC>;
#[doc = "Register A (channel = 0)"]
pub mod ra;
#[doc = "RB (rw) register accessor: Register B (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rb`]
module"]
pub type RB = crate::Reg<rb::RB_SPEC>;
#[doc = "Register B (channel = 0)"]
pub mod rb;
#[doc = "RC (rw) register accessor: Register C (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rc`]
module"]
pub type RC = crate::Reg<rc::RC_SPEC>;
#[doc = "Register C (channel = 0)"]
pub mod rc;
#[doc = "SR (r) register accessor: Status Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register (channel = 0)"]
pub mod sr;
#[doc = "IER (w) register accessor: Interrupt Enable Register (channel = 0)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register (channel = 0)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod imr;
#[doc = "EMR (rw) register accessor: Extended Mode Register (channel = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emr`]
module"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "Extended Mode Register (channel = 0)"]
pub mod emr;
