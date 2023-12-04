use bitfield_enum::{bitfield_enum, BitFieldFromByte};

bitfield_enum!(Watermark [mask=0x80, offset=7] {
    NotReached = 0,
    Reached = 1,
});

bitfield_enum!(Overrun [mask=0x40, offset=6] {
    FifoNotFull = 0,
    FifoFull = 1,
});

bitfield_enum!(SmartStatus [mask=0x20, offset=5] {
    FifoNotFull = 0,
    FifoWillBeFullAtNextODR = 1,
});

bitfield_enum!(CounterThreshold [mask=0x10, offset=4] {
    NotReached = 0,
    Reached = 1,
});

bitfield_enum!(LatchedOverrun [mask=0x08, offset=3] {
    FifoNotFull = 0,
    FifoFull = 1,
});

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FifoStatus {
    pub stored_words: u16,
    pub latched_overrun: LatchedOverrun,
    pub counter_threshold: CounterThreshold,
    pub smart_fifo: SmartStatus,
    pub overrun: Overrun,
    pub watermark: Watermark,
}

pub(crate) type FifoStatusBuffer = [u8; 2];

impl From<FifoStatusBuffer> for FifoStatus {
    fn from(buffer: FifoStatusBuffer) -> Self {
        const STORED_WORDS_MASK: u8 = 0x03;
        Self {
            stored_words: u16::from_le_bytes([buffer[0], buffer[1] & STORED_WORDS_MASK]),
            latched_overrun: LatchedOverrun::from_byte(buffer[1]),
            counter_threshold: CounterThreshold::from_byte(buffer[1]),
            smart_fifo: SmartStatus::from_byte(buffer[1]),
            overrun: Overrun::from_byte(buffer[1]),
            watermark: Watermark::from_byte(buffer[1]),
        }
    }
}
