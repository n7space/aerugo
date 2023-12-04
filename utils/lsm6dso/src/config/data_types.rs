use fugit::TimerInstantU32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vec3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Timestamps are provided with 0.25us resolution, therefore it's timer runs @ 4MHz.
const TIMESTAMP_TIMER_FREQUENCY_HZ: u32 = 4_000_000;

pub type Temperature = i16;
pub type LinearAcceleration = Vec3D<i16>;
pub type AngularRate = Vec3D<i16>;
pub type Timestamp = TimerInstantU32<{ TIMESTAMP_TIMER_FREQUENCY_HZ }>;

pub(crate) trait FromBuffer<const DATA_LENGTH: usize> {
    fn from_buffer(buffer: &[u8; DATA_LENGTH]) -> Self;
}

impl FromBuffer<2> for Temperature {
    fn from_buffer(buffer: &[u8; 2]) -> Self {
        Self::from_le_bytes(*buffer)
    }
}

// GyroscopeData implementation is not necessary, as it's the same as AccelerometerData.
// If underlying type ever changes, remember to add the conversion.
impl FromBuffer<6> for LinearAcceleration {
    fn from_buffer(buffer: &[u8; 6]) -> Self {
        Self {
            x: i16::from_le_bytes([buffer[0], buffer[1]]),
            y: i16::from_le_bytes([buffer[2], buffer[3]]),
            z: i16::from_le_bytes([buffer[4], buffer[5]]),
        }
    }
}
