#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub devctrl: DEVCTRL,
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    pub devisr: DEVISR,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub devicr: DEVICR,
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    pub devifr: DEVIFR,
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    pub devimr: DEVIMR,
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    pub devidr: DEVIDR,
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    pub devier: DEVIER,
    #[doc = "0x1c - Device Endpoint Register"]
    pub devept: DEVEPT,
    #[doc = "0x20 - Device Frame Number Register"]
    pub devfnum: DEVFNUM,
    _reserved9: [u8; 0xdc],
    #[doc = "0x100..0x128 - Device Endpoint Configuration Register"]
    pub deveptcfg: [DEVEPTCFG; 10],
    _reserved10: [u8; 0x08],
    _reserved_10_deveptisr: [u8; 0x28],
    _reserved11: [u8; 0x08],
    _reserved_11_devepticr: [u8; 0x28],
    _reserved12: [u8; 0x08],
    _reserved_12_deveptifr: [u8; 0x28],
    _reserved13: [u8; 0x08],
    _reserved_13_deveptimr: [u8; 0x28],
    _reserved14: [u8; 0x08],
    _reserved_14_deveptier: [u8; 0x28],
    _reserved15: [u8; 0x08],
    _reserved_15_deveptidr: [u8; 0x28],
    _reserved16: [u8; 0xc8],
    #[doc = "0x310..0x380 - Device DMA Channel Next Descriptor Address Register"]
    pub usbhs_devdma: [USBHS_DEVDMA; 7],
    _reserved17: [u8; 0x80],
    #[doc = "0x400 - Host General Control Register"]
    pub hstctrl: HSTCTRL,
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    pub hstisr: HSTISR,
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    pub hsticr: HSTICR,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub hstifr: HSTIFR,
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    pub hstimr: HSTIMR,
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    pub hstidr: HSTIDR,
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    pub hstier: HSTIER,
    #[doc = "0x41c - Host Pipe Register"]
    pub hstpip: HSTPIP,
    #[doc = "0x420 - Host Frame Number Register"]
    pub hstfnum: HSTFNUM,
    #[doc = "0x424 - Host Address 1 Register"]
    pub hstaddr1: HSTADDR1,
    #[doc = "0x428 - Host Address 2 Register"]
    pub hstaddr2: HSTADDR2,
    #[doc = "0x42c - Host Address 3 Register"]
    pub hstaddr3: HSTADDR3,
    _reserved29: [u8; 0xd0],
    _reserved_29_hstpipcfg: [u8; 0x28],
    _reserved30: [u8; 0x08],
    _reserved_30_hstpipisr: [u8; 0x28],
    _reserved31: [u8; 0x08],
    _reserved_31_hstpipicr: [u8; 0x28],
    _reserved32: [u8; 0x08],
    _reserved_32_hstpipifr: [u8; 0x28],
    _reserved33: [u8; 0x08],
    _reserved_33_hstpipimr: [u8; 0x28],
    _reserved34: [u8; 0x08],
    _reserved_34_hstpipier: [u8; 0x28],
    _reserved35: [u8; 0x08],
    _reserved_35_hstpipidr: [u8; 0x28],
    _reserved36: [u8; 0x08],
    #[doc = "0x650..0x678 - Host Pipe IN Request Register"]
    pub hstpipinrq: [HSTPIPINRQ; 10],
    _reserved37: [u8; 0x08],
    #[doc = "0x680..0x6a8 - Host Pipe Error Register"]
    pub hstpiperr: [HSTPIPERR; 10],
    _reserved38: [u8; 0x68],
    #[doc = "0x710..0x780 - Host DMA Channel Next Descriptor Address Register"]
    pub usbhs_hstdma: [USBHS_HSTDMA; 7],
    _reserved39: [u8; 0x80],
    #[doc = "0x800 - General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x804 - General Status Register"]
    pub sr: SR,
    #[doc = "0x808 - General Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x80c - General Status Set Register"]
    pub sfr: SFR,
}
impl RegisterBlock {
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub const fn deveptisr_intrpt_mode(&self) -> &[DEVEPTISR_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(304usize).cast() }
    }
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub const fn deveptisr_blk_mode(&self) -> &[DEVEPTISR_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(304usize).cast() }
    }
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub const fn deveptisr_iso_mode(&self) -> &[DEVEPTISR_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(304usize).cast() }
    }
    #[doc = "0x130..0x158 - Device Endpoint Interrupt Status Register"]
    #[inline(always)]
    pub const fn deveptisr_ctrl_mode(&self) -> &[DEVEPTISR_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(304usize).cast() }
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub const fn devepticr_intrpt_mode(&self) -> &[DEVEPTICR_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(352usize).cast() }
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub const fn devepticr_blk_mode(&self) -> &[DEVEPTICR_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(352usize).cast() }
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub const fn devepticr_iso_mode(&self) -> &[DEVEPTICR_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(352usize).cast() }
    }
    #[doc = "0x160..0x188 - Device Endpoint Interrupt Clear Register"]
    #[inline(always)]
    pub const fn devepticr_ctrl_mode(&self) -> &[DEVEPTICR_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(352usize).cast() }
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub const fn deveptifr_intrpt_mode(&self) -> &[DEVEPTIFR_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(400usize).cast() }
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub const fn deveptifr_blk_mode(&self) -> &[DEVEPTIFR_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(400usize).cast() }
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub const fn deveptifr_iso_mode(&self) -> &[DEVEPTIFR_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(400usize).cast() }
    }
    #[doc = "0x190..0x1b8 - Device Endpoint Interrupt Set Register"]
    #[inline(always)]
    pub const fn deveptifr_ctrl_mode(&self) -> &[DEVEPTIFR_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(400usize).cast() }
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub const fn deveptimr_intrpt_mode(&self) -> &[DEVEPTIMR_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(448usize).cast() }
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub const fn deveptimr_blk_mode(&self) -> &[DEVEPTIMR_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(448usize).cast() }
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub const fn deveptimr_iso_mode(&self) -> &[DEVEPTIMR_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(448usize).cast() }
    }
    #[doc = "0x1c0..0x1e8 - Device Endpoint Interrupt Mask Register"]
    #[inline(always)]
    pub const fn deveptimr_ctrl_mode(&self) -> &[DEVEPTIMR_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(448usize).cast() }
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub const fn deveptier_intrpt_mode(&self) -> &[DEVEPTIER_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(496usize).cast() }
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub const fn deveptier_blk_mode(&self) -> &[DEVEPTIER_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(496usize).cast() }
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub const fn deveptier_iso_mode(&self) -> &[DEVEPTIER_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(496usize).cast() }
    }
    #[doc = "0x1f0..0x218 - Device Endpoint Interrupt Enable Register"]
    #[inline(always)]
    pub const fn deveptier_ctrl_mode(&self) -> &[DEVEPTIER_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(496usize).cast() }
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub const fn deveptidr_intrpt_mode(&self) -> &[DEVEPTIDR_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(544usize).cast() }
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub const fn deveptidr_blk_mode(&self) -> &[DEVEPTIDR_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(544usize).cast() }
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub const fn deveptidr_iso_mode(&self) -> &[DEVEPTIDR_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(544usize).cast() }
    }
    #[doc = "0x220..0x248 - Device Endpoint Interrupt Disable Register"]
    #[inline(always)]
    pub const fn deveptidr_ctrl_mode(&self) -> &[DEVEPTIDR_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(544usize).cast() }
    }
    #[doc = "0x500..0x528 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub const fn hstpipcfg_ctrl_bulk_mode(&self) -> &[HSTPIPCFG_CTRL_BULK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1280usize).cast() }
    }
    #[doc = "0x500..0x528 - Host Pipe Configuration Register"]
    #[inline(always)]
    pub const fn hstpipcfg(&self) -> &[HSTPIPCFG; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1280usize).cast() }
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub const fn hstpipisr_intrpt_mode(&self) -> &[HSTPIPISR_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1328usize).cast() }
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub const fn hstpipisr_blk_mode(&self) -> &[HSTPIPISR_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1328usize).cast() }
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub const fn hstpipisr_iso_mode(&self) -> &[HSTPIPISR_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1328usize).cast() }
    }
    #[doc = "0x530..0x558 - Host Pipe Status Register"]
    #[inline(always)]
    pub const fn hstpipisr_ctrl_mode(&self) -> &[HSTPIPISR_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1328usize).cast() }
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub const fn hstpipicr_intrpt_mode(&self) -> &[HSTPIPICR_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376usize).cast() }
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub const fn hstpipicr_blk_mode(&self) -> &[HSTPIPICR_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376usize).cast() }
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub const fn hstpipicr_iso_mode(&self) -> &[HSTPIPICR_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376usize).cast() }
    }
    #[doc = "0x560..0x588 - Host Pipe Clear Register"]
    #[inline(always)]
    pub const fn hstpipicr_ctrl_mode(&self) -> &[HSTPIPICR_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376usize).cast() }
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub const fn hstpipifr_intrpt_mode(&self) -> &[HSTPIPIFR_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1424usize).cast() }
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub const fn hstpipifr_blk_mode(&self) -> &[HSTPIPIFR_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1424usize).cast() }
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub const fn hstpipifr_iso_mode(&self) -> &[HSTPIPIFR_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1424usize).cast() }
    }
    #[doc = "0x590..0x5b8 - Host Pipe Set Register"]
    #[inline(always)]
    pub const fn hstpipifr_ctrl_mode(&self) -> &[HSTPIPIFR_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1424usize).cast() }
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub const fn hstpipimr_intrpt_mode(&self) -> &[HSTPIPIMR_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1472usize).cast() }
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub const fn hstpipimr_blk_mode(&self) -> &[HSTPIPIMR_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1472usize).cast() }
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub const fn hstpipimr_iso_mode(&self) -> &[HSTPIPIMR_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1472usize).cast() }
    }
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register"]
    #[inline(always)]
    pub const fn hstpipimr_ctrl_mode(&self) -> &[HSTPIPIMR_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1472usize).cast() }
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub const fn hstpipier_intrpt_mode(&self) -> &[HSTPIPIER_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1520usize).cast() }
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub const fn hstpipier_blk_mode(&self) -> &[HSTPIPIER_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1520usize).cast() }
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub const fn hstpipier_iso_mode(&self) -> &[HSTPIPIER_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1520usize).cast() }
    }
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register"]
    #[inline(always)]
    pub const fn hstpipier_ctrl_mode(&self) -> &[HSTPIPIER_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1520usize).cast() }
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub const fn hstpipidr_intrpt_mode(&self) -> &[HSTPIPIDR_INTRPT_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1568usize).cast() }
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub const fn hstpipidr_blk_mode(&self) -> &[HSTPIPIDR_BLK_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1568usize).cast() }
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub const fn hstpipidr_iso_mode(&self) -> &[HSTPIPIDR_ISO_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1568usize).cast() }
    }
    #[doc = "0x620..0x648 - Host Pipe Disable Register"]
    #[inline(always)]
    pub const fn hstpipidr_ctrl_mode(&self) -> &[HSTPIPIDR_CTRL_MODE; 10] {
        unsafe { &*(self as *const Self).cast::<u8>().add(1568usize).cast() }
    }
}
#[doc = "DEVCTRL (rw) register accessor: Device General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devctrl`]
module"]
pub type DEVCTRL = crate::Reg<devctrl::DEVCTRL_SPEC>;
#[doc = "Device General Control Register"]
pub mod devctrl;
#[doc = "DEVISR (r) register accessor: Device Global Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devisr`]
module"]
pub type DEVISR = crate::Reg<devisr::DEVISR_SPEC>;
#[doc = "Device Global Interrupt Status Register"]
pub mod devisr;
#[doc = "DEVICR (w) register accessor: Device Global Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devicr`]
module"]
pub type DEVICR = crate::Reg<devicr::DEVICR_SPEC>;
#[doc = "Device Global Interrupt Clear Register"]
pub mod devicr;
#[doc = "DEVIFR (w) register accessor: Device Global Interrupt Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devifr`]
module"]
pub type DEVIFR = crate::Reg<devifr::DEVIFR_SPEC>;
#[doc = "Device Global Interrupt Set Register"]
pub mod devifr;
#[doc = "DEVIMR (r) register accessor: Device Global Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devimr`]
module"]
pub type DEVIMR = crate::Reg<devimr::DEVIMR_SPEC>;
#[doc = "Device Global Interrupt Mask Register"]
pub mod devimr;
#[doc = "DEVIDR (w) register accessor: Device Global Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devidr`]
module"]
pub type DEVIDR = crate::Reg<devidr::DEVIDR_SPEC>;
#[doc = "Device Global Interrupt Disable Register"]
pub mod devidr;
#[doc = "DEVIER (w) register accessor: Device Global Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devier`]
module"]
pub type DEVIER = crate::Reg<devier::DEVIER_SPEC>;
#[doc = "Device Global Interrupt Enable Register"]
pub mod devier;
#[doc = "DEVEPT (rw) register accessor: Device Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devept::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devept::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devept`]
module"]
pub type DEVEPT = crate::Reg<devept::DEVEPT_SPEC>;
#[doc = "Device Endpoint Register"]
pub mod devept;
#[doc = "DEVFNUM (r) register accessor: Device Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devfnum`]
module"]
pub type DEVFNUM = crate::Reg<devfnum::DEVFNUM_SPEC>;
#[doc = "Device Frame Number Register"]
pub mod devfnum;
#[doc = "DEVEPTCFG (rw) register accessor: Device Endpoint Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptcfg`]
module"]
pub type DEVEPTCFG = crate::Reg<deveptcfg::DEVEPTCFG_SPEC>;
#[doc = "Device Endpoint Configuration Register"]
pub mod deveptcfg;
#[doc = "DEVEPTISR_CTRL_MODE (r) register accessor: Device Endpoint Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr_ctrl_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr_ctrl_mode`]
module"]
pub type DEVEPTISR_CTRL_MODE = crate::Reg<deveptisr_ctrl_mode::DEVEPTISR_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_ctrl_mode;
#[doc = "DEVEPTISR_ISO_MODE (r) register accessor: Device Endpoint Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr_iso_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr_iso_mode`]
module"]
pub type DEVEPTISR_ISO_MODE = crate::Reg<deveptisr_iso_mode::DEVEPTISR_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_iso_mode;
#[doc = "DEVEPTISR_BLK_MODE (r) register accessor: Device Endpoint Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr_blk_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr_blk_mode`]
module"]
pub type DEVEPTISR_BLK_MODE = crate::Reg<deveptisr_blk_mode::DEVEPTISR_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_blk_mode;
#[doc = "DEVEPTISR_INTRPT_MODE (r) register accessor: Device Endpoint Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr_intrpt_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptisr_intrpt_mode`]
module"]
pub type DEVEPTISR_INTRPT_MODE = crate::Reg<deveptisr_intrpt_mode::DEVEPTISR_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Status Register"]
pub mod deveptisr_intrpt_mode;
#[doc = "DEVEPTICR_CTRL_MODE (w) register accessor: Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr_ctrl_mode`]
module"]
pub type DEVEPTICR_CTRL_MODE = crate::Reg<devepticr_ctrl_mode::DEVEPTICR_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_ctrl_mode;
#[doc = "DEVEPTICR_ISO_MODE (w) register accessor: Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr_iso_mode`]
module"]
pub type DEVEPTICR_ISO_MODE = crate::Reg<devepticr_iso_mode::DEVEPTICR_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_iso_mode;
#[doc = "DEVEPTICR_BLK_MODE (w) register accessor: Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr_blk_mode`]
module"]
pub type DEVEPTICR_BLK_MODE = crate::Reg<devepticr_blk_mode::DEVEPTICR_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_blk_mode;
#[doc = "DEVEPTICR_INTRPT_MODE (w) register accessor: Device Endpoint Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devepticr_intrpt_mode`]
module"]
pub type DEVEPTICR_INTRPT_MODE = crate::Reg<devepticr_intrpt_mode::DEVEPTICR_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Clear Register"]
pub mod devepticr_intrpt_mode;
#[doc = "DEVEPTIFR_CTRL_MODE (w) register accessor: Device Endpoint Interrupt Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr_ctrl_mode`]
module"]
pub type DEVEPTIFR_CTRL_MODE = crate::Reg<deveptifr_ctrl_mode::DEVEPTIFR_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_ctrl_mode;
#[doc = "DEVEPTIFR_ISO_MODE (w) register accessor: Device Endpoint Interrupt Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr_iso_mode`]
module"]
pub type DEVEPTIFR_ISO_MODE = crate::Reg<deveptifr_iso_mode::DEVEPTIFR_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_iso_mode;
#[doc = "DEVEPTIFR_BLK_MODE (w) register accessor: Device Endpoint Interrupt Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr_blk_mode`]
module"]
pub type DEVEPTIFR_BLK_MODE = crate::Reg<deveptifr_blk_mode::DEVEPTIFR_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_blk_mode;
#[doc = "DEVEPTIFR_INTRPT_MODE (w) register accessor: Device Endpoint Interrupt Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptifr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptifr_intrpt_mode`]
module"]
pub type DEVEPTIFR_INTRPT_MODE = crate::Reg<deveptifr_intrpt_mode::DEVEPTIFR_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Set Register"]
pub mod deveptifr_intrpt_mode;
#[doc = "DEVEPTIMR_CTRL_MODE (r) register accessor: Device Endpoint Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr_ctrl_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr_ctrl_mode`]
module"]
pub type DEVEPTIMR_CTRL_MODE = crate::Reg<deveptimr_ctrl_mode::DEVEPTIMR_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_ctrl_mode;
#[doc = "DEVEPTIMR_ISO_MODE (r) register accessor: Device Endpoint Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr_iso_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr_iso_mode`]
module"]
pub type DEVEPTIMR_ISO_MODE = crate::Reg<deveptimr_iso_mode::DEVEPTIMR_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_iso_mode;
#[doc = "DEVEPTIMR_BLK_MODE (r) register accessor: Device Endpoint Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr_blk_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr_blk_mode`]
module"]
pub type DEVEPTIMR_BLK_MODE = crate::Reg<deveptimr_blk_mode::DEVEPTIMR_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_blk_mode;
#[doc = "DEVEPTIMR_INTRPT_MODE (r) register accessor: Device Endpoint Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptimr_intrpt_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptimr_intrpt_mode`]
module"]
pub type DEVEPTIMR_INTRPT_MODE = crate::Reg<deveptimr_intrpt_mode::DEVEPTIMR_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Mask Register"]
pub mod deveptimr_intrpt_mode;
#[doc = "DEVEPTIER_CTRL_MODE (w) register accessor: Device Endpoint Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier_ctrl_mode`]
module"]
pub type DEVEPTIER_CTRL_MODE = crate::Reg<deveptier_ctrl_mode::DEVEPTIER_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_ctrl_mode;
#[doc = "DEVEPTIER_ISO_MODE (w) register accessor: Device Endpoint Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier_iso_mode`]
module"]
pub type DEVEPTIER_ISO_MODE = crate::Reg<deveptier_iso_mode::DEVEPTIER_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_iso_mode;
#[doc = "DEVEPTIER_BLK_MODE (w) register accessor: Device Endpoint Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier_blk_mode`]
module"]
pub type DEVEPTIER_BLK_MODE = crate::Reg<deveptier_blk_mode::DEVEPTIER_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_blk_mode;
#[doc = "DEVEPTIER_INTRPT_MODE (w) register accessor: Device Endpoint Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptier_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptier_intrpt_mode`]
module"]
pub type DEVEPTIER_INTRPT_MODE = crate::Reg<deveptier_intrpt_mode::DEVEPTIER_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Enable Register"]
pub mod deveptier_intrpt_mode;
#[doc = "DEVEPTIDR_CTRL_MODE (w) register accessor: Device Endpoint Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr_ctrl_mode`]
module"]
pub type DEVEPTIDR_CTRL_MODE = crate::Reg<deveptidr_ctrl_mode::DEVEPTIDR_CTRL_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_ctrl_mode;
#[doc = "DEVEPTIDR_ISO_MODE (w) register accessor: Device Endpoint Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr_iso_mode`]
module"]
pub type DEVEPTIDR_ISO_MODE = crate::Reg<deveptidr_iso_mode::DEVEPTIDR_ISO_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_iso_mode;
#[doc = "DEVEPTIDR_BLK_MODE (w) register accessor: Device Endpoint Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr_blk_mode`]
module"]
pub type DEVEPTIDR_BLK_MODE = crate::Reg<deveptidr_blk_mode::DEVEPTIDR_BLK_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_blk_mode;
#[doc = "DEVEPTIDR_INTRPT_MODE (w) register accessor: Device Endpoint Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptidr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deveptidr_intrpt_mode`]
module"]
pub type DEVEPTIDR_INTRPT_MODE = crate::Reg<deveptidr_intrpt_mode::DEVEPTIDR_INTRPT_MODE_SPEC>;
#[doc = "Device Endpoint Interrupt Disable Register"]
pub mod deveptidr_intrpt_mode;
#[doc = "Device DMA Channel Next Descriptor Address Register"]
pub use self::usbhs_devdma::USBHS_DEVDMA;
#[doc = r"Cluster"]
#[doc = "Device DMA Channel Next Descriptor Address Register"]
pub mod usbhs_devdma;
#[doc = "HSTCTRL (rw) register accessor: Host General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstctrl`]
module"]
pub type HSTCTRL = crate::Reg<hstctrl::HSTCTRL_SPEC>;
#[doc = "Host General Control Register"]
pub mod hstctrl;
#[doc = "HSTISR (r) register accessor: Host Global Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstisr`]
module"]
pub type HSTISR = crate::Reg<hstisr::HSTISR_SPEC>;
#[doc = "Host Global Interrupt Status Register"]
pub mod hstisr;
#[doc = "HSTICR (w) register accessor: Host Global Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsticr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsticr`]
module"]
pub type HSTICR = crate::Reg<hsticr::HSTICR_SPEC>;
#[doc = "Host Global Interrupt Clear Register"]
pub mod hsticr;
#[doc = "HSTIFR (w) register accessor: Host Global Interrupt Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstifr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstifr`]
module"]
pub type HSTIFR = crate::Reg<hstifr::HSTIFR_SPEC>;
#[doc = "Host Global Interrupt Set Register"]
pub mod hstifr;
#[doc = "HSTIMR (r) register accessor: Host Global Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstimr`]
module"]
pub type HSTIMR = crate::Reg<hstimr::HSTIMR_SPEC>;
#[doc = "Host Global Interrupt Mask Register"]
pub mod hstimr;
#[doc = "HSTIDR (w) register accessor: Host Global Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstidr`]
module"]
pub type HSTIDR = crate::Reg<hstidr::HSTIDR_SPEC>;
#[doc = "Host Global Interrupt Disable Register"]
pub mod hstidr;
#[doc = "HSTIER (w) register accessor: Host Global Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstier`]
module"]
pub type HSTIER = crate::Reg<hstier::HSTIER_SPEC>;
#[doc = "Host Global Interrupt Enable Register"]
pub mod hstier;
#[doc = "HSTPIP (rw) register accessor: Host Pipe Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpip`]
module"]
pub type HSTPIP = crate::Reg<hstpip::HSTPIP_SPEC>;
#[doc = "Host Pipe Register"]
pub mod hstpip;
#[doc = "HSTFNUM (rw) register accessor: Host Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstfnum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstfnum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstfnum`]
module"]
pub type HSTFNUM = crate::Reg<hstfnum::HSTFNUM_SPEC>;
#[doc = "Host Frame Number Register"]
pub mod hstfnum;
#[doc = "HSTADDR1 (rw) register accessor: Host Address 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstaddr1`]
module"]
pub type HSTADDR1 = crate::Reg<hstaddr1::HSTADDR1_SPEC>;
#[doc = "Host Address 1 Register"]
pub mod hstaddr1;
#[doc = "HSTADDR2 (rw) register accessor: Host Address 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstaddr2`]
module"]
pub type HSTADDR2 = crate::Reg<hstaddr2::HSTADDR2_SPEC>;
#[doc = "Host Address 2 Register"]
pub mod hstaddr2;
#[doc = "HSTADDR3 (rw) register accessor: Host Address 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstaddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstaddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstaddr3`]
module"]
pub type HSTADDR3 = crate::Reg<hstaddr3::HSTADDR3_SPEC>;
#[doc = "Host Address 3 Register"]
pub mod hstaddr3;
#[doc = "HSTPIPCFG (rw) register accessor: Host Pipe Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg`]
module"]
pub type HSTPIPCFG = crate::Reg<hstpipcfg::HSTPIPCFG_SPEC>;
#[doc = "Host Pipe Configuration Register"]
pub mod hstpipcfg;
#[doc = "HSTPIPCFG_CTRL_BULK_MODE (rw) register accessor: Host Pipe Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg_ctrl_bulk_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg_ctrl_bulk_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipcfg_ctrl_bulk_mode`]
module"]
pub type HSTPIPCFG_CTRL_BULK_MODE =
    crate::Reg<hstpipcfg_ctrl_bulk_mode::HSTPIPCFG_CTRL_BULK_MODE_SPEC>;
