#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Time Register"]
    pub timr: TIMR,
    #[doc = "0x0c - Calendar Register"]
    pub calr: CALR,
    #[doc = "0x10 - Time Alarm Register"]
    pub timalr: TIMALR,
    #[doc = "0x14 - Calendar Alarm Register"]
    pub calalr: CALALR,
    #[doc = "0x18 - Status Register"]
    pub sr: SR,
    #[doc = "0x1c - Status Clear Command Register"]
    pub sccr: SCCR,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x24 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x28 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x2c - Valid Entry Register"]
    pub ver: VER,
}
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "TIMR (rw) register accessor: Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timr`]
module"]
pub type TIMR = crate::Reg<timr::TIMR_SPEC>;
#[doc = "Time Register"]
pub mod timr;
#[doc = "CALR (rw) register accessor: Calendar Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`calr`]
module"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "Calendar Register"]
pub mod calr;
#[doc = "TIMALR (rw) register accessor: Time Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timalr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timalr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timalr`]
module"]
pub type TIMALR = crate::Reg<timalr::TIMALR_SPEC>;
#[doc = "Time Alarm Register"]
pub mod timalr;
#[doc = "CALALR (rw) register accessor: Calendar Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calalr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calalr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`calalr`]
module"]
pub type CALALR = crate::Reg<calalr::CALALR_SPEC>;
#[doc = "Calendar Alarm Register"]
pub mod calalr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "SCCR (w) register accessor: Status Clear Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sccr`]
module"]
pub type SCCR = crate::Reg<sccr::SCCR_SPEC>;
#[doc = "Status Clear Command Register"]
pub mod sccr;
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
#[doc = "VER (r) register accessor: Valid Entry Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ver`]
module"]
pub type VER = crate::Reg<ver::VER_SPEC>;
#[doc = "Valid Entry Register"]
pub mod ver;
