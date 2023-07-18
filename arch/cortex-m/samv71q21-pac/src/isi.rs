#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ISI Configuration 1 Register"]
    pub cfg1: CFG1,
    #[doc = "0x04 - ISI Configuration 2 Register"]
    pub cfg2: CFG2,
    #[doc = "0x08 - ISI Preview Size Register"]
    pub psize: PSIZE,
    #[doc = "0x0c - ISI Preview Decimation Factor Register"]
    pub pdecf: PDECF,
    #[doc = "0x10 - ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
    pub y2r_set0: Y2R_SET0,
    #[doc = "0x14 - ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
    pub y2r_set1: Y2R_SET1,
    #[doc = "0x18 - ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
    pub r2y_set0: R2Y_SET0,
    #[doc = "0x1c - ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
    pub r2y_set1: R2Y_SET1,
    #[doc = "0x20 - ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
    pub r2y_set2: R2Y_SET2,
    #[doc = "0x24 - ISI Control Register"]
    pub cr: CR,
    #[doc = "0x28 - ISI Status Register"]
    pub sr: SR,
    #[doc = "0x2c - ISI Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x30 - ISI Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x34 - ISI Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x38 - DMA Channel Enable Register"]
    pub dma_cher: DMA_CHER,
    #[doc = "0x3c - DMA Channel Disable Register"]
    pub dma_chdr: DMA_CHDR,
    #[doc = "0x40 - DMA Channel Status Register"]
    pub dma_chsr: DMA_CHSR,
    #[doc = "0x44 - DMA Preview Base Address Register"]
    pub dma_p_addr: DMA_P_ADDR,
    #[doc = "0x48 - DMA Preview Control Register"]
    pub dma_p_ctrl: DMA_P_CTRL,
    #[doc = "0x4c - DMA Preview Descriptor Address Register"]
    pub dma_p_dscr: DMA_P_DSCR,
    #[doc = "0x50 - DMA Codec Base Address Register"]
    pub dma_c_addr: DMA_C_ADDR,
    #[doc = "0x54 - DMA Codec Control Register"]
    pub dma_c_ctrl: DMA_C_CTRL,
    #[doc = "0x58 - DMA Codec Descriptor Address Register"]
    pub dma_c_dscr: DMA_C_DSCR,
    _reserved23: [u8; 0x88],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
#[doc = "CFG1 (rw) register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "ISI Configuration 1 Register"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "ISI Configuration 2 Register"]
pub mod cfg2;
#[doc = "PSIZE (rw) register accessor: an alias for `Reg<PSIZE_SPEC>`"]
pub type PSIZE = crate::Reg<psize::PSIZE_SPEC>;
#[doc = "ISI Preview Size Register"]
pub mod psize;
#[doc = "PDECF (rw) register accessor: an alias for `Reg<PDECF_SPEC>`"]
pub type PDECF = crate::Reg<pdecf::PDECF_SPEC>;
#[doc = "ISI Preview Decimation Factor Register"]
pub mod pdecf;
#[doc = "Y2R_SET0 (rw) register accessor: an alias for `Reg<Y2R_SET0_SPEC>`"]
pub type Y2R_SET0 = crate::Reg<y2r_set0::Y2R_SET0_SPEC>;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
pub mod y2r_set0;
#[doc = "Y2R_SET1 (rw) register accessor: an alias for `Reg<Y2R_SET1_SPEC>`"]
pub type Y2R_SET1 = crate::Reg<y2r_set1::Y2R_SET1_SPEC>;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
pub mod y2r_set1;
#[doc = "R2Y_SET0 (rw) register accessor: an alias for `Reg<R2Y_SET0_SPEC>`"]
pub type R2Y_SET0 = crate::Reg<r2y_set0::R2Y_SET0_SPEC>;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
pub mod r2y_set0;
#[doc = "R2Y_SET1 (rw) register accessor: an alias for `Reg<R2Y_SET1_SPEC>`"]
pub type R2Y_SET1 = crate::Reg<r2y_set1::R2Y_SET1_SPEC>;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
pub mod r2y_set1;
#[doc = "R2Y_SET2 (rw) register accessor: an alias for `Reg<R2Y_SET2_SPEC>`"]
pub type R2Y_SET2 = crate::Reg<r2y_set2::R2Y_SET2_SPEC>;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
pub mod r2y_set2;
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "ISI Control Register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "ISI Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "ISI Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "ISI Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "ISI Interrupt Mask Register"]
pub mod imr;
#[doc = "DMA_CHER (w) register accessor: an alias for `Reg<DMA_CHER_SPEC>`"]
pub type DMA_CHER = crate::Reg<dma_cher::DMA_CHER_SPEC>;
#[doc = "DMA Channel Enable Register"]
pub mod dma_cher;
#[doc = "DMA_CHDR (w) register accessor: an alias for `Reg<DMA_CHDR_SPEC>`"]
pub type DMA_CHDR = crate::Reg<dma_chdr::DMA_CHDR_SPEC>;
#[doc = "DMA Channel Disable Register"]
pub mod dma_chdr;
#[doc = "DMA_CHSR (r) register accessor: an alias for `Reg<DMA_CHSR_SPEC>`"]
pub type DMA_CHSR = crate::Reg<dma_chsr::DMA_CHSR_SPEC>;
#[doc = "DMA Channel Status Register"]
pub mod dma_chsr;
#[doc = "DMA_P_ADDR (rw) register accessor: an alias for `Reg<DMA_P_ADDR_SPEC>`"]
pub type DMA_P_ADDR = crate::Reg<dma_p_addr::DMA_P_ADDR_SPEC>;
#[doc = "DMA Preview Base Address Register"]
pub mod dma_p_addr;
#[doc = "DMA_P_CTRL (rw) register accessor: an alias for `Reg<DMA_P_CTRL_SPEC>`"]
pub type DMA_P_CTRL = crate::Reg<dma_p_ctrl::DMA_P_CTRL_SPEC>;
#[doc = "DMA Preview Control Register"]
pub mod dma_p_ctrl;
#[doc = "DMA_P_DSCR (rw) register accessor: an alias for `Reg<DMA_P_DSCR_SPEC>`"]
pub type DMA_P_DSCR = crate::Reg<dma_p_dscr::DMA_P_DSCR_SPEC>;
#[doc = "DMA Preview Descriptor Address Register"]
pub mod dma_p_dscr;
#[doc = "DMA_C_ADDR (rw) register accessor: an alias for `Reg<DMA_C_ADDR_SPEC>`"]
pub type DMA_C_ADDR = crate::Reg<dma_c_addr::DMA_C_ADDR_SPEC>;
#[doc = "DMA Codec Base Address Register"]
pub mod dma_c_addr;
#[doc = "DMA_C_CTRL (rw) register accessor: an alias for `Reg<DMA_C_CTRL_SPEC>`"]
pub type DMA_C_CTRL = crate::Reg<dma_c_ctrl::DMA_C_CTRL_SPEC>;
#[doc = "DMA Codec Control Register"]
pub mod dma_c_ctrl;
#[doc = "DMA_C_DSCR (rw) register accessor: an alias for `Reg<DMA_C_DSCR_SPEC>`"]
pub type DMA_C_DSCR = crate::Reg<dma_c_dscr::DMA_C_DSCR_SPEC>;
#[doc = "DMA Codec Descriptor Address Register"]
pub mod dma_c_dscr;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
