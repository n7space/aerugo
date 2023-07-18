#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Core Release Register"]
    pub crel: CREL,
    #[doc = "0x04 - Endian Register"]
    pub endn: ENDN,
    #[doc = "0x08 - Customer Register"]
    pub cust: CUST,
    #[doc = "0x0c - Data Bit Timing and Prescaler Register"]
    pub dbtp: DBTP,
    #[doc = "0x10 - Test Register"]
    pub test: TEST,
    #[doc = "0x14 - RAM Watchdog Register"]
    pub rwd: RWD,
    #[doc = "0x18 - CC Control Register"]
    pub cccr: CCCR,
    #[doc = "0x1c - Nominal Bit Timing and Prescaler Register"]
    pub nbtp: NBTP,
    #[doc = "0x20 - Timestamp Counter Configuration Register"]
    pub tscc: TSCC,
    #[doc = "0x24 - Timestamp Counter Value Register"]
    pub tscv: TSCV,
    #[doc = "0x28 - Timeout Counter Configuration Register"]
    pub tocc: TOCC,
    #[doc = "0x2c - Timeout Counter Value Register"]
    pub tocv: TOCV,
    _reserved12: [u8; 0x10],
    #[doc = "0x40 - Error Counter Register"]
    pub ecr: ECR,
    #[doc = "0x44 - Protocol Status Register"]
    pub psr: PSR,
    #[doc = "0x48 - Transmit Delay Compensation Register"]
    pub tdcr: TDCR,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - Interrupt Register"]
    pub ir: IR,
    #[doc = "0x54 - Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x58 - Interrupt Line Select Register"]
    pub ils: ILS,
    #[doc = "0x5c - Interrupt Line Enable Register"]
    pub ile: ILE,
    _reserved19: [u8; 0x20],
    #[doc = "0x80 - Global Filter Configuration Register"]
    pub gfc: GFC,
    #[doc = "0x84 - Standard ID Filter Configuration Register"]
    pub sidfc: SIDFC,
    #[doc = "0x88 - Extended ID Filter Configuration Register"]
    pub xidfc: XIDFC,
    _reserved22: [u8; 0x04],
    #[doc = "0x90 - Extended ID AND Mask Register"]
    pub xidam: XIDAM,
    #[doc = "0x94 - High Priority Message Status Register"]
    pub hpms: HPMS,
    #[doc = "0x98 - New Data 1 Register"]
    pub ndat1: NDAT1,
    #[doc = "0x9c - New Data 2 Register"]
    pub ndat2: NDAT2,
    #[doc = "0xa0 - Receive FIFO 0 Configuration Register"]
    pub rxf0c: RXF0C,
    #[doc = "0xa4 - Receive FIFO 0 Status Register"]
    pub rxf0s: RXF0S,
    #[doc = "0xa8 - Receive FIFO 0 Acknowledge Register"]
    pub rxf0a: RXF0A,
    #[doc = "0xac - Receive Rx Buffer Configuration Register"]
    pub rxbc: RXBC,
    #[doc = "0xb0 - Receive FIFO 1 Configuration Register"]
    pub rxf1c: RXF1C,
    #[doc = "0xb4 - Receive FIFO 1 Status Register"]
    pub rxf1s: RXF1S,
    #[doc = "0xb8 - Receive FIFO 1 Acknowledge Register"]
    pub rxf1a: RXF1A,
    #[doc = "0xbc - Receive Buffer / FIFO Element Size Configuration Register"]
    pub rxesc: RXESC,
    #[doc = "0xc0 - Transmit Buffer Configuration Register"]
    pub txbc: TXBC,
    #[doc = "0xc4 - Transmit FIFO/Queue Status Register"]
    pub txfqs: TXFQS,
    #[doc = "0xc8 - Transmit Buffer Element Size Configuration Register"]
    pub txesc: TXESC,
    #[doc = "0xcc - Transmit Buffer Request Pending Register"]
    pub txbrp: TXBRP,
    #[doc = "0xd0 - Transmit Buffer Add Request Register"]
    pub txbar: TXBAR,
    #[doc = "0xd4 - Transmit Buffer Cancellation Request Register"]
    pub txbcr: TXBCR,
    #[doc = "0xd8 - Transmit Buffer Transmission Occurred Register"]
    pub txbto: TXBTO,
    #[doc = "0xdc - Transmit Buffer Cancellation Finished Register"]
    pub txbcf: TXBCF,
    #[doc = "0xe0 - Transmit Buffer Transmission Interrupt Enable Register"]
    pub txbtie: TXBTIE,
    #[doc = "0xe4 - Transmit Buffer Cancellation Finished Interrupt Enable Register"]
    pub txbcie: TXBCIE,
    _reserved44: [u8; 0x08],
    #[doc = "0xf0 - Transmit Event FIFO Configuration Register"]
    pub txefc: TXEFC,
    #[doc = "0xf4 - Transmit Event FIFO Status Register"]
    pub txefs: TXEFS,
    #[doc = "0xf8 - Transmit Event FIFO Acknowledge Register"]
    pub txefa: TXEFA,
}
#[doc = "CREL (r) register accessor: an alias for `Reg<CREL_SPEC>`"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "Core Release Register"]
pub mod crel;
#[doc = "ENDN (r) register accessor: an alias for `Reg<ENDN_SPEC>`"]
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
#[doc = "Endian Register"]
pub mod endn;
#[doc = "CUST (rw) register accessor: an alias for `Reg<CUST_SPEC>`"]
pub type CUST = crate::Reg<cust::CUST_SPEC>;
#[doc = "Customer Register"]
pub mod cust;
#[doc = "DBTP (rw) register accessor: an alias for `Reg<DBTP_SPEC>`"]
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
#[doc = "Data Bit Timing and Prescaler Register"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test Register"]
pub mod test;
#[doc = "RWD (rw) register accessor: an alias for `Reg<RWD_SPEC>`"]
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
#[doc = "RAM Watchdog Register"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: an alias for `Reg<CCCR_SPEC>`"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "CC Control Register"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: an alias for `Reg<NBTP_SPEC>`"]
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
#[doc = "Nominal Bit Timing and Prescaler Register"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: an alias for `Reg<TSCC_SPEC>`"]
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
#[doc = "Timestamp Counter Configuration Register"]
pub mod tscc;
#[doc = "TSCV (rw) register accessor: an alias for `Reg<TSCV_SPEC>`"]
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
#[doc = "Timestamp Counter Value Register"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: an alias for `Reg<TOCC_SPEC>`"]
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
#[doc = "Timeout Counter Configuration Register"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: an alias for `Reg<TOCV_SPEC>`"]
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
#[doc = "Timeout Counter Value Register"]
pub mod tocv;
#[doc = "ECR (r) register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "PSR (r) register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Protocol Status Register"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: an alias for `Reg<TDCR_SPEC>`"]
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
#[doc = "Transmit Delay Compensation Register"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt Register"]
pub mod ir;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "ILS (rw) register accessor: an alias for `Reg<ILS_SPEC>`"]
pub type ILS = crate::Reg<ils::ILS_SPEC>;
#[doc = "Interrupt Line Select Register"]
pub mod ils;
#[doc = "ILE (rw) register accessor: an alias for `Reg<ILE_SPEC>`"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "Interrupt Line Enable Register"]
pub mod ile;
#[doc = "GFC (rw) register accessor: an alias for `Reg<GFC_SPEC>`"]
pub type GFC = crate::Reg<gfc::GFC_SPEC>;
#[doc = "Global Filter Configuration Register"]
pub mod gfc;
#[doc = "SIDFC (rw) register accessor: an alias for `Reg<SIDFC_SPEC>`"]
pub type SIDFC = crate::Reg<sidfc::SIDFC_SPEC>;
#[doc = "Standard ID Filter Configuration Register"]
pub mod sidfc;
#[doc = "XIDFC (rw) register accessor: an alias for `Reg<XIDFC_SPEC>`"]
pub type XIDFC = crate::Reg<xidfc::XIDFC_SPEC>;
#[doc = "Extended ID Filter Configuration Register"]
pub mod xidfc;
#[doc = "XIDAM (rw) register accessor: an alias for `Reg<XIDAM_SPEC>`"]
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
#[doc = "Extended ID AND Mask Register"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: an alias for `Reg<HPMS_SPEC>`"]
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
#[doc = "High Priority Message Status Register"]
pub mod hpms;
#[doc = "NDAT1 (rw) register accessor: an alias for `Reg<NDAT1_SPEC>`"]
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
#[doc = "New Data 1 Register"]
pub mod ndat1;
#[doc = "NDAT2 (rw) register accessor: an alias for `Reg<NDAT2_SPEC>`"]
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
#[doc = "New Data 2 Register"]
pub mod ndat2;
#[doc = "RXF0C (rw) register accessor: an alias for `Reg<RXF0C_SPEC>`"]
pub type RXF0C = crate::Reg<rxf0c::RXF0C_SPEC>;
#[doc = "Receive FIFO 0 Configuration Register"]
pub mod rxf0c;
#[doc = "RXF0S (r) register accessor: an alias for `Reg<RXF0S_SPEC>`"]
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
#[doc = "Receive FIFO 0 Status Register"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: an alias for `Reg<RXF0A_SPEC>`"]
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
#[doc = "Receive FIFO 0 Acknowledge Register"]
pub mod rxf0a;
#[doc = "RXBC (rw) register accessor: an alias for `Reg<RXBC_SPEC>`"]
pub type RXBC = crate::Reg<rxbc::RXBC_SPEC>;
#[doc = "Receive Rx Buffer Configuration Register"]
pub mod rxbc;
#[doc = "RXF1C (rw) register accessor: an alias for `Reg<RXF1C_SPEC>`"]
pub type RXF1C = crate::Reg<rxf1c::RXF1C_SPEC>;
#[doc = "Receive FIFO 1 Configuration Register"]
pub mod rxf1c;
#[doc = "RXF1S (r) register accessor: an alias for `Reg<RXF1S_SPEC>`"]
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
#[doc = "Receive FIFO 1 Status Register"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: an alias for `Reg<RXF1A_SPEC>`"]
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
#[doc = "Receive FIFO 1 Acknowledge Register"]
pub mod rxf1a;
#[doc = "RXESC (rw) register accessor: an alias for `Reg<RXESC_SPEC>`"]
pub type RXESC = crate::Reg<rxesc::RXESC_SPEC>;
#[doc = "Receive Buffer / FIFO Element Size Configuration Register"]
pub mod rxesc;
#[doc = "TXBC (rw) register accessor: an alias for `Reg<TXBC_SPEC>`"]
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
#[doc = "Transmit Buffer Configuration Register"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: an alias for `Reg<TXFQS_SPEC>`"]
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
#[doc = "Transmit FIFO/Queue Status Register"]
pub mod txfqs;
#[doc = "TXESC (rw) register accessor: an alias for `Reg<TXESC_SPEC>`"]
pub type TXESC = crate::Reg<txesc::TXESC_SPEC>;
#[doc = "Transmit Buffer Element Size Configuration Register"]
pub mod txesc;
#[doc = "TXBRP (r) register accessor: an alias for `Reg<TXBRP_SPEC>`"]
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
#[doc = "Transmit Buffer Request Pending Register"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: an alias for `Reg<TXBAR_SPEC>`"]
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
#[doc = "Transmit Buffer Add Request Register"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: an alias for `Reg<TXBCR_SPEC>`"]
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
#[doc = "Transmit Buffer Cancellation Request Register"]
pub mod txbcr;
#[doc = "TXBTO (r) register accessor: an alias for `Reg<TXBTO_SPEC>`"]
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
#[doc = "Transmit Buffer Transmission Occurred Register"]
pub mod txbto;
#[doc = "TXBCF (r) register accessor: an alias for `Reg<TXBCF_SPEC>`"]
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
#[doc = "Transmit Buffer Cancellation Finished Register"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: an alias for `Reg<TXBTIE_SPEC>`"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
#[doc = "Transmit Buffer Transmission Interrupt Enable Register"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: an alias for `Reg<TXBCIE_SPEC>`"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
#[doc = "Transmit Buffer Cancellation Finished Interrupt Enable Register"]
pub mod txbcie;
#[doc = "TXEFC (rw) register accessor: an alias for `Reg<TXEFC_SPEC>`"]
pub type TXEFC = crate::Reg<txefc::TXEFC_SPEC>;
#[doc = "Transmit Event FIFO Configuration Register"]
pub mod txefc;
#[doc = "TXEFS (r) register accessor: an alias for `Reg<TXEFS_SPEC>`"]
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
#[doc = "Transmit Event FIFO Status Register"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: an alias for `Reg<TXEFA_SPEC>`"]
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
#[doc = "Transmit Event FIFO Acknowledge Register"]
pub mod txefa;
