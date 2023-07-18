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
#[doc = "DEVDMANXTDSC (rw) register accessor: an alias for `Reg<DEVDMANXTDSC_SPEC>`"]
pub type DEVDMANXTDSC = crate::Reg<devdmanxtdsc::DEVDMANXTDSC_SPEC>;
#[doc = "Device DMA Channel Next Descriptor Address Register"]
pub mod devdmanxtdsc;
#[doc = "DEVDMAADDRESS (rw) register accessor: an alias for `Reg<DEVDMAADDRESS_SPEC>`"]
pub type DEVDMAADDRESS = crate::Reg<devdmaaddress::DEVDMAADDRESS_SPEC>;
#[doc = "Device DMA Channel Address Register"]
pub mod devdmaaddress;
#[doc = "DEVDMACONTROL (rw) register accessor: an alias for `Reg<DEVDMACONTROL_SPEC>`"]
pub type DEVDMACONTROL = crate::Reg<devdmacontrol::DEVDMACONTROL_SPEC>;
#[doc = "Device DMA Channel Control Register"]
pub mod devdmacontrol;
#[doc = "DEVDMASTATUS (rw) register accessor: an alias for `Reg<DEVDMASTATUS_SPEC>`"]
pub type DEVDMASTATUS = crate::Reg<devdmastatus::DEVDMASTATUS_SPEC>;
#[doc = "Device DMA Channel Status Register"]
pub mod devdmastatus;
