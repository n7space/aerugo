use aerugo::EventId;

pub enum CommandEvent {
    Start,
    Stop,
    GetExecutionStats,
}

impl From<CommandEvent> for EventId {
    fn from(value: CommandEvent) -> Self {
        match value {
            CommandEvent::Start => 0x10,
            CommandEvent::Stop => 0x20,
            CommandEvent::GetExecutionStats => 0x60,
        }
    }
}
