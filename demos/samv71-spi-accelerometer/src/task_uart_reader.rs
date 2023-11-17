use crate::{
    AccelerometerScale, Command, CommandEvent, CommandType, GyroscopeScale, OutputDataRate,
    TransferArrayType,
};

use aerugo::{logln, MessageQueueHandle, RuntimeApi};

pub struct TaskUartReaderContext {
    pub data_output_rate_queue: MessageQueueHandle<OutputDataRate, 2>,
    pub accelerometer_scale_queue: MessageQueueHandle<AccelerometerScale, 2>,
    pub gyroscope_scale_queue: MessageQueueHandle<GyroscopeScale, 2>,
}

pub fn task_uart_reader(
    val: TransferArrayType,
    context: &mut TaskUartReaderContext,
    api: &'static dyn RuntimeApi,
) {
    match Command::from_array(val) {
        Some(command) => match command.command_type() {
            CommandType::Start => {
                let result = api.emit_event(CommandEvent::Start.into());

                if result.is_err() {
                    logln!("Failed to emit Start");
                }
            }
            CommandType::Stop => {
                let result = api.emit_event(CommandEvent::Stop.into());

                if result.is_err() {
                    logln!("Failed to emit Stop");
                }
            }
            CommandType::SetDataOutputRate => match command.argument().try_into() {
                Ok(output_data_rate) => {
                    let result = context.data_output_rate_queue.send_data(output_data_rate);

                    if result.is_err() {
                        logln!("Failed to send data output rate");
                    }
                }
                Err(msg) => logln!("{}", msg),
            },
            CommandType::SetAccelerometerScale => match command.argument().try_into() {
                Ok(accelerometer_scale) => {
                    let result = context
                        .accelerometer_scale_queue
                        .send_data(accelerometer_scale);

                    if result.is_err() {
                        logln!("Failed to send accelerometer scale");
                    }
                }
                Err(msg) => logln!("{}", msg),
            },
            CommandType::SetGyroscopeScale => match command.argument().try_into() {
                Ok(gyroscope_scale) => {
                    let result = context.gyroscope_scale_queue.send_data(gyroscope_scale);

                    if result.is_err() {
                        logln!("Failed to set gyroscope scale");
                    }
                }
                Err(msg) => logln!("{}", msg),
            },
            CommandType::GetExecutionStats => {
                let result = api.emit_event(CommandEvent::GetExecutionStats.into());

                if result.is_err() {
                    logln!("Failed to emit GetExecutionStats");
                }
            }
        },
        None => logln!("Received malformed command"),
    };
}
