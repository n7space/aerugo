use crate::{ccsds::CCSDSPrimaryHeader, TelecommandBuffer};

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
    pub fn from_telecommand(buffer: TelecommandBuffer) -> Option<Self> {
        let header = match CCSDSPrimaryHeader::try_from(&buffer[0..=5].try_into().unwrap()) {
            Ok(header) => header,
            Err(_) => return None,
        };

        if header.data_length != 1 {
            return None;
        }

        let command_type: Result<CommandType, u8> = header.try_into();

        if command_type.is_err() {
            return None;
        }

        Some(Self {
            command_type: command_type.unwrap(),
            argument: buffer[6],
        })
    }

    pub fn command_type(&self) -> CommandType {
        self.command_type
    }

    pub fn argument(&self) -> u8 {
        self.argument
    }
}
