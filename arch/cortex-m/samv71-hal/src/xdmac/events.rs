//! Interrupt-related XDMAC items.

use samv71q21_pac::xdmac::xdmac_chid::cis::R as ChannelStatusRegisterReader;

/// Enumeration representing IRQ status.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EventState {
    /// IRQ is disabled.
    Disabled,
    /// IRQ is enabled.
    Enabled,
}

/// Represents the state of XDMAC channel's interrupts status.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ChannelEvents {
    /// End of block interrupt.
    pub end_of_block: EventState,
    /// End of transfer list interrupt.
    pub end_of_list: EventState,
    /// End of disable interrupt.
    pub end_of_disable: EventState,
    /// End of flush interrupt.
    pub end_of_flush: EventState,
    /// Read bus error interrupt.
    pub read_bus_error: EventState,
    /// Write bus error interrupt.
    pub write_bus_error: EventState,
    /// Request overflow error interrupt.
    pub request_overflow_error: EventState,
}

impl EventState {
    /// Shorthand for converting the event state into `bool`.
    /// Reverse operation can be done with `.into()`.
    ///
    /// Converting into a `bool` using `.into()` is also allowed, but sometimes the syntax is very
    /// nasty.
    pub fn into_bool(self) -> bool {
        self.into()
    }
}

impl From<bool> for EventState {
    fn from(value: bool) -> Self {
        match value {
            true => EventState::Enabled,
            false => EventState::Disabled,
        }
    }
}

impl From<EventState> for bool {
    fn from(value: EventState) -> Self {
        match value {
            EventState::Disabled => false,
            EventState::Enabled => true,
        }
    }
}

impl From<ChannelStatusRegisterReader> for ChannelEvents {
    fn from(reg: ChannelStatusRegisterReader) -> Self {
        ChannelEvents {
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