#[doc = "Host Pipe Configuration Register"]
pub mod hstpipcfg_ctrl_bulk_mode;
#[doc = "HSTPIPISR_CTRL_MODE (r) register accessor: Host Pipe Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr_ctrl_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr_ctrl_mode`]
module"]
pub type HSTPIPISR_CTRL_MODE = crate::Reg<hstpipisr_ctrl_mode::HSTPIPISR_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_ctrl_mode;
#[doc = "HSTPIPISR_ISO_MODE (r) register accessor: Host Pipe Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr_iso_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr_iso_mode`]
module"]
pub type HSTPIPISR_ISO_MODE = crate::Reg<hstpipisr_iso_mode::HSTPIPISR_ISO_MODE_SPEC>;
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_iso_mode;
#[doc = "HSTPIPISR_BLK_MODE (r) register accessor: Host Pipe Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr_blk_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr_blk_mode`]
module"]
pub type HSTPIPISR_BLK_MODE = crate::Reg<hstpipisr_blk_mode::HSTPIPISR_BLK_MODE_SPEC>;
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_blk_mode;
#[doc = "HSTPIPISR_INTRPT_MODE (r) register accessor: Host Pipe Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr_intrpt_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipisr_intrpt_mode`]
module"]
pub type HSTPIPISR_INTRPT_MODE = crate::Reg<hstpipisr_intrpt_mode::HSTPIPISR_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Status Register"]
pub mod hstpipisr_intrpt_mode;
#[doc = "HSTPIPICR_CTRL_MODE (w) register accessor: Host Pipe Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr_ctrl_mode`]
module"]
pub type HSTPIPICR_CTRL_MODE = crate::Reg<hstpipicr_ctrl_mode::HSTPIPICR_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_ctrl_mode;
#[doc = "HSTPIPICR_ISO_MODE (w) register accessor: Host Pipe Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr_iso_mode`]
module"]
pub type HSTPIPICR_ISO_MODE = crate::Reg<hstpipicr_iso_mode::HSTPIPICR_ISO_MODE_SPEC>;
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_iso_mode;
#[doc = "HSTPIPICR_BLK_MODE (w) register accessor: Host Pipe Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr_blk_mode`]
module"]
pub type HSTPIPICR_BLK_MODE = crate::Reg<hstpipicr_blk_mode::HSTPIPICR_BLK_MODE_SPEC>;
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_blk_mode;
#[doc = "HSTPIPICR_INTRPT_MODE (w) register accessor: Host Pipe Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipicr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipicr_intrpt_mode`]
module"]
pub type HSTPIPICR_INTRPT_MODE = crate::Reg<hstpipicr_intrpt_mode::HSTPIPICR_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Clear Register"]
pub mod hstpipicr_intrpt_mode;
#[doc = "HSTPIPIFR_CTRL_MODE (w) register accessor: Host Pipe Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr_ctrl_mode`]
module"]
pub type HSTPIPIFR_CTRL_MODE = crate::Reg<hstpipifr_ctrl_mode::HSTPIPIFR_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_ctrl_mode;
#[doc = "HSTPIPIFR_ISO_MODE (w) register accessor: Host Pipe Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr_iso_mode`]
module"]
pub type HSTPIPIFR_ISO_MODE = crate::Reg<hstpipifr_iso_mode::HSTPIPIFR_ISO_MODE_SPEC>;
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_iso_mode;
#[doc = "HSTPIPIFR_BLK_MODE (w) register accessor: Host Pipe Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr_blk_mode`]
module"]
pub type HSTPIPIFR_BLK_MODE = crate::Reg<hstpipifr_blk_mode::HSTPIPIFR_BLK_MODE_SPEC>;
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_blk_mode;
#[doc = "HSTPIPIFR_INTRPT_MODE (w) register accessor: Host Pipe Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipifr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipifr_intrpt_mode`]
module"]
pub type HSTPIPIFR_INTRPT_MODE = crate::Reg<hstpipifr_intrpt_mode::HSTPIPIFR_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Set Register"]
pub mod hstpipifr_intrpt_mode;
#[doc = "HSTPIPIMR_CTRL_MODE (r) register accessor: Host Pipe Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr_ctrl_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr_ctrl_mode`]
module"]
pub type HSTPIPIMR_CTRL_MODE = crate::Reg<hstpipimr_ctrl_mode::HSTPIPIMR_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_ctrl_mode;
#[doc = "HSTPIPIMR_ISO_MODE (r) register accessor: Host Pipe Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr_iso_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr_iso_mode`]
module"]
pub type HSTPIPIMR_ISO_MODE = crate::Reg<hstpipimr_iso_mode::HSTPIPIMR_ISO_MODE_SPEC>;
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_iso_mode;
#[doc = "HSTPIPIMR_BLK_MODE (r) register accessor: Host Pipe Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr_blk_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr_blk_mode`]
module"]
pub type HSTPIPIMR_BLK_MODE = crate::Reg<hstpipimr_blk_mode::HSTPIPIMR_BLK_MODE_SPEC>;
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_blk_mode;
#[doc = "HSTPIPIMR_INTRPT_MODE (r) register accessor: Host Pipe Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipimr_intrpt_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipimr_intrpt_mode`]
module"]
pub type HSTPIPIMR_INTRPT_MODE = crate::Reg<hstpipimr_intrpt_mode::HSTPIPIMR_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Mask Register"]
pub mod hstpipimr_intrpt_mode;
#[doc = "HSTPIPIER_CTRL_MODE (w) register accessor: Host Pipe Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier_ctrl_mode`]
module"]
pub type HSTPIPIER_CTRL_MODE = crate::Reg<hstpipier_ctrl_mode::HSTPIPIER_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_ctrl_mode;
#[doc = "HSTPIPIER_ISO_MODE (w) register accessor: Host Pipe Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier_iso_mode`]
module"]
pub type HSTPIPIER_ISO_MODE = crate::Reg<hstpipier_iso_mode::HSTPIPIER_ISO_MODE_SPEC>;
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_iso_mode;
#[doc = "HSTPIPIER_BLK_MODE (w) register accessor: Host Pipe Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier_blk_mode`]
module"]
pub type HSTPIPIER_BLK_MODE = crate::Reg<hstpipier_blk_mode::HSTPIPIER_BLK_MODE_SPEC>;
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_blk_mode;
#[doc = "HSTPIPIER_INTRPT_MODE (w) register accessor: Host Pipe Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipier_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipier_intrpt_mode`]
module"]
pub type HSTPIPIER_INTRPT_MODE = crate::Reg<hstpipier_intrpt_mode::HSTPIPIER_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Enable Register"]
pub mod hstpipier_intrpt_mode;
#[doc = "HSTPIPIDR_CTRL_MODE (w) register accessor: Host Pipe Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr_ctrl_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr_ctrl_mode`]
module"]
pub type HSTPIPIDR_CTRL_MODE = crate::Reg<hstpipidr_ctrl_mode::HSTPIPIDR_CTRL_MODE_SPEC>;
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_ctrl_mode;
#[doc = "HSTPIPIDR_ISO_MODE (w) register accessor: Host Pipe Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr_iso_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr_iso_mode`]
module"]
pub type HSTPIPIDR_ISO_MODE = crate::Reg<hstpipidr_iso_mode::HSTPIPIDR_ISO_MODE_SPEC>;
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_iso_mode;
#[doc = "HSTPIPIDR_BLK_MODE (w) register accessor: Host Pipe Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr_blk_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr_blk_mode`]
module"]
pub type HSTPIPIDR_BLK_MODE = crate::Reg<hstpipidr_blk_mode::HSTPIPIDR_BLK_MODE_SPEC>;
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_blk_mode;
#[doc = "HSTPIPIDR_INTRPT_MODE (w) register accessor: Host Pipe Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipidr_intrpt_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipidr_intrpt_mode`]
module"]
pub type HSTPIPIDR_INTRPT_MODE = crate::Reg<hstpipidr_intrpt_mode::HSTPIPIDR_INTRPT_MODE_SPEC>;
#[doc = "Host Pipe Disable Register"]
pub mod hstpipidr_intrpt_mode;
#[doc = "HSTPIPINRQ (rw) register accessor: Host Pipe IN Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipinrq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipinrq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpipinrq`]
module"]
pub type HSTPIPINRQ = crate::Reg<hstpipinrq::HSTPIPINRQ_SPEC>;
#[doc = "Host Pipe IN Request Register"]
pub mod hstpipinrq;
#[doc = "HSTPIPERR (rw) register accessor: Host Pipe Error Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstpiperr`]
module"]
pub type HSTPIPERR = crate::Reg<hstpiperr::HSTPIPERR_SPEC>;
#[doc = "Host Pipe Error Register"]
pub mod hstpiperr;
#[doc = "Host DMA Channel Next Descriptor Address Register"]
pub use self::usbhs_hstdma::USBHS_HSTDMA;
#[doc = r"Cluster"]
#[doc = "Host DMA Channel Next Descriptor Address Register"]
pub mod usbhs_hstdma;
#[doc = "CTRL (rw) register accessor: General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "General Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: General Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "General Status Register"]
pub mod sr;
#[doc = "SCR (w) register accessor: General Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "General Status Clear Register"]
pub mod scr;
#[doc = "SFR (w) register accessor: General Status Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sfr`]
module"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "General Status Set Register"]
pub mod sfr;
