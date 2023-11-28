use crate::ccsds::CCSDSPrimaryHeader;

#[derive(Copy, Clone)]
pub enum CommandType {
    Start,
    Stop,
    SetDataOutputRate,
    SetAccelerometerScale,
    SetGyroscopeScale,
    GetExecutionStats,
}

impl CommandType {
    pub const fn from_byte(val: u8) -> Option<Self> {
        match val {
            0x10 => Some(CommandType::Start),
            0x20 => Some(CommandType::Stop),
            0x30 => Some(CommandType::SetDataOutputRate),
            0x40 => Some(CommandType::SetAccelerometerScale),
            0x50 => Some(CommandType::SetGyroscopeScale),
            0x60 => Some(CommandType::GetExecutionStats),
            _ => None,
        }
    }
}

pub struct Command {
    command_type: CommandType,
    argument: u8,
}

impl TryFrom<CCSDSPrimaryHeader> for CommandType {
    type Error = u8;

    fn try_from(header: CCSDSPrimaryHeader) -> Result<Self, Self::Error> {
        let opcode = header.sequence_control.name.get() as u8;
        CommandType::from_byte(opcode).ok_or(opcode)
    }
}

impl Command {
    pub fn from_telecommand(header: CCSDSPrimaryHeader, data: &[u8]) -> Result<Self, &'static str> {
        if header.data_length != 1 {
            return Err("unexpected data length, expected 1 byte");
        }

        let command_type: CommandType = header.try_into().map_err(|_| "invalid command type")?;

        Ok(Self {
            command_type,
            argument: data[0],
        })
    }

    pub fn command_type(&self) -> CommandType {
        self.command_type
    }

    pub fn argument(&self) -> u8 {
        self.argument
    }
}

pub trait FromCommandArgument
where
    Self: Sized,
{
    type Error;
    fn from_command_argument(argument: u8) -> Result<Self, Self::Error>;
}

pub mod arguments {
    use super::FromCommandArgument;
    use crate::task_set_accelerometer_scale::AccelerometerScale;
    use crate::task_set_data_output_rate::OutputDataRate;
    use crate::task_set_gyroscope_scale::GyroscopeScale;

    impl FromCommandArgument for OutputDataRate {
        type Error = u8;

        fn from_command_argument(argument: u8) -> Result<Self, Self::Error> {
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

    impl FromCommandArgument for AccelerometerScale {
        type Error = u8;

        fn from_command_argument(argument: u8) -> Result<Self, Self::Error> {
            match argument {
                0x01 => Ok(AccelerometerScale::Scale2g),
                0x02 => Ok(AccelerometerScale::Scale4g),
                0x03 => Ok(AccelerometerScale::Scale8g),
                0x04 => Ok(AccelerometerScale::Scale16g),
                other => Err(other),
            }
        }
    }

    impl FromCommandArgument for GyroscopeScale {
        type Error = u8;

        fn from_command_argument(argument: u8) -> Result<Self, Self::Error> {
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
