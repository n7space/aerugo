use crate::{
    bitfield_enum::{bitfield_enum, FromRegister},
    config::data_types::{AngularRate, FromBuffer, LinearAcceleration, Temperature, Timestamp},
};

bitfield_enum!(Tag [mask=0xF8, offset=3] {
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

pub type RawData = [u8; 6];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FifoWordStruct {
    pub tag: Tag,
    pub data: RawData,
}

impl FromBuffer<7> for FifoWordStruct {
    fn from_buffer(buffer: &[u8; 7]) -> Self {
        FifoWordStruct {
            tag: Tag::from_reg(buffer[0]),
            data: buffer[1..7].try_into().unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FifoWord {
    Gyroscope(AngularRate),
    Accelerometer(LinearAcceleration),
    Temperature(Temperature),
    Timestamp(Timestamp),
    ConfigChange(RawData),
    AccelerometerNCT2(LinearAcceleration),
    AccelerometerNCT1(LinearAcceleration),
    Accelerometer2xC(LinearAcceleration),
    Accelerometer3xC(LinearAcceleration),
    GyroscopeNCT2(AngularRate),
    GyroscopeNCT1(AngularRate),
    Gyroscope2xC(AngularRate),
    Gyroscope3xC(AngularRate),
    SensorHubSlave0(RawData),
    SensorHubSlave1(RawData),
    SensorHubSlave2(RawData),
    SensorHubSlave3(RawData),
    StepCounter(RawData),
    SensorHubNack(RawData),
}

impl From<FifoWordStruct> for FifoWord {
    fn from(value: FifoWordStruct) -> Self {
        match value.tag {
            Tag::GyroscopeNC => FifoWord::Gyroscope(AngularRate::from_buffer(&value.data)),
            Tag::AccelerometerNC => {
                FifoWord::Accelerometer(LinearAcceleration::from_buffer(&value.data))
            }
            Tag::Temperature => FifoWord::Temperature(Temperature::from_buffer(
                &value.data[4..6].try_into().unwrap(),
            )),
            Tag::Timestamp => {
                let ticks = u32::from_be_bytes(value.data[2..6].try_into().unwrap());
                FifoWord::Timestamp(Timestamp::from_ticks(ticks))
            }
            Tag::ConfigChange => FifoWord::ConfigChange(value.data),
            Tag::AccelerometerNCT2 => {
                FifoWord::AccelerometerNCT2(LinearAcceleration::from_buffer(&value.data))
            }
            Tag::AccelerometerNCT1 => {
                FifoWord::AccelerometerNCT1(LinearAcceleration::from_buffer(&value.data))
            }
            Tag::Accelerometer2xC => {
                FifoWord::Accelerometer2xC(LinearAcceleration::from_buffer(&value.data))
            }
            Tag::Accelerometer3xC => {
                FifoWord::Accelerometer3xC(LinearAcceleration::from_buffer(&value.data))
            }
            Tag::GyroscopeNCT2 => FifoWord::GyroscopeNCT2(AngularRate::from_buffer(&value.data)),
            Tag::GyroscopeNCT1 => FifoWord::GyroscopeNCT1(AngularRate::from_buffer(&value.data)),
            Tag::Gyroscope2xC => FifoWord::Gyroscope2xC(AngularRate::from_buffer(&value.data)),
            Tag::Gyroscope3xC => FifoWord::Gyroscope3xC(AngularRate::from_buffer(&value.data)),
            Tag::SensorHubSlave0 => FifoWord::SensorHubSlave0(value.data),
            Tag::SensorHubSlave1 => FifoWord::SensorHubSlave1(value.data),
            Tag::SensorHubSlave2 => FifoWord::SensorHubSlave2(value.data),
            Tag::SensorHubSlave3 => FifoWord::SensorHubSlave3(value.data),
            Tag::StepCounter => FifoWord::StepCounter(value.data),
            Tag::SensorHubNack => FifoWord::SensorHubNack(value.data),
        }
    }
}

impl FromBuffer<7> for FifoWord {
    fn from_buffer(buffer: &[u8; 7]) -> Self {
        FifoWordStruct::from_buffer(buffer).into()
    }
}
