#[doc = r"Register block"]
#[repr(C)]
pub struct XDMAC_CHID {
    #[doc = "0x00 - Channel Interrupt Enable Register"]
    pub cie: CIE,
    #[doc = "0x04 - Channel Interrupt Disable Register"]
    pub cid: CID,
    #[doc = "0x08 - Channel Interrupt Mask Register"]
    pub cim: CIM,
    #[doc = "0x0c - Channel Interrupt Status Register"]
    pub cis: CIS,
    #[doc = "0x10 - Channel Source Address Register"]
    pub csa: CSA,
    #[doc = "0x14 - Channel Destination Address Register"]
    pub cda: CDA,
    #[doc = "0x18 - Channel Next Descriptor Address Register"]
    pub cnda: CNDA,
    #[doc = "0x1c - Channel Next Descriptor Control Register"]
    pub cndc: CNDC,
    #[doc = "0x20 - Channel Microblock Control Register"]
    pub cubc: CUBC,
    #[doc = "0x24 - Channel Block Control Register"]
    pub cbc: CBC,
    #[doc = "0x28 - Channel Configuration Register"]
    pub cc: CC,
    #[doc = "0x2c - Channel Data Stride Memory Set Pattern"]
    pub cds_msp: CDS_MSP,
    #[doc = "0x30 - Channel Source Microblock Stride"]
    pub csus: CSUS,
    #[doc = "0x34 - Channel Destination Microblock Stride"]
    pub cdus: CDUS,
}
#[doc = "CIE (w) register accessor: Channel Interrupt Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cie::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cie`]
module"]
pub type CIE = crate::Reg<cie::CIE_SPEC>;
#[doc = "Channel Interrupt Enable Register"]
pub mod cie;
#[doc = "CID (w) register accessor: Channel Interrupt Disable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cid`]
module"]
pub type CID = crate::Reg<cid::CID_SPEC>;
#[doc = "Channel Interrupt Disable Register"]
pub mod cid;
#[doc = "CIM (r) register accessor: Channel Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cim::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cim`]
module"]
pub type CIM = crate::Reg<cim::CIM_SPEC>;
#[doc = "Channel Interrupt Mask Register"]
pub mod cim;
#[doc = "CIS (r) register accessor: Channel Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cis`]
module"]
pub type CIS = crate::Reg<cis::CIS_SPEC>;
#[doc = "Channel Interrupt Status Register"]
pub mod cis;
#[doc = "CSA (rw) register accessor: Channel Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`csa`]
module"]
pub type CSA = crate::Reg<csa::CSA_SPEC>;
#[doc = "Channel Source Address Register"]
pub mod csa;
#[doc = "CDA (rw) register accessor: Channel Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cda`]
module"]
pub type CDA = crate::Reg<cda::CDA_SPEC>;
#[doc = "Channel Destination Address Register"]
pub mod cda;
#[doc = "CNDA (rw) register accessor: Channel Next Descriptor Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnda`]
module"]
pub type CNDA = crate::Reg<cnda::CNDA_SPEC>;
#[doc = "Channel Next Descriptor Address Register"]
pub mod cnda;
#[doc = "CNDC (rw) register accessor: Channel Next Descriptor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cndc`]
module"]
pub type CNDC = crate::Reg<cndc::CNDC_SPEC>;
#[doc = "Channel Next Descriptor Control Register"]
pub mod cndc;
#[doc = "CUBC (rw) register accessor: Channel Microblock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cubc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cubc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cubc`]
module"]
pub type CUBC = crate::Reg<cubc::CUBC_SPEC>;
#[doc = "Channel Microblock Control Register"]
pub mod cubc;
#[doc = "CBC (rw) register accessor: Channel Block Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cbc`]
module"]
pub type CBC = crate::Reg<cbc::CBC_SPEC>;
#[doc = "Channel Block Control Register"]
pub mod cbc;
#[doc = "CC (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cc`]
module"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod cc;
#[doc = "CDS_MSP (rw) register accessor: Channel Data Stride Memory Set Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cds_msp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cds_msp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cds_msp`]
module"]
pub type CDS_MSP = crate::Reg<cds_msp::CDS_MSP_SPEC>;
#[doc = "Channel Data Stride Memory Set Pattern"]
pub mod cds_msp;
#[doc = "CSUS (rw) register accessor: Channel Source Microblock Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`csus`]
module"]
pub type CSUS = crate::Reg<csus::CSUS_SPEC>;
#[doc = "Channel Source Microblock Stride"]
pub mod csus;
#[doc = "CDUS (rw) register accessor: Channel Destination Microblock Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cdus`]
module"]
pub type CDUS = crate::Reg<cdus::CDUS_SPEC>;
#[doc = "Channel Destination Microblock Stride"]
pub mod cdus;
