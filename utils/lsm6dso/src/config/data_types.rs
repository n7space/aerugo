pub struct Vec3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Temperature = i16;
pub type LinearAcceleration = Vec3D<i16>;
pub type AngularRate = Vec3D<i16>;
pub type Timestamp = u32;

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

impl FromBuffer<4> for Timestamp {
    fn from_buffer(buffer: &[u8; 4]) -> Self {
        u32::from_le_bytes(*buffer)
    }
}
