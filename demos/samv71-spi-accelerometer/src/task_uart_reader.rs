use crate::{Command, CommandEvent, CommandType, TransferArrayType};

use aerugo::{logln, RuntimeApi};

#[derive(Default)]
pub struct TaskUartReaderContext {}

pub fn task_uart_reader(
    val: TransferArrayType,
    _: &mut TaskUartReaderContext,
    api: &'static dyn RuntimeApi,
) {
    match Command::from_array(val) {
        Some(command) => match command.command_type() {
            CommandType::Start => {
                api.emit_event(CommandEvent::Start.into())
                    .expect("Failed to emit CommandEvent::Start");
            }
            CommandType::Stop => {
                api.emit_event(CommandEvent::Stop.into())
                    .expect("Failed to emit CommandEvent::Stop");
            }
            CommandType::SetAccelerometerScale => logln!("Got SetAccelerometerScale"),
            CommandType::SetGyroscopeScale => logln!("Got SetGyroscopeScale"),
        },
        None => logln!("Received malformed command"),
    };
}
