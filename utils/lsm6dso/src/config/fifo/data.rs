use crate::{
    config::{data_types::FromBuffer, templates::register_enum},
    registers::FromRegister,
};

register_enum!(Tag [mask=0xF8, offset=3] {
    GyroscopeNC = 0x01,
    AccelerometerNC = 0x02,
    Temperature = 0x03,
    Timestamp = 0x04,
    ConfigChange = 0x05,
    AccelerometerNCT2 = 0x06,
    AccelerometerNCT1 = 0x07,
    Accelerometer2xC = 0x08,
    Accelerometer3xC = 0x09,
    GyroscopeNCT2 = 0x0A,
    GyroscopeNCT1 = 0x0B,
    Gyroscope2xC = 0x0C,
    Gyroscope3xC = 0x0D,
    SensorHubSlave0 = 0x0E,
    SensorHubSlave1 = 0x0F,
    SensorHubSlave2 = 0x10,
    SensorHubSlave3 = 0x11,
    StepCounter = 0x12,
    SensorHubNack = 0x19,
});

pub struct FifoWord {
    pub tag: Tag,
    pub data: [u8; 6],
}

impl FromBuffer<7> for FifoWord {
    fn from_buffer(buffer: &[u8; 7]) -> Self {
        FifoWord {
            tag: Tag::from_reg(buffer[0]),
            data: buffer[1..7].try_into().unwrap(),
        }
    }
}
