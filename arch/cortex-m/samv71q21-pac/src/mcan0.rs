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
#[doc = "CREL (r) register accessor: Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crel`]
module"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "Core Release Register"]
pub mod crel;
#[doc = "ENDN (r) register accessor: Endian Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`endn`]
module"]
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
#[doc = "Endian Register"]
pub mod endn;
#[doc = "CUST (rw) register accessor: Customer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cust::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cust::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cust`]
module"]
pub type CUST = crate::Reg<cust::CUST_SPEC>;
#[doc = "Customer Register"]
pub mod cust;
#[doc = "DBTP (rw) register accessor: Data Bit Timing and Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dbtp`]
module"]
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
#[doc = "Data Bit Timing and Prescaler Register"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: Test Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`test`]
module"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test Register"]
pub mod test;
#[doc = "RWD (rw) register accessor: RAM Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rwd`]
module"]
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
#[doc = "RAM Watchdog Register"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: CC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cccr`]
module"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "CC Control Register"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: Nominal Bit Timing and Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`nbtp`]
module"]
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
#[doc = "Nominal Bit Timing and Prescaler Register"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: Timestamp Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tscc`]
module"]
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
#[doc = "Timestamp Counter Configuration Register"]
pub mod tscc;
#[doc = "TSCV (rw) register accessor: Timestamp Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tscv`]
module"]
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
#[doc = "Timestamp Counter Value Register"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: Timeout Counter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tocc`]
module"]
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
#[doc = "Timeout Counter Configuration Register"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: Timeout Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tocv`]
module"]
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
#[doc = "Timeout Counter Value Register"]
pub mod tocv;
#[doc = "ECR (r) register accessor: Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ecr`]
module"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "PSR (r) register accessor: Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`psr`]
module"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Protocol Status Register"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: Transmit Delay Compensation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tdcr`]
module"]
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
#[doc = "Transmit Delay Compensation Register"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ir`]
module"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt Register"]
pub mod ir;
#[doc = "IE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ie`]
module"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "ILS (rw) register accessor: Interrupt Line Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ils`]
module"]
pub type ILS = crate::Reg<ils::ILS_SPEC>;
#[doc = "Interrupt Line Select Register"]
pub mod ils;
#[doc = "ILE (rw) register accessor: Interrupt Line Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ile`]
module"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "Interrupt Line Enable Register"]
pub mod ile;
#[doc = "GFC (rw) register accessor: Global Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gfc`]
module"]
pub type GFC = crate::Reg<gfc::GFC_SPEC>;
#[doc = "Global Filter Configuration Register"]
pub mod gfc;
#[doc = "SIDFC (rw) register accessor: Standard ID Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sidfc`]
module"]
pub type SIDFC = crate::Reg<sidfc::SIDFC_SPEC>;
#[doc = "Standard ID Filter Configuration Register"]
pub mod sidfc;
#[doc = "XIDFC (rw) register accessor: Extended ID Filter Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`xidfc`]
module"]
pub type XIDFC = crate::Reg<xidfc::XIDFC_SPEC>;
#[doc = "Extended ID Filter Configuration Register"]
pub mod xidfc;
#[doc = "XIDAM (rw) register accessor: Extended ID AND Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`xidam`]
module"]
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
#[doc = "Extended ID AND Mask Register"]
pub mod xidam;
#[doc = "HPMS (r) register accessor: High Priority Message Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hpms`]
module"]
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
#[doc = "High Priority Message Status Register"]
pub mod hpms;
#[doc = "NDAT1 (rw) register accessor: New Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ndat1`]
module"]
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
#[doc = "New Data 1 Register"]
pub mod ndat1;
#[doc = "NDAT2 (rw) register accessor: New Data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ndat2`]
module"]
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
#[doc = "New Data 2 Register"]
pub mod ndat2;
#[doc = "RXF0C (rw) register accessor: Receive FIFO 0 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxf0c`]
module"]
pub type RXF0C = crate::Reg<rxf0c::RXF0C_SPEC>;
#[doc = "Receive FIFO 0 Configuration Register"]
pub mod rxf0c;
#[doc = "RXF0S (r) register accessor: Receive FIFO 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxf0s`]
module"]
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
#[doc = "Receive FIFO 0 Status Register"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: Receive FIFO 0 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxf0a`]
module"]
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
#[doc = "Receive FIFO 0 Acknowledge Register"]
pub mod rxf0a;
#[doc = "RXBC (rw) register accessor: Receive Rx Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxbc`]
module"]
pub type RXBC = crate::Reg<rxbc::RXBC_SPEC>;
#[doc = "Receive Rx Buffer Configuration Register"]
pub mod rxbc;
#[doc = "RXF1C (rw) register accessor: Receive FIFO 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxf1c`]
module"]
pub type RXF1C = crate::Reg<rxf1c::RXF1C_SPEC>;
#[doc = "Receive FIFO 1 Configuration Register"]
pub mod rxf1c;
#[doc = "RXF1S (r) register accessor: Receive FIFO 1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxf1s`]
module"]
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
#[doc = "Receive FIFO 1 Status Register"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: Receive FIFO 1 Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxf1a`]
module"]
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
#[doc = "Receive FIFO 1 Acknowledge Register"]
pub mod rxf1a;
#[doc = "RXESC (rw) register accessor: Receive Buffer / FIFO Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxesc`]
module"]
pub type RXESC = crate::Reg<rxesc::RXESC_SPEC>;
#[doc = "Receive Buffer / FIFO Element Size Configuration Register"]
pub mod rxesc;
#[doc = "TXBC (rw) register accessor: Transmit Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txbc`]
module"]
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
#[doc = "Transmit Buffer Configuration Register"]
pub mod txbc;
#[doc = "TXFQS (r) register accessor: Transmit FIFO/Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfqs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txfqs`]
module"]
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
#[doc = "Transmit FIFO/Queue Status Register"]
pub mod txfqs;
#[doc = "TXESC (rw) register accessor: Transmit Buffer Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txesc`]
module"]
pub type TXESC = crate::Reg<txesc::TXESC_SPEC>;
#[doc = "Transmit Buffer Element Size Configuration Register"]
pub mod txesc;
#[doc = "TXBRP (r) register accessor: Transmit Buffer Request Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbrp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txbrp`]
module"]
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
#[doc = "Transmit Buffer Request Pending Register"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: Transmit Buffer Add Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txbar`]
module"]
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
#[doc = "Transmit Buffer Add Request Register"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: Transmit Buffer Cancellation Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txbcr`]
module"]
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
#[doc = "Transmit Buffer Cancellation Request Register"]
pub mod txbcr;
#[doc = "TXBTO (r) register accessor: Transmit Buffer Transmission Occurred Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbto::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txbto`]
module"]
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
#[doc = "Transmit Buffer Transmission Occurred Register"]
pub mod txbto;
#[doc = "TXBCF (r) register accessor: Transmit Buffer Cancellation Finished Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txbcf`]
module"]
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
#[doc = "Transmit Buffer Cancellation Finished Register"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: Transmit Buffer Transmission Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbtie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txbtie`]
module"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
#[doc = "Transmit Buffer Transmission Interrupt Enable Register"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: Transmit Buffer Cancellation Finished Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txbcie`]
module"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
#[doc = "Transmit Buffer Cancellation Finished Interrupt Enable Register"]
pub mod txbcie;
#[doc = "TXEFC (rw) register accessor: Transmit Event FIFO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txefc`]
module"]
pub type TXEFC = crate::Reg<txefc::TXEFC_SPEC>;
#[doc = "Transmit Event FIFO Configuration Register"]
pub mod txefc;
#[doc = "TXEFS (r) register accessor: Transmit Event FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txefs`]
module"]
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
#[doc = "Transmit Event FIFO Status Register"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: Transmit Event FIFO Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txefa`]
module"]
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
#[doc = "Transmit Event FIFO Acknowledge Register"]
pub mod txefa;
