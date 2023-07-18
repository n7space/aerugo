#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network Control Register"]
    pub ncr: NCR,
    #[doc = "0x04 - Network Configuration Register"]
    pub ncfgr: NCFGR,
    #[doc = "0x08 - Network Status Register"]
    pub nsr: NSR,
    #[doc = "0x0c - User Register"]
    pub ur: UR,
    #[doc = "0x10 - DMA Configuration Register"]
    pub dcfgr: DCFGR,
    #[doc = "0x14 - Transmit Status Register"]
    pub tsr: TSR,
    #[doc = "0x18 - Receive Buffer Queue Base Address Register"]
    pub rbqb: RBQB,
    #[doc = "0x1c - Transmit Buffer Queue Base Address Register"]
    pub tbqb: TBQB,
    #[doc = "0x20 - Receive Status Register"]
    pub rsr: RSR,
    #[doc = "0x24 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x34 - PHY Maintenance Register"]
    pub man: MAN,
    #[doc = "0x38 - Received Pause Quantum Register"]
    pub rpq: RPQ,
    #[doc = "0x3c - Transmit Pause Quantum Register"]
    pub tpq: TPQ,
    #[doc = "0x40 - TX Partial Store and Forward Register"]
    pub tpsf: TPSF,
    #[doc = "0x44 - RX Partial Store and Forward Register"]
    pub rpsf: RPSF,
    #[doc = "0x48 - RX Jumbo Frame Max Length Register"]
    pub rjfml: RJFML,
    _reserved19: [u8; 0x34],
    #[doc = "0x80 - Hash Register Bottom"]
    pub hrb: HRB,
    #[doc = "0x84 - Hash Register Top"]
    pub hrt: HRT,
    #[doc = "0x88..0xa8 - Specific Address 1 Bottom Register"]
    pub gmac_sa: [GMAC_SA; 4],
    #[doc = "0xa8 - Type ID Match 1 Register"]
    pub tidm1: TIDM1,
    #[doc = "0xac - Type ID Match 2 Register"]
    pub tidm2: TIDM2,
    #[doc = "0xb0 - Type ID Match 3 Register"]
    pub tidm3: TIDM3,
    #[doc = "0xb4 - Type ID Match 4 Register"]
    pub tidm4: TIDM4,
    #[doc = "0xb8 - Wake on LAN Register"]
    pub wol: WOL,
    #[doc = "0xbc - IPG Stretch Register"]
    pub ipgs: IPGS,
    #[doc = "0xc0 - Stacked VLAN Register"]
    pub svlan: SVLAN,
    #[doc = "0xc4 - Transmit PFC Pause Register"]
    pub tpfcp: TPFCP,
    #[doc = "0xc8 - Specific Address 1 Mask Bottom Register"]
    pub samb1: SAMB1,
    #[doc = "0xcc - Specific Address 1 Mask Top Register"]
    pub samt1: SAMT1,
    _reserved32: [u8; 0x0c],
    #[doc = "0xdc - 1588 Timer Nanosecond Comparison Register"]
    pub nsc: NSC,
    #[doc = "0xe0 - 1588 Timer Second Comparison Low Register"]
    pub scl: SCL,
    #[doc = "0xe4 - 1588 Timer Second Comparison High Register"]
    pub sch: SCH,
    #[doc = "0xe8 - PTP Event Frame Transmitted Seconds High Register"]
    pub eftsh: EFTSH,
    #[doc = "0xec - PTP Event Frame Received Seconds High Register"]
    pub efrsh: EFRSH,
    #[doc = "0xf0 - PTP Peer Event Frame Transmitted Seconds High Register"]
    pub peftsh: PEFTSH,
    #[doc = "0xf4 - PTP Peer Event Frame Received Seconds High Register"]
    pub pefrsh: PEFRSH,
    _reserved39: [u8; 0x08],
    #[doc = "0x100 - Octets Transmitted Low Register"]
    pub otlo: OTLO,
    #[doc = "0x104 - Octets Transmitted High Register"]
    pub othi: OTHI,
    #[doc = "0x108 - Frames Transmitted Register"]
    pub ft: FT,
    #[doc = "0x10c - Broadcast Frames Transmitted Register"]
    pub bcft: BCFT,
    #[doc = "0x110 - Multicast Frames Transmitted Register"]
    pub mft: MFT,
    #[doc = "0x114 - Pause Frames Transmitted Register"]
    pub pft: PFT,
    #[doc = "0x118 - 64 Byte Frames Transmitted Register"]
    pub bft64: BFT64,
    #[doc = "0x11c - 65 to 127 Byte Frames Transmitted Register"]
    pub tbft127: TBFT127,
    #[doc = "0x120 - 128 to 255 Byte Frames Transmitted Register"]
    pub tbft255: TBFT255,
    #[doc = "0x124 - 256 to 511 Byte Frames Transmitted Register"]
    pub tbft511: TBFT511,
    #[doc = "0x128 - 512 to 1023 Byte Frames Transmitted Register"]
    pub tbft1023: TBFT1023,
    #[doc = "0x12c - 1024 to 1518 Byte Frames Transmitted Register"]
    pub tbft1518: TBFT1518,
    #[doc = "0x130 - Greater Than 1518 Byte Frames Transmitted Register"]
    pub gtbft1518: GTBFT1518,
    #[doc = "0x134 - Transmit Underruns Register"]
    pub tur: TUR,
    #[doc = "0x138 - Single Collision Frames Register"]
    pub scf: SCF,
    #[doc = "0x13c - Multiple Collision Frames Register"]
    pub mcf: MCF,
    #[doc = "0x140 - Excessive Collisions Register"]
    pub ec: EC,
    #[doc = "0x144 - Late Collisions Register"]
    pub lc: LC,
    #[doc = "0x148 - Deferred Transmission Frames Register"]
    pub dtf: DTF,
    #[doc = "0x14c - Carrier Sense Errors Register"]
    pub cse: CSE,
    #[doc = "0x150 - Octets Received Low Received Register"]
    pub orlo: ORLO,
    #[doc = "0x154 - Octets Received High Received Register"]
    pub orhi: ORHI,
    #[doc = "0x158 - Frames Received Register"]
    pub fr: FR,
    #[doc = "0x15c - Broadcast Frames Received Register"]
    pub bcfr: BCFR,
    #[doc = "0x160 - Multicast Frames Received Register"]
    pub mfr: MFR,
    #[doc = "0x164 - Pause Frames Received Register"]
    pub pfr: PFR,
    #[doc = "0x168 - 64 Byte Frames Received Register"]
    pub bfr64: BFR64,
    #[doc = "0x16c - 65 to 127 Byte Frames Received Register"]
    pub tbfr127: TBFR127,
    #[doc = "0x170 - 128 to 255 Byte Frames Received Register"]
    pub tbfr255: TBFR255,
    #[doc = "0x174 - 256 to 511 Byte Frames Received Register"]
    pub tbfr511: TBFR511,
    #[doc = "0x178 - 512 to 1023 Byte Frames Received Register"]
    pub tbfr1023: TBFR1023,
    #[doc = "0x17c - 1024 to 1518 Byte Frames Received Register"]
    pub tbfr1518: TBFR1518,
    #[doc = "0x180 - 1519 to Maximum Byte Frames Received Register"]
    pub tmxbfr: TMXBFR,
    #[doc = "0x184 - Undersize Frames Received Register"]
    pub ufr: UFR,
    #[doc = "0x188 - Oversize Frames Received Register"]
    pub ofr: OFR,
    #[doc = "0x18c - Jabbers Received Register"]
    pub jr: JR,
    #[doc = "0x190 - Frame Check Sequence Errors Register"]
    pub fcse: FCSE,
    #[doc = "0x194 - Length Field Frame Errors Register"]
    pub lffe: LFFE,
    #[doc = "0x198 - Receive Symbol Errors Register"]
    pub rse: RSE,
    #[doc = "0x19c - Alignment Errors Register"]
    pub ae: AE,
    #[doc = "0x1a0 - Receive Resource Errors Register"]
    pub rre: RRE,
    #[doc = "0x1a4 - Receive Overrun Register"]
    pub roe: ROE,
    #[doc = "0x1a8 - IP Header Checksum Errors Register"]
    pub ihce: IHCE,
    #[doc = "0x1ac - TCP Checksum Errors Register"]
    pub tce: TCE,
    #[doc = "0x1b0 - UDP Checksum Errors Register"]
    pub uce: UCE,
    _reserved84: [u8; 0x08],
    #[doc = "0x1bc - 1588 Timer Increment Sub-nanoseconds Register"]
    pub tisubn: TISUBN,
    #[doc = "0x1c0 - 1588 Timer Seconds High Register"]
    pub tsh: TSH,
    _reserved86: [u8; 0x0c],
    #[doc = "0x1d0 - 1588 Timer Seconds Low Register"]
    pub tsl: TSL,
    #[doc = "0x1d4 - 1588 Timer Nanoseconds Register"]
    pub tn: TN,
    #[doc = "0x1d8 - 1588 Timer Adjust Register"]
    pub ta: TA,
    #[doc = "0x1dc - 1588 Timer Increment Register"]
    pub ti: TI,
    #[doc = "0x1e0 - PTP Event Frame Transmitted Seconds Low Register"]
    pub eftsl: EFTSL,
    #[doc = "0x1e4 - PTP Event Frame Transmitted Nanoseconds Register"]
    pub eftn: EFTN,
    #[doc = "0x1e8 - PTP Event Frame Received Seconds Low Register"]
    pub efrsl: EFRSL,
    #[doc = "0x1ec - PTP Event Frame Received Nanoseconds Register"]
    pub efrn: EFRN,
    #[doc = "0x1f0 - PTP Peer Event Frame Transmitted Seconds Low Register"]
    pub peftsl: PEFTSL,
    #[doc = "0x1f4 - PTP Peer Event Frame Transmitted Nanoseconds Register"]
    pub peftn: PEFTN,
    #[doc = "0x1f8 - PTP Peer Event Frame Received Seconds Low Register"]
    pub pefrsl: PEFRSL,
    #[doc = "0x1fc - PTP Peer Event Frame Received Nanoseconds Register"]
    pub pefrn: PEFRN,
    _reserved98: [u8; 0x70],
    #[doc = "0x270 - Received LPI Transitions"]
    pub rxlpi: RXLPI,
    #[doc = "0x274 - Received LPI Time"]
    pub rxlpitime: RXLPITIME,
    #[doc = "0x278 - Transmit LPI Transitions"]
    pub txlpi: TXLPI,
    #[doc = "0x27c - Transmit LPI Time"]
    pub txlpitime: TXLPITIME,
    _reserved102: [u8; 0x0180],
    #[doc = "0x400..0x414 - Interrupt Status Register Priority Queue (1..5)"]
    pub isrpq: [ISRPQ; 5],
    _reserved103: [u8; 0x2c],
    #[doc = "0x440..0x454 - Transmit Buffer Queue Base Address Register Priority Queue (1..5)"]
    pub tbqbapq: [TBQBAPQ; 5],
    _reserved104: [u8; 0x2c],
    #[doc = "0x480..0x494 - Receive Buffer Queue Base Address Register Priority Queue (1..5)"]
    pub rbqbapq: [RBQBAPQ; 5],
    _reserved105: [u8; 0x0c],
    #[doc = "0x4a0..0x4b4 - Receive Buffer Size Register Priority Queue (1..5)"]
    pub rbsrpq: [RBSRPQ; 5],
    _reserved106: [u8; 0x08],
    #[doc = "0x4bc - Credit-Based Shaping Control Register"]
    pub cbscr: CBSCR,
    #[doc = "0x4c0 - Credit-Based Shaping IdleSlope Register for Queue A"]
    pub cbsisqa: CBSISQA,
    #[doc = "0x4c4 - Credit-Based Shaping IdleSlope Register for Queue B"]
    pub cbsisqb: CBSISQB,
    _reserved109: [u8; 0x38],
    #[doc = "0x500..0x510 - Screening Type 1 Register Priority Queue"]
    pub st1rpq: [ST1RPQ; 4],
    _reserved110: [u8; 0x30],
    #[doc = "0x540..0x560 - Screening Type 2 Register Priority Queue"]
    pub st2rpq: [ST2RPQ; 8],
    _reserved111: [u8; 0xa0],
    #[doc = "0x600..0x614 - Interrupt Enable Register Priority Queue (1..5)"]
    pub ierpq: [IERPQ; 5],
    _reserved112: [u8; 0x0c],
    #[doc = "0x620..0x634 - Interrupt Disable Register Priority Queue (1..5)"]
    pub idrpq: [IDRPQ; 5],
    _reserved113: [u8; 0x0c],
    #[doc = "0x640..0x654 - Interrupt Mask Register Priority Queue (1..5)"]
    pub imrpq: [IMRPQ; 5],
    _reserved114: [u8; 0x8c],
    #[doc = "0x6e0..0x6f0 - Screening Type 2 Ethertype Register"]
    pub st2er: [ST2ER; 4],
    _reserved115: [u8; 0x10],
    #[doc = "0x700..0x7c0 - Screening Type 2 Compare Word 0 Register"]
    pub gmac_st2cw: [GMAC_ST2CW; 24],
}
impl RegisterBlock {
    #[doc = "0x88..0x90 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub fn gmac_sa1(&self) -> &GMAC_SA {
        &self.gmac_sa[0]
    }
    #[doc = "0x90..0x98 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub fn gmac_sa2(&self) -> &GMAC_SA {
        &self.gmac_sa[1]
    }
    #[doc = "0x98..0xa0 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub fn gmac_sa3(&self) -> &GMAC_SA {
        &self.gmac_sa[2]
    }
    #[doc = "0xa0..0xa8 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub fn gmac_sa4(&self) -> &GMAC_SA {
        &self.gmac_sa[3]
    }
}
#[doc = "NCR (rw) register accessor: an alias for `Reg<NCR_SPEC>`"]
pub type NCR = crate::Reg<ncr::NCR_SPEC>;
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "NCFGR (rw) register accessor: an alias for `Reg<NCFGR_SPEC>`"]
pub type NCFGR = crate::Reg<ncfgr::NCFGR_SPEC>;
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "NSR (r) register accessor: an alias for `Reg<NSR_SPEC>`"]
pub type NSR = crate::Reg<nsr::NSR_SPEC>;
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "UR (rw) register accessor: an alias for `Reg<UR_SPEC>`"]
pub type UR = crate::Reg<ur::UR_SPEC>;
#[doc = "User Register"]
pub mod ur;
#[doc = "DCFGR (rw) register accessor: an alias for `Reg<DCFGR_SPEC>`"]
pub type DCFGR = crate::Reg<dcfgr::DCFGR_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod dcfgr;
#[doc = "TSR (rw) register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "RBQB (rw) register accessor: an alias for `Reg<RBQB_SPEC>`"]
pub type RBQB = crate::Reg<rbqb::RBQB_SPEC>;
#[doc = "Receive Buffer Queue Base Address Register"]
pub mod rbqb;
#[doc = "TBQB (rw) register accessor: an alias for `Reg<TBQB_SPEC>`"]
pub type TBQB = crate::Reg<tbqb::TBQB_SPEC>;
#[doc = "Transmit Buffer Queue Base Address Register"]
pub mod tbqb;
#[doc = "RSR (rw) register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "MAN (rw) register accessor: an alias for `Reg<MAN_SPEC>`"]
pub type MAN = crate::Reg<man::MAN_SPEC>;
#[doc = "PHY Maintenance Register"]
pub mod man;
#[doc = "RPQ (r) register accessor: an alias for `Reg<RPQ_SPEC>`"]
pub type RPQ = crate::Reg<rpq::RPQ_SPEC>;
#[doc = "Received Pause Quantum Register"]
pub mod rpq;
#[doc = "TPQ (rw) register accessor: an alias for `Reg<TPQ_SPEC>`"]
pub type TPQ = crate::Reg<tpq::TPQ_SPEC>;
#[doc = "Transmit Pause Quantum Register"]
pub mod tpq;
#[doc = "TPSF (rw) register accessor: an alias for `Reg<TPSF_SPEC>`"]
pub type TPSF = crate::Reg<tpsf::TPSF_SPEC>;
#[doc = "TX Partial Store and Forward Register"]
pub mod tpsf;
#[doc = "RPSF (rw) register accessor: an alias for `Reg<RPSF_SPEC>`"]
pub type RPSF = crate::Reg<rpsf::RPSF_SPEC>;
#[doc = "RX Partial Store and Forward Register"]
pub mod rpsf;
#[doc = "RJFML (rw) register accessor: an alias for `Reg<RJFML_SPEC>`"]
pub type RJFML = crate::Reg<rjfml::RJFML_SPEC>;
#[doc = "RX Jumbo Frame Max Length Register"]
pub mod rjfml;
#[doc = "HRB (rw) register accessor: an alias for `Reg<HRB_SPEC>`"]
pub type HRB = crate::Reg<hrb::HRB_SPEC>;
#[doc = "Hash Register Bottom"]
pub mod hrb;
#[doc = "HRT (rw) register accessor: an alias for `Reg<HRT_SPEC>`"]
pub type HRT = crate::Reg<hrt::HRT_SPEC>;
#[doc = "Hash Register Top"]
pub mod hrt;
#[doc = "Specific Address 1 Bottom Register"]
pub use self::gmac_sa::GMAC_SA;
#[doc = r"Cluster"]
#[doc = "Specific Address 1 Bottom Register"]
pub mod gmac_sa;
#[doc = "TIDM1 (rw) register accessor: an alias for `Reg<TIDM1_SPEC>`"]
pub type TIDM1 = crate::Reg<tidm1::TIDM1_SPEC>;
#[doc = "Type ID Match 1 Register"]
pub mod tidm1;
#[doc = "TIDM2 (rw) register accessor: an alias for `Reg<TIDM2_SPEC>`"]
pub type TIDM2 = crate::Reg<tidm2::TIDM2_SPEC>;
#[doc = "Type ID Match 2 Register"]
pub mod tidm2;
#[doc = "TIDM3 (rw) register accessor: an alias for `Reg<TIDM3_SPEC>`"]
pub type TIDM3 = crate::Reg<tidm3::TIDM3_SPEC>;
#[doc = "Type ID Match 3 Register"]
pub mod tidm3;
#[doc = "TIDM4 (rw) register accessor: an alias for `Reg<TIDM4_SPEC>`"]
pub type TIDM4 = crate::Reg<tidm4::TIDM4_SPEC>;
#[doc = "Type ID Match 4 Register"]
pub mod tidm4;
#[doc = "WOL (rw) register accessor: an alias for `Reg<WOL_SPEC>`"]
pub type WOL = crate::Reg<wol::WOL_SPEC>;
#[doc = "Wake on LAN Register"]
pub mod wol;
#[doc = "IPGS (rw) register accessor: an alias for `Reg<IPGS_SPEC>`"]
pub type IPGS = crate::Reg<ipgs::IPGS_SPEC>;
#[doc = "IPG Stretch Register"]
pub mod ipgs;
#[doc = "SVLAN (rw) register accessor: an alias for `Reg<SVLAN_SPEC>`"]
pub type SVLAN = crate::Reg<svlan::SVLAN_SPEC>;
#[doc = "Stacked VLAN Register"]
pub mod svlan;
#[doc = "TPFCP (rw) register accessor: an alias for `Reg<TPFCP_SPEC>`"]
pub type TPFCP = crate::Reg<tpfcp::TPFCP_SPEC>;
#[doc = "Transmit PFC Pause Register"]
pub mod tpfcp;
#[doc = "SAMB1 (rw) register accessor: an alias for `Reg<SAMB1_SPEC>`"]
pub type SAMB1 = crate::Reg<samb1::SAMB1_SPEC>;
#[doc = "Specific Address 1 Mask Bottom Register"]
pub mod samb1;
#[doc = "SAMT1 (rw) register accessor: an alias for `Reg<SAMT1_SPEC>`"]
pub type SAMT1 = crate::Reg<samt1::SAMT1_SPEC>;
#[doc = "Specific Address 1 Mask Top Register"]
pub mod samt1;
#[doc = "NSC (rw) register accessor: an alias for `Reg<NSC_SPEC>`"]
pub type NSC = crate::Reg<nsc::NSC_SPEC>;
#[doc = "1588 Timer Nanosecond Comparison Register"]
pub mod nsc;
#[doc = "SCL (rw) register accessor: an alias for `Reg<SCL_SPEC>`"]
pub type SCL = crate::Reg<scl::SCL_SPEC>;
#[doc = "1588 Timer Second Comparison Low Register"]
pub mod scl;
#[doc = "SCH (rw) register accessor: an alias for `Reg<SCH_SPEC>`"]
pub type SCH = crate::Reg<sch::SCH_SPEC>;
#[doc = "1588 Timer Second Comparison High Register"]
pub mod sch;
#[doc = "EFTSH (r) register accessor: an alias for `Reg<EFTSH_SPEC>`"]
pub type EFTSH = crate::Reg<eftsh::EFTSH_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds High Register"]
pub mod eftsh;
#[doc = "EFRSH (r) register accessor: an alias for `Reg<EFRSH_SPEC>`"]
pub type EFRSH = crate::Reg<efrsh::EFRSH_SPEC>;
#[doc = "PTP Event Frame Received Seconds High Register"]
pub mod efrsh;
#[doc = "PEFTSH (r) register accessor: an alias for `Reg<PEFTSH_SPEC>`"]
pub type PEFTSH = crate::Reg<peftsh::PEFTSH_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register"]
pub mod peftsh;
#[doc = "PEFRSH (r) register accessor: an alias for `Reg<PEFRSH_SPEC>`"]
pub type PEFRSH = crate::Reg<pefrsh::PEFRSH_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds High Register"]
pub mod pefrsh;
#[doc = "OTLO (r) register accessor: an alias for `Reg<OTLO_SPEC>`"]
pub type OTLO = crate::Reg<otlo::OTLO_SPEC>;
#[doc = "Octets Transmitted Low Register"]
pub mod otlo;
#[doc = "OTHI (r) register accessor: an alias for `Reg<OTHI_SPEC>`"]
pub type OTHI = crate::Reg<othi::OTHI_SPEC>;
#[doc = "Octets Transmitted High Register"]
pub mod othi;
#[doc = "FT (r) register accessor: an alias for `Reg<FT_SPEC>`"]
pub type FT = crate::Reg<ft::FT_SPEC>;
#[doc = "Frames Transmitted Register"]
pub mod ft;
#[doc = "BCFT (r) register accessor: an alias for `Reg<BCFT_SPEC>`"]
pub type BCFT = crate::Reg<bcft::BCFT_SPEC>;
#[doc = "Broadcast Frames Transmitted Register"]
pub mod bcft;
#[doc = "MFT (r) register accessor: an alias for `Reg<MFT_SPEC>`"]
pub type MFT = crate::Reg<mft::MFT_SPEC>;
#[doc = "Multicast Frames Transmitted Register"]
pub mod mft;
#[doc = "PFT (r) register accessor: an alias for `Reg<PFT_SPEC>`"]
pub type PFT = crate::Reg<pft::PFT_SPEC>;
#[doc = "Pause Frames Transmitted Register"]
pub mod pft;
#[doc = "BFT64 (r) register accessor: an alias for `Reg<BFT64_SPEC>`"]
pub type BFT64 = crate::Reg<bft64::BFT64_SPEC>;
#[doc = "64 Byte Frames Transmitted Register"]
pub mod bft64;
#[doc = "TBFT127 (r) register accessor: an alias for `Reg<TBFT127_SPEC>`"]
pub type TBFT127 = crate::Reg<tbft127::TBFT127_SPEC>;
#[doc = "65 to 127 Byte Frames Transmitted Register"]
pub mod tbft127;
#[doc = "TBFT255 (r) register accessor: an alias for `Reg<TBFT255_SPEC>`"]
pub type TBFT255 = crate::Reg<tbft255::TBFT255_SPEC>;
#[doc = "128 to 255 Byte Frames Transmitted Register"]
pub mod tbft255;
#[doc = "TBFT511 (r) register accessor: an alias for `Reg<TBFT511_SPEC>`"]
pub type TBFT511 = crate::Reg<tbft511::TBFT511_SPEC>;
#[doc = "256 to 511 Byte Frames Transmitted Register"]
pub mod tbft511;
#[doc = "TBFT1023 (r) register accessor: an alias for `Reg<TBFT1023_SPEC>`"]
pub type TBFT1023 = crate::Reg<tbft1023::TBFT1023_SPEC>;
#[doc = "512 to 1023 Byte Frames Transmitted Register"]
pub mod tbft1023;
#[doc = "TBFT1518 (r) register accessor: an alias for `Reg<TBFT1518_SPEC>`"]
pub type TBFT1518 = crate::Reg<tbft1518::TBFT1518_SPEC>;
#[doc = "1024 to 1518 Byte Frames Transmitted Register"]
pub mod tbft1518;
#[doc = "GTBFT1518 (r) register accessor: an alias for `Reg<GTBFT1518_SPEC>`"]
pub type GTBFT1518 = crate::Reg<gtbft1518::GTBFT1518_SPEC>;
#[doc = "Greater Than 1518 Byte Frames Transmitted Register"]
pub mod gtbft1518;
#[doc = "TUR (r) register accessor: an alias for `Reg<TUR_SPEC>`"]
pub type TUR = crate::Reg<tur::TUR_SPEC>;
#[doc = "Transmit Underruns Register"]
pub mod tur;
#[doc = "SCF (r) register accessor: an alias for `Reg<SCF_SPEC>`"]
pub type SCF = crate::Reg<scf::SCF_SPEC>;
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "MCF (r) register accessor: an alias for `Reg<MCF_SPEC>`"]
pub type MCF = crate::Reg<mcf::MCF_SPEC>;
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "EC (r) register accessor: an alias for `Reg<EC_SPEC>`"]
pub type EC = crate::Reg<ec::EC_SPEC>;
#[doc = "Excessive Collisions Register"]
pub mod ec;
#[doc = "LC (r) register accessor: an alias for `Reg<LC_SPEC>`"]
pub type LC = crate::Reg<lc::LC_SPEC>;
#[doc = "Late Collisions Register"]
pub mod lc;
#[doc = "DTF (r) register accessor: an alias for `Reg<DTF_SPEC>`"]
pub type DTF = crate::Reg<dtf::DTF_SPEC>;
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "CSE (r) register accessor: an alias for `Reg<CSE_SPEC>`"]
pub type CSE = crate::Reg<cse::CSE_SPEC>;
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "ORLO (r) register accessor: an alias for `Reg<ORLO_SPEC>`"]
pub type ORLO = crate::Reg<orlo::ORLO_SPEC>;
#[doc = "Octets Received Low Received Register"]
pub mod orlo;
#[doc = "ORHI (r) register accessor: an alias for `Reg<ORHI_SPEC>`"]
pub type ORHI = crate::Reg<orhi::ORHI_SPEC>;
#[doc = "Octets Received High Received Register"]
pub mod orhi;
#[doc = "FR (r) register accessor: an alias for `Reg<FR_SPEC>`"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "Frames Received Register"]
pub mod fr;
#[doc = "BCFR (r) register accessor: an alias for `Reg<BCFR_SPEC>`"]
pub type BCFR = crate::Reg<bcfr::BCFR_SPEC>;
#[doc = "Broadcast Frames Received Register"]
pub mod bcfr;
#[doc = "MFR (r) register accessor: an alias for `Reg<MFR_SPEC>`"]
pub type MFR = crate::Reg<mfr::MFR_SPEC>;
#[doc = "Multicast Frames Received Register"]
pub mod mfr;
#[doc = "PFR (r) register accessor: an alias for `Reg<PFR_SPEC>`"]
pub type PFR = crate::Reg<pfr::PFR_SPEC>;
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "BFR64 (r) register accessor: an alias for `Reg<BFR64_SPEC>`"]
pub type BFR64 = crate::Reg<bfr64::BFR64_SPEC>;
#[doc = "64 Byte Frames Received Register"]
pub mod bfr64;
#[doc = "TBFR127 (r) register accessor: an alias for `Reg<TBFR127_SPEC>`"]
pub type TBFR127 = crate::Reg<tbfr127::TBFR127_SPEC>;
#[doc = "65 to 127 Byte Frames Received Register"]
pub mod tbfr127;
#[doc = "TBFR255 (r) register accessor: an alias for `Reg<TBFR255_SPEC>`"]
pub type TBFR255 = crate::Reg<tbfr255::TBFR255_SPEC>;
#[doc = "128 to 255 Byte Frames Received Register"]
pub mod tbfr255;
#[doc = "TBFR511 (r) register accessor: an alias for `Reg<TBFR511_SPEC>`"]
pub type TBFR511 = crate::Reg<tbfr511::TBFR511_SPEC>;
#[doc = "256 to 511 Byte Frames Received Register"]
pub mod tbfr511;
#[doc = "TBFR1023 (r) register accessor: an alias for `Reg<TBFR1023_SPEC>`"]
pub type TBFR1023 = crate::Reg<tbfr1023::TBFR1023_SPEC>;
#[doc = "512 to 1023 Byte Frames Received Register"]
pub mod tbfr1023;
#[doc = "TBFR1518 (r) register accessor: an alias for `Reg<TBFR1518_SPEC>`"]
pub type TBFR1518 = crate::Reg<tbfr1518::TBFR1518_SPEC>;
#[doc = "1024 to 1518 Byte Frames Received Register"]
pub mod tbfr1518;
#[doc = "TMXBFR (r) register accessor: an alias for `Reg<TMXBFR_SPEC>`"]
pub type TMXBFR = crate::Reg<tmxbfr::TMXBFR_SPEC>;
#[doc = "1519 to Maximum Byte Frames Received Register"]
pub mod tmxbfr;
#[doc = "UFR (r) register accessor: an alias for `Reg<UFR_SPEC>`"]
pub type UFR = crate::Reg<ufr::UFR_SPEC>;
#[doc = "Undersize Frames Received Register"]
pub mod ufr;
#[doc = "OFR (r) register accessor: an alias for `Reg<OFR_SPEC>`"]
pub type OFR = crate::Reg<ofr::OFR_SPEC>;
#[doc = "Oversize Frames Received Register"]
pub mod ofr;
#[doc = "JR (r) register accessor: an alias for `Reg<JR_SPEC>`"]
pub type JR = crate::Reg<jr::JR_SPEC>;
#[doc = "Jabbers Received Register"]
pub mod jr;
#[doc = "FCSE (r) register accessor: an alias for `Reg<FCSE_SPEC>`"]
pub type FCSE = crate::Reg<fcse::FCSE_SPEC>;
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "LFFE (r) register accessor: an alias for `Reg<LFFE_SPEC>`"]
pub type LFFE = crate::Reg<lffe::LFFE_SPEC>;
#[doc = "Length Field Frame Errors Register"]
pub mod lffe;
#[doc = "RSE (r) register accessor: an alias for `Reg<RSE_SPEC>`"]
pub type RSE = crate::Reg<rse::RSE_SPEC>;
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "AE (r) register accessor: an alias for `Reg<AE_SPEC>`"]
pub type AE = crate::Reg<ae::AE_SPEC>;
#[doc = "Alignment Errors Register"]
pub mod ae;
#[doc = "RRE (r) register accessor: an alias for `Reg<RRE_SPEC>`"]
pub type RRE = crate::Reg<rre::RRE_SPEC>;
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "ROE (r) register accessor: an alias for `Reg<ROE_SPEC>`"]
pub type ROE = crate::Reg<roe::ROE_SPEC>;
#[doc = "Receive Overrun Register"]
pub mod roe;
#[doc = "IHCE (r) register accessor: an alias for `Reg<IHCE_SPEC>`"]
pub type IHCE = crate::Reg<ihce::IHCE_SPEC>;
#[doc = "IP Header Checksum Errors Register"]
pub mod ihce;
#[doc = "TCE (r) register accessor: an alias for `Reg<TCE_SPEC>`"]
pub type TCE = crate::Reg<tce::TCE_SPEC>;
#[doc = "TCP Checksum Errors Register"]
pub mod tce;
#[doc = "UCE (r) register accessor: an alias for `Reg<UCE_SPEC>`"]
pub type UCE = crate::Reg<uce::UCE_SPEC>;
#[doc = "UDP Checksum Errors Register"]
pub mod uce;
#[doc = "TISUBN (rw) register accessor: an alias for `Reg<TISUBN_SPEC>`"]
pub type TISUBN = crate::Reg<tisubn::TISUBN_SPEC>;
#[doc = "1588 Timer Increment Sub-nanoseconds Register"]
pub mod tisubn;
#[doc = "TSH (rw) register accessor: an alias for `Reg<TSH_SPEC>`"]
pub type TSH = crate::Reg<tsh::TSH_SPEC>;
#[doc = "1588 Timer Seconds High Register"]
pub mod tsh;
#[doc = "TSL (rw) register accessor: an alias for `Reg<TSL_SPEC>`"]
pub type TSL = crate::Reg<tsl::TSL_SPEC>;
#[doc = "1588 Timer Seconds Low Register"]
pub mod tsl;
#[doc = "TN (rw) register accessor: an alias for `Reg<TN_SPEC>`"]
pub type TN = crate::Reg<tn::TN_SPEC>;
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tn;
#[doc = "TA (w) register accessor: an alias for `Reg<TA_SPEC>`"]
pub type TA = crate::Reg<ta::TA_SPEC>;
#[doc = "1588 Timer Adjust Register"]
pub mod ta;
#[doc = "TI (rw) register accessor: an alias for `Reg<TI_SPEC>`"]
pub type TI = crate::Reg<ti::TI_SPEC>;
#[doc = "1588 Timer Increment Register"]
pub mod ti;
#[doc = "EFTSL (r) register accessor: an alias for `Reg<EFTSL_SPEC>`"]
pub type EFTSL = crate::Reg<eftsl::EFTSL_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Low Register"]
pub mod eftsl;
#[doc = "EFTN (r) register accessor: an alias for `Reg<EFTN_SPEC>`"]
pub type EFTN = crate::Reg<eftn::EFTN_SPEC>;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod eftn;
#[doc = "EFRSL (r) register accessor: an alias for `Reg<EFRSL_SPEC>`"]
pub type EFRSL = crate::Reg<efrsl::EFRSL_SPEC>;
#[doc = "PTP Event Frame Received Seconds Low Register"]
pub mod efrsl;
#[doc = "EFRN (r) register accessor: an alias for `Reg<EFRN_SPEC>`"]
pub type EFRN = crate::Reg<efrn::EFRN_SPEC>;
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod efrn;
#[doc = "PEFTSL (r) register accessor: an alias for `Reg<PEFTSL_SPEC>`"]
pub type PEFTSL = crate::Reg<peftsl::PEFTSL_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register"]
pub mod peftsl;
#[doc = "PEFTN (r) register accessor: an alias for `Reg<PEFTN_SPEC>`"]
pub type PEFTN = crate::Reg<peftn::PEFTN_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod peftn;
#[doc = "PEFRSL (r) register accessor: an alias for `Reg<PEFRSL_SPEC>`"]
pub type PEFRSL = crate::Reg<pefrsl::PEFRSL_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Low Register"]
pub mod pefrsl;
#[doc = "PEFRN (r) register accessor: an alias for `Reg<PEFRN_SPEC>`"]
pub type PEFRN = crate::Reg<pefrn::PEFRN_SPEC>;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod pefrn;
#[doc = "RXLPI (r) register accessor: an alias for `Reg<RXLPI_SPEC>`"]
pub type RXLPI = crate::Reg<rxlpi::RXLPI_SPEC>;
#[doc = "Received LPI Transitions"]
pub mod rxlpi;
#[doc = "RXLPITIME (r) register accessor: an alias for `Reg<RXLPITIME_SPEC>`"]
pub type RXLPITIME = crate::Reg<rxlpitime::RXLPITIME_SPEC>;
#[doc = "Received LPI Time"]
pub mod rxlpitime;
#[doc = "TXLPI (r) register accessor: an alias for `Reg<TXLPI_SPEC>`"]
pub type TXLPI = crate::Reg<txlpi::TXLPI_SPEC>;
#[doc = "Transmit LPI Transitions"]
pub mod txlpi;
#[doc = "TXLPITIME (r) register accessor: an alias for `Reg<TXLPITIME_SPEC>`"]
pub type TXLPITIME = crate::Reg<txlpitime::TXLPITIME_SPEC>;
#[doc = "Transmit LPI Time"]
pub mod txlpitime;
#[doc = "ISRPQ (r) register accessor: an alias for `Reg<ISRPQ_SPEC>`"]
pub type ISRPQ = crate::Reg<isrpq::ISRPQ_SPEC>;
#[doc = "Interrupt Status Register Priority Queue (1..5)"]
pub mod isrpq;
#[doc = "TBQBAPQ (rw) register accessor: an alias for `Reg<TBQBAPQ_SPEC>`"]
pub type TBQBAPQ = crate::Reg<tbqbapq::TBQBAPQ_SPEC>;
#[doc = "Transmit Buffer Queue Base Address Register Priority Queue (1..5)"]
pub mod tbqbapq;
#[doc = "RBQBAPQ (rw) register accessor: an alias for `Reg<RBQBAPQ_SPEC>`"]
pub type RBQBAPQ = crate::Reg<rbqbapq::RBQBAPQ_SPEC>;
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (1..5)"]
pub mod rbqbapq;
#[doc = "RBSRPQ (rw) register accessor: an alias for `Reg<RBSRPQ_SPEC>`"]
pub type RBSRPQ = crate::Reg<rbsrpq::RBSRPQ_SPEC>;
#[doc = "Receive Buffer Size Register Priority Queue (1..5)"]
pub mod rbsrpq;
#[doc = "CBSCR (rw) register accessor: an alias for `Reg<CBSCR_SPEC>`"]
pub type CBSCR = crate::Reg<cbscr::CBSCR_SPEC>;
#[doc = "Credit-Based Shaping Control Register"]
pub mod cbscr;
#[doc = "CBSISQA (rw) register accessor: an alias for `Reg<CBSISQA_SPEC>`"]
pub type CBSISQA = crate::Reg<cbsisqa::CBSISQA_SPEC>;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue A"]
pub mod cbsisqa;
#[doc = "CBSISQB (rw) register accessor: an alias for `Reg<CBSISQB_SPEC>`"]
pub type CBSISQB = crate::Reg<cbsisqb::CBSISQB_SPEC>;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue B"]
pub mod cbsisqb;
#[doc = "ST1RPQ (rw) register accessor: an alias for `Reg<ST1RPQ_SPEC>`"]
pub type ST1RPQ = crate::Reg<st1rpq::ST1RPQ_SPEC>;
#[doc = "Screening Type 1 Register Priority Queue"]
pub mod st1rpq;
#[doc = "ST2RPQ (rw) register accessor: an alias for `Reg<ST2RPQ_SPEC>`"]
pub type ST2RPQ = crate::Reg<st2rpq::ST2RPQ_SPEC>;
#[doc = "Screening Type 2 Register Priority Queue"]
pub mod st2rpq;
#[doc = "IERPQ (w) register accessor: an alias for `Reg<IERPQ_SPEC>`"]
pub type IERPQ = crate::Reg<ierpq::IERPQ_SPEC>;
#[doc = "Interrupt Enable Register Priority Queue (1..5)"]
pub mod ierpq;
#[doc = "IDRPQ (w) register accessor: an alias for `Reg<IDRPQ_SPEC>`"]
pub type IDRPQ = crate::Reg<idrpq::IDRPQ_SPEC>;
#[doc = "Interrupt Disable Register Priority Queue (1..5)"]
pub mod idrpq;
#[doc = "IMRPQ (rw) register accessor: an alias for `Reg<IMRPQ_SPEC>`"]
pub type IMRPQ = crate::Reg<imrpq::IMRPQ_SPEC>;
#[doc = "Interrupt Mask Register Priority Queue (1..5)"]
pub mod imrpq;
#[doc = "ST2ER (rw) register accessor: an alias for `Reg<ST2ER_SPEC>`"]
pub type ST2ER = crate::Reg<st2er::ST2ER_SPEC>;
#[doc = "Screening Type 2 Ethertype Register"]
pub mod st2er;
#[doc = "Screening Type 2 Compare Word 0 Register"]
pub use self::gmac_st2cw::GMAC_ST2CW;
#[doc = r"Cluster"]
#[doc = "Screening Type 2 Compare Word 0 Register"]
pub mod gmac_st2cw;
