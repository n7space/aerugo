#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Type Register"]
    pub gtype: GTYPE,
    #[doc = "0x04 - Global Configuration Register"]
    pub gcfg: GCFG,
    #[doc = "0x08 - Global Weighted Arbiter Configuration Register"]
    pub gwac: GWAC,
    #[doc = "0x0c - Global Interrupt Enable Register"]
    pub gie: GIE,
    #[doc = "0x10 - Global Interrupt Disable Register"]
    pub gid: GID,
    #[doc = "0x14 - Global Interrupt Mask Register"]
    pub gim: GIM,
    #[doc = "0x18 - Global Interrupt Status Register"]
    pub gis: GIS,
    #[doc = "0x1c - Global Channel Enable Register"]
    pub ge: GE,
    #[doc = "0x20 - Global Channel Disable Register"]
    pub gd: GD,
    #[doc = "0x24 - Global Channel Status Register"]
    pub gs: GS,
    #[doc = "0x28 - Global Channel Read Suspend Register"]
    pub grs: GRS,
    #[doc = "0x2c - Global Channel Write Suspend Register"]
    pub gws: GWS,
    #[doc = "0x30 - Global Channel Read Write Suspend Register"]
    pub grws: GRWS,
    #[doc = "0x34 - Global Channel Read Write Resume Register"]
    pub grwr: GRWR,
    #[doc = "0x38 - Global Channel Software Request Register"]
    pub gswr: GSWR,
    #[doc = "0x3c - Global Channel Software Request Status Register"]
    pub gsws: GSWS,
    #[doc = "0x40 - Global Channel Software Flush Request Register"]
    pub gswf: GSWF,
    _reserved17: [u8; 0x0c],
    #[doc = "0x50..0x88 - Channel Interrupt Enable Register"]
    pub xdmac_chid0: XDMAC_CHID,
    _reserved18: [u8; 0x08],
    #[doc = "0x90..0xc8 - Channel Interrupt Enable Register"]
    pub xdmac_chid1: XDMAC_CHID,
    _reserved19: [u8; 0x08],
    #[doc = "0xd0..0x108 - Channel Interrupt Enable Register"]
    pub xdmac_chid2: XDMAC_CHID,
    _reserved20: [u8; 0x08],
    #[doc = "0x110..0x148 - Channel Interrupt Enable Register"]
    pub xdmac_chid3: XDMAC_CHID,
    _reserved21: [u8; 0x08],
    #[doc = "0x150..0x188 - Channel Interrupt Enable Register"]
    pub xdmac_chid4: XDMAC_CHID,
    _reserved22: [u8; 0x08],
    #[doc = "0x190..0x1c8 - Channel Interrupt Enable Register"]
    pub xdmac_chid5: XDMAC_CHID,
    _reserved23: [u8; 0x08],
    #[doc = "0x1d0..0x208 - Channel Interrupt Enable Register"]
    pub xdmac_chid6: XDMAC_CHID,
    _reserved24: [u8; 0x08],
    #[doc = "0x210..0x248 - Channel Interrupt Enable Register"]
    pub xdmac_chid7: XDMAC_CHID,
    _reserved25: [u8; 0x08],
    #[doc = "0x250..0x288 - Channel Interrupt Enable Register"]
    pub xdmac_chid8: XDMAC_CHID,
    _reserved26: [u8; 0x08],
    #[doc = "0x290..0x2c8 - Channel Interrupt Enable Register"]
    pub xdmac_chid9: XDMAC_CHID,
    _reserved27: [u8; 0x08],
    #[doc = "0x2d0..0x308 - Channel Interrupt Enable Register"]
    pub xdmac_chid10: XDMAC_CHID,
    _reserved28: [u8; 0x08],
    #[doc = "0x310..0x348 - Channel Interrupt Enable Register"]
    pub xdmac_chid11: XDMAC_CHID,
    _reserved29: [u8; 0x08],
    #[doc = "0x350..0x388 - Channel Interrupt Enable Register"]
    pub xdmac_chid12: XDMAC_CHID,
    _reserved30: [u8; 0x08],
    #[doc = "0x390..0x3c8 - Channel Interrupt Enable Register"]
    pub xdmac_chid13: XDMAC_CHID,
    _reserved31: [u8; 0x08],
    #[doc = "0x3d0..0x408 - Channel Interrupt Enable Register"]
    pub xdmac_chid14: XDMAC_CHID,
    _reserved32: [u8; 0x08],
    #[doc = "0x410..0x448 - Channel Interrupt Enable Register"]
    pub xdmac_chid15: XDMAC_CHID,
    _reserved33: [u8; 0x08],
    #[doc = "0x450..0x488 - Channel Interrupt Enable Register"]
    pub xdmac_chid16: XDMAC_CHID,
    _reserved34: [u8; 0x08],
    #[doc = "0x490..0x4c8 - Channel Interrupt Enable Register"]
    pub xdmac_chid17: XDMAC_CHID,
    _reserved35: [u8; 0x08],
    #[doc = "0x4d0..0x508 - Channel Interrupt Enable Register"]
    pub xdmac_chid18: XDMAC_CHID,
    _reserved36: [u8; 0x08],
    #[doc = "0x510..0x548 - Channel Interrupt Enable Register"]
    pub xdmac_chid19: XDMAC_CHID,
    _reserved37: [u8; 0x08],
    #[doc = "0x550..0x588 - Channel Interrupt Enable Register"]
    pub xdmac_chid20: XDMAC_CHID,
    _reserved38: [u8; 0x08],
    #[doc = "0x590..0x5c8 - Channel Interrupt Enable Register"]
    pub xdmac_chid21: XDMAC_CHID,
    _reserved39: [u8; 0x08],
    #[doc = "0x5d0..0x608 - Channel Interrupt Enable Register"]
    pub xdmac_chid22: XDMAC_CHID,
    _reserved40: [u8; 0x08],
    #[doc = "0x610..0x648 - Channel Interrupt Enable Register"]
    pub xdmac_chid23: XDMAC_CHID,
}
#[doc = "GTYPE (r) register accessor: an alias for `Reg<GTYPE_SPEC>`"]
pub type GTYPE = crate::Reg<gtype::GTYPE_SPEC>;
#[doc = "Global Type Register"]
pub mod gtype;
#[doc = "GCFG (rw) register accessor: an alias for `Reg<GCFG_SPEC>`"]
pub type GCFG = crate::Reg<gcfg::GCFG_SPEC>;
#[doc = "Global Configuration Register"]
pub mod gcfg;
#[doc = "GWAC (rw) register accessor: an alias for `Reg<GWAC_SPEC>`"]
pub type GWAC = crate::Reg<gwac::GWAC_SPEC>;
#[doc = "Global Weighted Arbiter Configuration Register"]
pub mod gwac;
#[doc = "GIE (w) register accessor: an alias for `Reg<GIE_SPEC>`"]
pub type GIE = crate::Reg<gie::GIE_SPEC>;
#[doc = "Global Interrupt Enable Register"]
pub mod gie;
#[doc = "GID (w) register accessor: an alias for `Reg<GID_SPEC>`"]
pub type GID = crate::Reg<gid::GID_SPEC>;
#[doc = "Global Interrupt Disable Register"]
pub mod gid;
#[doc = "GIM (r) register accessor: an alias for `Reg<GIM_SPEC>`"]
pub type GIM = crate::Reg<gim::GIM_SPEC>;
#[doc = "Global Interrupt Mask Register"]
pub mod gim;
#[doc = "GIS (r) register accessor: an alias for `Reg<GIS_SPEC>`"]
pub type GIS = crate::Reg<gis::GIS_SPEC>;
#[doc = "Global Interrupt Status Register"]
pub mod gis;
#[doc = "GE (w) register accessor: an alias for `Reg<GE_SPEC>`"]
pub type GE = crate::Reg<ge::GE_SPEC>;
#[doc = "Global Channel Enable Register"]
pub mod ge;
#[doc = "GD (w) register accessor: an alias for `Reg<GD_SPEC>`"]
pub type GD = crate::Reg<gd::GD_SPEC>;
#[doc = "Global Channel Disable Register"]
pub mod gd;
#[doc = "GS (r) register accessor: an alias for `Reg<GS_SPEC>`"]
pub type GS = crate::Reg<gs::GS_SPEC>;
#[doc = "Global Channel Status Register"]
pub mod gs;
#[doc = "GRS (rw) register accessor: an alias for `Reg<GRS_SPEC>`"]
pub type GRS = crate::Reg<grs::GRS_SPEC>;
#[doc = "Global Channel Read Suspend Register"]
pub mod grs;
#[doc = "GWS (rw) register accessor: an alias for `Reg<GWS_SPEC>`"]
pub type GWS = crate::Reg<gws::GWS_SPEC>;
#[doc = "Global Channel Write Suspend Register"]
pub mod gws;
#[doc = "GRWS (w) register accessor: an alias for `Reg<GRWS_SPEC>`"]
pub type GRWS = crate::Reg<grws::GRWS_SPEC>;
#[doc = "Global Channel Read Write Suspend Register"]
pub mod grws;
#[doc = "GRWR (w) register accessor: an alias for `Reg<GRWR_SPEC>`"]
pub type GRWR = crate::Reg<grwr::GRWR_SPEC>;
#[doc = "Global Channel Read Write Resume Register"]
pub mod grwr;
#[doc = "GSWR (w) register accessor: an alias for `Reg<GSWR_SPEC>`"]
pub type GSWR = crate::Reg<gswr::GSWR_SPEC>;
#[doc = "Global Channel Software Request Register"]
pub mod gswr;
#[doc = "GSWS (r) register accessor: an alias for `Reg<GSWS_SPEC>`"]
pub type GSWS = crate::Reg<gsws::GSWS_SPEC>;
#[doc = "Global Channel Software Request Status Register"]
pub mod gsws;
#[doc = "GSWF (w) register accessor: an alias for `Reg<GSWF_SPEC>`"]
pub type GSWF = crate::Reg<gswf::GSWF_SPEC>;
#[doc = "Global Channel Software Flush Request Register"]
pub mod gswf;
#[doc = "Channel Interrupt Enable Register"]
pub use self::xdmac_chid::XDMAC_CHID;
#[doc = r"Cluster"]
#[doc = "Channel Interrupt Enable Register"]
pub mod xdmac_chid;
