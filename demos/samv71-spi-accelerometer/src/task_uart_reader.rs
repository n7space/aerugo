use crate::{
    AccelerometerScale, Command, CommandEvent, CommandType, OutputDataRate, TransferArrayType,
};

use aerugo::{logln, MessageQueueHandle, RuntimeApi};

pub struct TaskUartReaderContext {
    pub data_output_rate_queue: MessageQueueHandle<OutputDataRate, 2>,
    pub accelerometer_scale_queue: MessageQueueHandle<AccelerometerScale, 2>,
}

pub fn task_uart_reader(
    val: TransferArrayType,
    context: &mut TaskUartReaderContext,
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
            CommandType::SetDataOutputRate => {
                context
                    .data_output_rate_queue
                    .send_data(command.argument().into())
                    .expect("Failed to send data output rate");
            }
            CommandType::SetGyroscopeScale => logln!("Got SetGyroscopeScale"),
            CommandType::SetAccelerometerScale => {
                context
                    .accelerometer_scale_queue
                    .send_data(command.argument().into())
                    .expect("Failed to send accelerometer scale");
            }
        },
        None => logln!("Received malformed command"),
    };
}
