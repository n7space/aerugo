#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_HSTDMA {
    #[doc = "0x00 - Host DMA Channel Next Descriptor Address Register"]
    pub hstdmanxtdsc: HSTDMANXTDSC,
    #[doc = "0x04 - Host DMA Channel Address Register"]
    pub hstdmaaddress: HSTDMAADDRESS,
    #[doc = "0x08 - Host DMA Channel Control Register"]
    pub hstdmacontrol: HSTDMACONTROL,
    #[doc = "0x0c - Host DMA Channel Status Register"]
    pub hstdmastatus: HSTDMASTATUS,
}
#[doc = "HSTDMANXTDSC (rw) register accessor: Host DMA Channel Next Descriptor Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmanxtdsc`]
module"]
pub type HSTDMANXTDSC = crate::Reg<hstdmanxtdsc::HSTDMANXTDSC_SPEC>;
#[doc = "Host DMA Channel Next Descriptor Address Register"]
pub mod hstdmanxtdsc;
#[doc = "HSTDMAADDRESS (rw) register accessor: Host DMA Channel Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmaaddress::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmaaddress::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmaaddress`]
module"]
pub type HSTDMAADDRESS = crate::Reg<hstdmaaddress::HSTDMAADDRESS_SPEC>;
#[doc = "Host DMA Channel Address Register"]
pub mod hstdmaaddress;
#[doc = "HSTDMACONTROL (rw) register accessor: Host DMA Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmacontrol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmacontrol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmacontrol`]
module"]
pub type HSTDMACONTROL = crate::Reg<hstdmacontrol::HSTDMACONTROL_SPEC>;
#[doc = "Host DMA Channel Control Register"]
pub mod hstdmacontrol;
#[doc = "HSTDMASTATUS (rw) register accessor: Host DMA Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmastatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmastatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hstdmastatus`]
module"]
pub type HSTDMASTATUS = crate::Reg<hstdmastatus::HSTDMASTATUS_SPEC>;
#[doc = "Host DMA Channel Status Register"]
pub mod hstdmastatus;
