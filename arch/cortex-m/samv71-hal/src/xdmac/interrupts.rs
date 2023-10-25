//! Interrupt-related XDMAC items.

use samv71q21_pac::xdmac::xdmac_chid::cis::R as ChannelStatusRegisterReader;

/// Enumeration representing IRQ status.
pub enum InterruptStatus {
    /// IRQ is disabled.
    Disabled,
    /// IRQ is enabled.
    Enabled,
}

/// Represents the state of XDMAC channel's interrupts status.
pub struct ChannelInterrupts {
    /// End of block interrupt.
    pub end_of_block: InterruptStatus,
    /// End of transfer list interrupt.
    pub end_of_list: InterruptStatus,
    /// End of disable interrupt.
    pub end_of_disable: InterruptStatus,
    /// End of flush interrupt.
    pub end_of_flush: InterruptStatus,
    /// Read bus error interrupt.
    pub read_bus_error: InterruptStatus,
    /// Write bus error interrupt.
    pub write_bus_error: InterruptStatus,
    /// Request overflow error interrupt.
    pub request_overflow_error: InterruptStatus,
}

impl From<bool> for InterruptStatus {
    fn from(value: bool) -> Self {
        match value {
            true => InterruptStatus::Enabled,
            false => InterruptStatus::Disabled,
        }
    }
}

impl From<ChannelStatusRegisterReader> for ChannelInterrupts {
    fn from(reg: ChannelStatusRegisterReader) -> Self {
        ChannelInterrupts {
            end_of_block: reg.bis().bit_is_set().into(),
            end_of_list: reg.lis().bit_is_set().into(),
            end_of_disable: reg.dis().bit_is_set().into(),
            end_of_flush: reg.fis().bit_is_set().into(),
            read_bus_error: reg.rbeis().bit_is_set().into(),
            write_bus_error: reg.wbeis().bit_is_set().into(),
            request_overflow_error: reg.rois().bit_is_set().into(),
        }
    }
}
