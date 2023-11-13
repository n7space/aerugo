use crate::TransferArrayType;

#[derive(Copy, Clone)]
pub enum CommandType {
    Start,
    Stop,
    SetAccelerometerScale,
    SetGyroscopeScale,
}

impl CommandType {
    pub const fn from_byte(val: u8) -> Option<Self> {
        match val {
            0x10 => Some(CommandType::Start),
            0x20 => Some(CommandType::Stop),
            0x30 => Some(CommandType::SetAccelerometerScale),
            0x40 => Some(CommandType::SetGyroscopeScale),
            _ => None
        }
    }
}

pub struct Command {
    command_type: CommandType,
    argument: u8,
}

impl Command {
    pub fn from_array(arr: TransferArrayType) -> Option<Self> {
        let data_length = ((arr[4] as u16) << 8) | (arr[5] as u16);

        if data_length != 1 {
            return None;
        }

        let command_type = CommandType::from_byte(arr[3]);

        if command_type.is_none() {
            return None;
        }

        Some(Self {
            command_type: command_type.unwrap(),
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
