//! Interrupt-related XDMAC items.

use samv71q21_pac::xdmac::xdmac_chid::{cim::R as CIMReader, cis::R as CISReader};

/// Represents the state of XDMAC channel's interrupts status.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ChannelEvents {
    /// End of block interrupt.
    pub end_of_block: bool,
    /// End of transfer list interrupt.
    pub end_of_list: bool,
    /// End of disable interrupt.
    pub end_of_disable: bool,
    /// End of flush interrupt.
    pub end_of_flush: bool,
    /// Read bus error interrupt.
    pub read_bus_error: bool,
    /// Write bus error interrupt.
    pub write_bus_error: bool,
    /// Request overflow error interrupt.
    pub request_overflow_error: bool,
}

impl From<CISReader> for ChannelEvents {
    fn from(reg: CISReader) -> Self {
        ChannelEvents {
            end_of_block: reg.bis().bit_is_set(),
            end_of_list: reg.lis().bit_is_set(),
            end_of_disable: reg.dis().bit_is_set(),
            end_of_flush: reg.fis().bit_is_set(),
            read_bus_error: reg.rbeis().bit_is_set(),
            write_bus_error: reg.wbeis().bit_is_set(),
            request_overflow_error: reg.rois().bit_is_set(),
        }
    }
}

impl From<CIMReader> for ChannelEvents {
    fn from(reg: CIMReader) -> Self {
        ChannelEvents {
            end_of_block: reg.bim().bit_is_set(),
            end_of_list: reg.lim().bit_is_set(),
            end_of_disable: reg.dim().bit_is_set(),
            end_of_flush: reg.fim().bit_is_set(),
            read_bus_error: reg.rbeim().bit_is_set(),
            write_bus_error: reg.wbeim().bit_is_set(),
            request_overflow_error: reg.roim().bit_is_set(),
        }
    }
}
