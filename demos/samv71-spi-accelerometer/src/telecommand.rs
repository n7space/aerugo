use derive_more::TryFrom;

use crate::ccsds::CCSDSPrimaryHeader;

#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFrom)]
#[repr(u8)]
#[try_from(repr)]
pub enum TelecommandType {
    Start = 0x10,
    Stop = 0x20,
    SetDataOutputRate = 0x30,
    SetAccelerometerScale = 0x40,
    SetGyroscopeScale = 0x50,
    GetExecutionStats = 0x60,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Telecommand {
    telecommand_type: TelecommandType,
    argument: u8,
}

impl TryFrom<CCSDSPrimaryHeader> for TelecommandType {
    type Error = u8;

    fn try_from(header: CCSDSPrimaryHeader) -> Result<Self, Self::Error> {
        let opcode = header.sequence_control.name.get() as u8;
        TelecommandType::try_from(opcode).map_err(|err| err.input)
    }
}

impl Telecommand {
    pub fn from_ccsds_packet(
        header: CCSDSPrimaryHeader,
        data: &[u8],
    ) -> Result<Self, &'static str> {
        if header.data_length != 1 {
            return Err("unexpected data length, expected 1 byte");
        }

        Ok(Self {
            telecommand_type: header.try_into().map_err(|_| "invalid command type")?,
            argument: data[0],
        })
    }

    pub fn command_type(&self) -> TelecommandType {
        self.telecommand_type
    }

    pub fn argument(&self) -> u8 {
        self.argument
    }
}

pub trait FromTelecommandArgument
where
    Self: Sized,
{
    type Error;
    fn from_telecommand_argument(argument: u8) -> Result<Self, Self::Error>;
}

pub mod arguments {
    use super::FromTelecommandArgument;
    use crate::task_set_accelerometer_scale::AccelerometerScale;
    use crate::task_set_data_output_rate::OutputDataRate;
    use crate::task_set_gyroscope_scale::GyroscopeScale;

    impl FromTelecommandArgument for OutputDataRate {
        type Error = u8;

        fn from_telecommand_argument(argument: u8) -> Result<Self, Self::Error> {
            match argument {
                0x01 => Ok(OutputDataRate::Odr12_5Hz),
                0x02 => Ok(OutputDataRate::Odr26Hz),
                0x03 => Ok(OutputDataRate::Odr52Hz),
                0x04 => Ok(OutputDataRate::Odr104Hz),
                0x05 => Ok(OutputDataRate::Odr208Hz),
                0x06 => Ok(OutputDataRate::Odr416Hz),
                0x07 => Ok(OutputDataRate::Odr833Hz),
                0x08 => Ok(OutputDataRate::Odr1667Hz),
                0x09 => Ok(OutputDataRate::Odr3333Hz),
                0x0A => Ok(OutputDataRate::Odr6667Hz),
                other => Err(other),
            }
        }
    }

    impl FromTelecommandArgument for AccelerometerScale {
        type Error = u8;

        fn from_telecommand_argument(argument: u8) -> Result<Self, Self::Error> {
            match argument {
                0x01 => Ok(AccelerometerScale::Scale2g),
                0x02 => Ok(AccelerometerScale::Scale4g),
                0x03 => Ok(AccelerometerScale::Scale8g),
                0x04 => Ok(AccelerometerScale::Scale16g),
                other => Err(other),
            }
        }
    }

    impl FromTelecommandArgument for GyroscopeScale {
        type Error = u8;

        fn from_telecommand_argument(argument: u8) -> Result<Self, Self::Error> {
            match argument {
                0x01 => Ok(GyroscopeScale::Scale125dps),
                0x02 => Ok(GyroscopeScale::Scale250dps),
                0x03 => Ok(GyroscopeScale::Scale500dps),
                0x04 => Ok(GyroscopeScale::Scale1000dps),
                0x05 => Ok(GyroscopeScale::Scale2000dps),
                other => Err(other),
            }
        }
    }
}
