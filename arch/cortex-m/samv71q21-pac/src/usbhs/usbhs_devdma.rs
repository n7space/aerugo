#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_DEVDMA {
    #[doc = "0x00 - Device DMA Channel Next Descriptor Address Register"]
    pub devdmanxtdsc: DEVDMANXTDSC,
    #[doc = "0x04 - Device DMA Channel Address Register"]
    pub devdmaaddress: DEVDMAADDRESS,
    #[doc = "0x08 - Device DMA Channel Control Register"]
    pub devdmacontrol: DEVDMACONTROL,
    #[doc = "0x0c - Device DMA Channel Status Register"]
    pub devdmastatus: DEVDMASTATUS,
}
#[doc = "DEVDMANXTDSC (rw) register accessor: Device DMA Channel Next Descriptor Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmanxtdsc`]
module"]
pub type DEVDMANXTDSC = crate::Reg<devdmanxtdsc::DEVDMANXTDSC_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register"]
pub mod devdmanxtdsc;
#[doc = "DEVDMAADDRESS (rw) register accessor: Device DMA Channel Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmaaddress::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmaaddress::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmaaddress`]
module"]
pub type DEVDMAADDRESS = crate::Reg<devdmaaddress::DEVDMAADDRESS_SPEC>;
#[doc = "Device DMA Channel Address Register"]
pub mod devdmaaddress;
#[doc = "DEVDMACONTROL (rw) register accessor: Device DMA Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmacontrol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmacontrol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmacontrol`]
module"]
pub type DEVDMACONTROL = crate::Reg<devdmacontrol::DEVDMACONTROL_SPEC>;
#[doc = "Device DMA Channel Control Register"]
pub mod devdmacontrol;
#[doc = "DEVDMASTATUS (rw) register accessor: Device DMA Channel Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmastatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmastatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devdmastatus`]
module"]
pub type DEVDMASTATUS = crate::Reg<devdmastatus::DEVDMASTATUS_SPEC>;
#[doc = "Device DMA Channel Status Register"]
pub mod devdmastatus;
