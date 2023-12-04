use crate::{
    bitfield_enum::{
        bitfield_enum, BitFieldFromByte, BitFieldToByte, MultiByteBitField,
        MultiByteBitFieldFromBytes, MultiByteBitFieldToBytes,
    },
    bounded_int::BoundedU16,
};

/// Type representing FIFO watermark threshold as FIFO records (6 bytes of sensor data + tag)
pub type FifoWatermarkThreshold = BoundedU16<0, 0x1FF>;
impl MultiByteBitField for FifoWatermarkThreshold {
    const MASKS: [u8; 2] = [0xFF, 0x01];
    const OFFSETS: [usize; 2] = [0, 0];
}

impl MultiByteBitFieldToBytes for FifoWatermarkThreshold {
    fn to_bytes(self) -> [u8; 2] {
        self.to_le_bytes()
    }
}

impl MultiByteBitFieldFromBytes for FifoWatermarkThreshold {
    fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 2);
        let value_lsb = bytes[0];
        let value_msb = bytes[1] & Self::MASKS[1];
        Self::new(u16::from_le_bytes([value_lsb, value_msb])).unwrap()
    }
}

/// Type representing amount of items in FIFO as FIFO records (6 bytes of sensor data + tag)
pub type FifoDataLength = BoundedU16<0, 0x3FF>;
impl MultiByteBitField for FifoDataLength {
    const MASKS: [u8; 2] = [0xFF, 0x03];
    const OFFSETS: [usize; 2] = [0, 0];
}

impl MultiByteBitFieldToBytes for FifoDataLength {
    fn to_bytes(self) -> [u8; 2] {
        self.to_le_bytes()
    }
}

impl MultiByteBitFieldFromBytes for FifoDataLength {
    fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 2);
        let value_lsb = bytes[0];
        let value_msb = bytes[1] & Self::MASKS[1];
        Self::new(u16::from_le_bytes([value_lsb, value_msb])).unwrap()
    }
}

bitfield_enum!(DataRateChangeBatching [mask=0x10, offset=4] {
    Disabled = 0,
    Enabled = 1,
});

bitfield_enum!(StopOnWatermarkThreshold [mask=0x80, offset=7]{
    Yes = 1,
    No = 0,
});

bitfield_enum!(GyroscopeBatchingRate [mask=0xF0, offset=4] {
    NoBatching = 0b0000,
    Batch12_5Hz = 0b0001,
    Batch26Hz = 0b0010,
    Batch52Hz = 0b0011,
    Batch104Hz = 0b0100,
    Batch208Hz = 0b0101,
    Batch416Hz = 0b0110,
    Batch833Hz = 0b0111,
    Batch1667Hz = 0b1000,
    Batch3333Hz = 0b1001,
    Batch6667Hz = 0b1010,
    Batch6_5Hz = 0b1011,
});

bitfield_enum!(AccelerometerBatchingRate [mask=0x0F, offset=0] {
    NoBatching = 0b0000,
    Batch12_5Hz = 0b0001,
    Batch26Hz = 0b0010,
    Batch52Hz = 0b0011,
    Batch104Hz = 0b0100,
    Batch208Hz = 0b0101,
    Batch416Hz = 0b0110,
    Batch833Hz = 0b0111,
    Batch1667Hz = 0b1000,
    Batch3333Hz = 0b1001,
    Batch6667Hz = 0b1010,
    Batch1_6Hz = 0b1011,
});

bitfield_enum!(FifoMode [mask=0x07, offset=0] {
    /// FIFO disabled.
    Bypass = 0b000,
    /// FIFO enabled and sensor stops collecting data when FIFO is full.
    Fifo = 0b001,
    /// Continuous mode until trigger is deasserted, then FIFO mode.
    ContinuousToFifo = 0b011,
    /// Bypass mode until trigger is deasserted, then Continuous mode.
    BypassToContinuous = 0b101,
    /// If FIFO is full, then the new sample overwrites the older one.
    Continuous = 0b110,
    /// Bypass mode until trigger is deasserted, then FIFO mode.
    BypassToFifo = 0b111,
});

/// Structure representing LSM6DSO FIFO configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FifoConfig {
    /// Watermark threshold, defines the amount of FIFO items required for watermark flag to rise.
    pub watermark_threshold: FifoWatermarkThreshold,
    /// State of ODR CHANGE virtual sensor batching in FIFO.
    pub odr_change_batched: DataRateChangeBatching,
    /// Whether to limit FIFO depth up to watermark threshold, or not.
    pub stop_on_watermark: StopOnWatermarkThreshold,
    /// Gyroscope batching data rate.
    pub gyroscope_batching_rate: GyroscopeBatchingRate,
    /// Accelerometer batching data rate
    pub accelerometer_batching_rate: AccelerometerBatchingRate,
    /// FIFO mode
    pub mode: FifoMode,
}

/// Type representing memory buffer for FIFO configuration.
pub type FifoConfigBuffer = [u8; 4];

impl From<FifoConfig> for FifoConfigBuffer {
    fn from(config: FifoConfig) -> Self {
        let watermark_threshold_bytes = config.watermark_threshold.to_bytes();
        [
            watermark_threshold_bytes[0],
            watermark_threshold_bytes[1]
                | config.odr_change_batched.to_byte()
                | config.stop_on_watermark.to_byte(),
            config.accelerometer_batching_rate.to_byte() | config.gyroscope_batching_rate.to_byte(),
            config.mode.to_byte(),
        ]
    }
}

impl From<FifoConfigBuffer> for FifoConfig {
    fn from(regs: FifoConfigBuffer) -> Self {
        FifoConfig {
            watermark_threshold: FifoWatermarkThreshold::from_bytes(&regs[0..=1]),
            odr_change_batched: DataRateChangeBatching::from_byte(regs[1]),
            stop_on_watermark: StopOnWatermarkThreshold::from_byte(regs[1]),
            gyroscope_batching_rate: GyroscopeBatchingRate::from_byte(regs[2]),
            accelerometer_batching_rate: AccelerometerBatchingRate::from_byte(regs[2]),
            mode: FifoMode::from_byte(regs[3]),
        }
    }
}
