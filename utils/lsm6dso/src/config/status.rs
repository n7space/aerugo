#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Status {
    pub new_temperature_available: bool,
    pub new_gyroscope_data_available: bool,
    pub new_accelerometer_data_available: bool,
}

impl From<Status> for u8 {
    fn from(value: Status) -> Self {
        ((value.new_temperature_available as u8) << 2)
            | ((value.new_gyroscope_data_available as u8) << 1)
            | (value.new_accelerometer_data_available as u8)
    }
}
