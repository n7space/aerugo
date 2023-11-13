use crate::TransferArrayType;

#[derive(Copy, Clone)]
pub enum CommandType {
    Start,
    Stop,
    SetAccelerometerScale,
    SetGyroscopeScale,
}

impl CommandType {
    pub const fn from_byte(val: u8) -> Self {
        match val {
            0x10 => CommandType::Start,
            0x20 => CommandType::Stop,
            0x30 => CommandType::SetAccelerometerScale,
            0x40 => CommandType::SetGyroscopeScale,
            _ => unreachable!(),
        }
    }
}

pub struct Command {
    command_type: CommandType,
    argument: u8,
}

impl Command {
    pub const fn from_array(arr: TransferArrayType) -> Option<Self> {
        let data_length = ((arr[4] as u16) << 8) | (arr[5] as u16);

        if data_length != 1 {
            return None;
        }

        Some(Self {
            command_type: CommandType::from_byte(arr[3]),
            argument: arr[6],
        })
    }

    pub fn command_type(&self) -> CommandType {
        self.command_type
    }

    pub fn argument(&self) -> u8 {
        self.argument
    }
}
