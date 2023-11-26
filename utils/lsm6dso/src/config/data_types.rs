pub struct Vec3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Temperature = i16;
pub type AccelerometerData = Vec3D<i16>;
pub type GyroscopeData = Vec3D<i16>;

pub(crate) trait FromBuffer<const DATA_LENGTH: usize> {
    fn from_buffer(buffer: &[u8; DATA_LENGTH]) -> Self;
}

impl FromBuffer<2> for Temperature {
    fn from_buffer(buffer: &[u8; 2]) -> Self {
        Self::from_le_bytes(*buffer)
    }
}

impl FromBuffer<6> for AccelerometerData {
    fn from_buffer(buffer: &[u8; 6]) -> Self {
        Self {
            x: i16::from_le_bytes([buffer[0], buffer[1]]),
            y: i16::from_le_bytes([buffer[2], buffer[3]]),
            z: i16::from_le_bytes([buffer[4], buffer[5]]),
        }
    }
}

// GyroscopeData implementation is not necessary, as it's the same as AccelerometerData.
// If underlying type ever changes, remember to add the conversion.
