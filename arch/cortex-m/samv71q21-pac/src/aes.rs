#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x20..0x40 - Key Word Register"]
    pub keywr: [KEYWR; 8],
    #[doc = "0x40..0x50 - Input Data Register"]
    pub idatar: [IDATAR; 4],
    #[doc = "0x50..0x60 - Output Data Register"]
    pub odatar: [ODATAR; 4],
    #[doc = "0x60..0x70 - Initialization Vector Register"]
    pub ivr: [IVR; 4],
    #[doc = "0x70 - Additional Authenticated Data Length Register"]
    pub aadlenr: AADLENR,
    #[doc = "0x74 - Plaintext/Ciphertext Length Register"]
    pub clenr: CLENR,
    #[doc = "0x78..0x88 - GCM Intermediate Hash Word Register"]
    pub ghashr: [GHASHR; 4],
    #[doc = "0x88..0x98 - GCM Authentication Tag Word Register"]
    pub tagr: [TAGR; 4],
    #[doc = "0x98 - GCM Encryption Counter Value Register"]
    pub ctrr: CTRR,
    #[doc = "0x9c..0xac - GCM H Word Register"]
    pub gcmhr: [GCMHR; 4],
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mr`]
module"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
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
#[doc = "KEYWR (w) register accessor: Key Word Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keywr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`keywr`]
module"]
pub type KEYWR = crate::Reg<keywr::KEYWR_SPEC>;
#[doc = "Key Word Register"]
pub mod keywr;
#[doc = "IDATAR (w) register accessor: Input Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idatar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idatar`]
module"]
pub type IDATAR = crate::Reg<idatar::IDATAR_SPEC>;
#[doc = "Input Data Register"]
pub mod idatar;
#[doc = "ODATAR (r) register accessor: Output Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`odatar`]
module"]
pub type ODATAR = crate::Reg<odatar::ODATAR_SPEC>;
#[doc = "Output Data Register"]
pub mod odatar;
#[doc = "IVR (w) register accessor: Initialization Vector Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ivr`]
module"]
pub type IVR = crate::Reg<ivr::IVR_SPEC>;
#[doc = "Initialization Vector Register"]
pub mod ivr;
#[doc = "AADLENR (rw) register accessor: Additional Authenticated Data Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aadlenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aadlenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aadlenr`]
module"]
pub type AADLENR = crate::Reg<aadlenr::AADLENR_SPEC>;
#[doc = "Additional Authenticated Data Length Register"]
pub mod aadlenr;
#[doc = "CLENR (rw) register accessor: Plaintext/Ciphertext Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clenr`]
module"]
pub type CLENR = crate::Reg<clenr::CLENR_SPEC>;
#[doc = "Plaintext/Ciphertext Length Register"]
pub mod clenr;
#[doc = "GHASHR (rw) register accessor: GCM Intermediate Hash Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghashr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ghashr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ghashr`]
module"]
pub type GHASHR = crate::Reg<ghashr::GHASHR_SPEC>;
#[doc = "GCM Intermediate Hash Word Register"]
pub mod ghashr;
#[doc = "TAGR (r) register accessor: GCM Authentication Tag Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tagr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tagr`]
module"]
pub type TAGR = crate::Reg<tagr::TAGR_SPEC>;
#[doc = "GCM Authentication Tag Word Register"]
pub mod tagr;
#[doc = "CTRR (r) register accessor: GCM Encryption Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrr`]
module"]
pub type CTRR = crate::Reg<ctrr::CTRR_SPEC>;
#[doc = "GCM Encryption Counter Value Register"]
pub mod ctrr;
#[doc = "GCMHR (rw) register accessor: GCM H Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcmhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcmhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gcmhr`]
module"]
pub type GCMHR = crate::Reg<gcmhr::GCMHR_SPEC>;
#[doc = "GCM H Word Register"]
pub mod gcmhr;
