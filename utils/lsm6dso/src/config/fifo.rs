//! Module with config-related items

use crate::{
    bounded_int::BoundedU16,
    registers::{MultiRegisterConversion, MultiRegisterField, RegisterConversion, RegisterField},
};

/// Type representing FIFO watermark threshold as FIFO records (6 bytes of sensor data + tag)
pub type FifoWatermarkThreshold = BoundedU16<0, 0x1FF>;
impl MultiRegisterField for FifoWatermarkThreshold {
    const MASKS: [u8; 2] = [0xFF, 0x01];
    const OFFSETS: [usize; 2] = [0, 0];
}

impl MultiRegisterConversion for FifoWatermarkThreshold {
    fn to_regs(self) -> [u8; 2] {
        self.to_le_bytes()
    }

    fn from_regs(regs: &[u8]) -> Self {
        assert!(regs.len() >= 2);
        let value_lsb = regs[0];
        let value_msb = regs[1] & Self::MASKS[1];
        Self::new(u16::from_le_bytes([value_lsb, value_msb])).unwrap()
    }
}

/// Type representing amount of items in FIFO as FIFO records (6 bytes of sensor data + tag)
pub type FifoDataLength = BoundedU16<0, 0x3FF>;
impl MultiRegisterField for FifoDataLength {
    const MASKS: [u8; 2] = [0xFF, 0x03];
    const OFFSETS: [usize; 2] = [0, 0];
}

impl MultiRegisterConversion for FifoDataLength {
    fn to_regs(self) -> [u8; 2] {
        self.to_le_bytes()
    }

