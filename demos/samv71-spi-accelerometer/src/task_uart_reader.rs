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
            CommandType::Stop => logln!("Got Stop"),
            CommandType::SetAccelerometerScale => logln!("Got SetAccelerometerScale"),
            CommandType::SetGyroscopeScale => logln!("Got SetGyroscopeScale"),
        },
        None => logln!("Received malformed command"),
    };
}
