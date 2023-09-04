#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Clock Mode Register"]
    pub cmr: CMR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Receive Clock Mode Register"]
    pub rcmr: RCMR,
    #[doc = "0x14 - Receive Frame Mode Register"]
    pub rfmr: RFMR,
    #[doc = "0x18 - Transmit Clock Mode Register"]
    pub tcmr: TCMR,
    #[doc = "0x1c - Transmit Frame Mode Register"]
    pub tfmr: TFMR,
    #[doc = "0x20 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x24 - Transmit Holding Register"]
    pub thr: THR,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - Receive Sync. Holding Register"]
    pub rshr: RSHR,
    #[doc = "0x34 - Transmit Sync. Holding Register"]
    pub tshr: TSHR,
    #[doc = "0x38 - Receive Compare 0 Register"]
    pub rc0r: RC0R,
    #[doc = "0x3c - Receive Compare 1 Register"]
    pub rc1r: RC1R,
    #[doc = "0x40 - Status Register"]
    pub sr: SR,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub imr: IMR,
    _reserved16: [u8; 0x94],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CMR (rw) register accessor: Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmr`]
module"]
pub type CMR = crate::Reg<cmr::CMR_SPEC>;
#[doc = "Clock Mode Register"]
pub mod cmr;
#[doc = "RCMR (rw) register accessor: Receive Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rcmr`]
module"]
pub type RCMR = crate::Reg<rcmr::RCMR_SPEC>;
#[doc = "Receive Clock Mode Register"]
pub mod rcmr;
#[doc = "RFMR (rw) register accessor: Receive Frame Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfmr`]
module"]
pub type RFMR = crate::Reg<rfmr::RFMR_SPEC>;
#[doc = "Receive Frame Mode Register"]
pub mod rfmr;
#[doc = "TCMR (rw) register accessor: Transmit Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tcmr`]
module"]
pub type TCMR = crate::Reg<tcmr::TCMR_SPEC>;
#[doc = "Transmit Clock Mode Register"]
pub mod tcmr;
#[doc = "TFMR (rw) register accessor: Transmit Frame Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tfmr`]
module"]
pub type TFMR = crate::Reg<tfmr::TFMR_SPEC>;
#[doc = "Transmit Frame Mode Register"]
pub mod tfmr;
#[doc = "RHR (r) register accessor: Receive Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rhr`]
module"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "THR (w) register accessor: Transmit Holding Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`thr`]
module"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "RSHR (r) register accessor: Receive Sync. Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rshr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rshr`]
module"]
pub type RSHR = crate::Reg<rshr::RSHR_SPEC>;
#[doc = "Receive Sync. Holding Register"]
pub mod rshr;
#[doc = "TSHR (rw) register accessor: Transmit Sync. Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tshr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tshr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tshr`]
module"]
pub type TSHR = crate::Reg<tshr::TSHR_SPEC>;
#[doc = "Transmit Sync. Holding Register"]
pub mod tshr;
#[doc = "RC0R (rw) register accessor: Receive Compare 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rc0r`]
module"]
pub type RC0R = crate::Reg<rc0r::RC0R_SPEC>;
#[doc = "Receive Compare 0 Register"]
pub mod rc0r;
#[doc = "RC1R (rw) register accessor: Receive Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rc1r`]
module"]
pub type RC1R = crate::Reg<rc1r::RC1R_SPEC>;
#[doc = "Receive Compare 1 Register"]
pub mod rc1r;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
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