    fn from_regs(regs: &[u8]) -> Self {
        assert!(regs.len() >= 2);
        let value_lsb = regs[0];
        let value_msb = regs[1] & Self::MASKS[1];
        Self::new(u16::from_le_bytes([value_lsb, value_msb])).unwrap()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DataRateChangeBatching {
    Enabled = 1,
    Disabled = 0,
}
impl RegisterField for DataRateChangeBatching {
    const MASK: u8 = 0x10;
    const OFFSET: usize = 4;
}
impl RegisterConversion for DataRateChangeBatching {
    fn to_reg(self) -> u8 {
        (self as u8) << Self::OFFSET
    }

    fn from_reg(reg: u8) -> Self {
        if reg & Self::MASK != 0 {
            DataRateChangeBatching::Enabled
        } else {
            DataRateChangeBatching::Disabled
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StopOnWatermarkThreshold {
    Yes = 1,
    No = 0,
}
impl RegisterField for StopOnWatermarkThreshold {
    const MASK: u8 = 0x80;
    const OFFSET: usize = 7;
}
impl RegisterConversion for StopOnWatermarkThreshold {
    fn to_reg(self) -> u8 {
        (self as u8) << Self::OFFSET
    }

    fn from_reg(reg: u8) -> Self {
        if reg & Self::MASK != 0 {
            StopOnWatermarkThreshold::Yes
        } else {
            StopOnWatermarkThreshold::No
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GyroscopeBatchingRate {
    NoBatching = 0b0000,
    Batch12_5Hz = 0b0001,
    Batch26Hz = 0b0010,
    Batch52Hz = 0b0011,
    Batch104Hz = 0b0100,
    Batch208Hz = 0b0101,
    Batch417Hz = 0b0110,
    Batch833Hz = 0b0111,
    Batch1667Hz = 0b1000,
    Batch3333Hz = 0b1001,
    Batch6667Hz = 0b1010,
    Batch6_5Hz = 0b1011,
}
impl RegisterField for GyroscopeBatchingRate {
    const MASK: u8 = 0xF0;
    const OFFSET: usize = 4;
}
impl RegisterConversion for GyroscopeBatchingRate {
    fn to_reg(self) -> u8 {
        (self as u8) << Self::OFFSET
    }

    fn from_reg(reg: u8) -> Self {
        match (reg & Self::MASK) >> Self::OFFSET {
            0b0000 => GyroscopeBatchingRate::NoBatching,
            0b0001 => GyroscopeBatchingRate::Batch12_5Hz,
            0b0010 => GyroscopeBatchingRate::Batch26Hz,
            0b0011 => GyroscopeBatchingRate::Batch52Hz,
            0b0100 => GyroscopeBatchingRate::Batch104Hz,
            0b0101 => GyroscopeBatchingRate::Batch208Hz,
            0b0110 => GyroscopeBatchingRate::Batch417Hz,
            0b0111 => GyroscopeBatchingRate::Batch833Hz,
            0b1000 => GyroscopeBatchingRate::Batch1667Hz,
            0b1001 => GyroscopeBatchingRate::Batch3333Hz,
            0b1010 => GyroscopeBatchingRate::Batch6667Hz,
            0b1011 => GyroscopeBatchingRate::Batch6_5Hz,
            other => {
                panic!("Unexpected value tried to be parsed as GyroscopeBatchingRate: {other:2X}");
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AccelerometerBatchingRate {
    NoBatching = 0b0000,
    Batch12_5Hz = 0b0001,
    Batch26Hz = 0b0010,
    Batch52Hz = 0b0011,
    Batch104Hz = 0b0100,
    Batch208Hz = 0b0101,
    Batch417Hz = 0b0110,
    Batch833Hz = 0b0111,
    Batch1667Hz = 0b1000,
    Batch3333Hz = 0b1001,
    Batch6667Hz = 0b1010,
    Batch1_6Hz = 0b1011,
}
impl RegisterField for AccelerometerBatchingRate {
    const MASK: u8 = 0x0F;
    const OFFSET: usize = 0;
}
impl RegisterConversion for AccelerometerBatchingRate {
    fn to_reg(self) -> u8 {
        (self as u8) << Self::OFFSET
    }

    fn from_reg(reg: u8) -> Self {
        match (reg & Self::MASK) >> Self::OFFSET {
            0b0000 => AccelerometerBatchingRate::NoBatching,
            0b0001 => AccelerometerBatchingRate::Batch12_5Hz,
            0b0010 => AccelerometerBatchingRate::Batch26Hz,
            0b0011 => AccelerometerBatchingRate::Batch52Hz,
            0b0100 => AccelerometerBatchingRate::Batch104Hz,
            0b0101 => AccelerometerBatchingRate::Batch208Hz,
            0b0110 => AccelerometerBatchingRate::Batch417Hz,
            0b0111 => AccelerometerBatchingRate::Batch833Hz,
            0b1000 => AccelerometerBatchingRate::Batch1667Hz,
            0b1001 => AccelerometerBatchingRate::Batch3333Hz,
            0b1010 => AccelerometerBatchingRate::Batch6667Hz,
            0b1011 => AccelerometerBatchingRate::Batch1_6Hz,
            other => {
                panic!(
                    "Unexpected value tried to be parsed as AccelerometerBatchingRate: {other:2X}"
                );
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FifoMode {
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
}
impl RegisterField for FifoMode {
    const MASK: u8 = 0x07;
    const OFFSET: usize = 0;
}
impl RegisterConversion for FifoMode {
    fn to_reg(self) -> u8 {
        (self as u8) << Self::OFFSET
    }

    fn from_reg(reg: u8) -> Self {
        match (reg & Self::MASK) >> Self::OFFSET {
            0b000 => FifoMode::Bypass,
            0b001 => FifoMode::Fifo,
            0b011 => FifoMode::ContinuousToFifo,
            0b101 => FifoMode::BypassToContinuous,
            0b110 => FifoMode::Continuous,
            0b111 => FifoMode::BypassToFifo,
            other => {
                panic!("Unexpected value tried to be parsed as FifoMode: {other:2X}");
            }
        }
    }
}

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
        let watermark_threshold_bytes = config.watermark_threshold.to_regs();
        [
            watermark_threshold_bytes[0],
            watermark_threshold_bytes[1]
                | config.odr_change_batched.to_reg()
                | config.stop_on_watermark.to_reg(),
            config.accelerometer_batching_rate.to_reg() | config.gyroscope_batching_rate.to_reg(),
            config.mode.to_reg(),
        ]
    }
}

impl From<FifoConfigBuffer> for FifoConfig {
    fn from(regs: FifoConfigBuffer) -> Self {
        FifoConfig {
            watermark_threshold: FifoWatermarkThreshold::from_regs(&regs[0..=1]),
            odr_change_batched: DataRateChangeBatching::from_reg(regs[1]),
            stop_on_watermark: StopOnWatermarkThreshold::from_reg(regs[1]),
            gyroscope_batching_rate: GyroscopeBatchingRate::from_reg(regs[2]),
            accelerometer_batching_rate: AccelerometerBatchingRate::from_reg(regs[2]),
            mode: FifoMode::from_reg(regs[3]),
        }
    }
}
