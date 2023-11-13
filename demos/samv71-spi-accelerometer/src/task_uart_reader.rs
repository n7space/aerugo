use crate::{Command, CommandType, TransferArrayType};

use aerugo::{logln, RuntimeApi};

pub struct TaskUartReaderContext {}

pub fn task_uart_reader(
    val: TransferArrayType,
    _: &mut TaskUartReaderContext,
    _: &'static dyn RuntimeApi,
) {
    match Command::from_array(val) {
        Some(command) => match command.command_type() {
            CommandType::Start => logln!("Got Start"),
            CommandType::Stop => logln!("Got Stop"),
            CommandType::SetAccelerometerScale => logln!("Got SetAccelerometerScale"),
            CommandType::SetGyroscopeScale => logln!("Got SetGyroscopeScale"),
        },
        None => logln!("Received malformed command"),
    };
}
