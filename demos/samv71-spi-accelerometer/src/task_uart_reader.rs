use crate::{
    ccsds::{CCSDSPrimaryHeader, PacketType},
    telecommand::FromTelecommandArgument,
    telemetry::{InvalidTelecommandError, SetValueError, Telemetry},
    AccelerometerScale, CommandEvent, GyroscopeScale, OutputDataRate, Telecommand,
    TelecommandBuffer, TelecommandType, UART_WRITER_STORAGE,
};

use aerugo::{logln, MessageQueueHandle, RuntimeApi};

pub struct TaskUartReaderContext {
    pub data_output_rate_queue: MessageQueueHandle<OutputDataRate, 2>,
    pub accelerometer_scale_queue: MessageQueueHandle<AccelerometerScale, 2>,
    pub gyroscope_scale_queue: MessageQueueHandle<GyroscopeScale, 2>,
}

pub fn task_uart_reader(
    buffer: TelecommandBuffer,
    context: &mut TaskUartReaderContext,
    api: &'static dyn RuntimeApi,
) {
    let header = match CCSDSPrimaryHeader::try_from(&buffer[0..=5].try_into().unwrap()) {
        Ok(header) => header,
        Err(reason) => {
            Telemetry::new_invalid_telecommand_error(InvalidTelecommandError::InvalidCCSDSHeader)
                .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });
            logln!(
                "Could not parse CCSDS primary header of received telecommand ({:?}): {:02X?}",
                reason,
                buffer
            );
            return;
        }
    };

    if header.identity.packet_type != PacketType::Telecommand {
        Telemetry::new_invalid_telecommand_error(
            InvalidTelecommandError::UnexpectedTelemetryReceived,
        )
        .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });
        logln!("Received unexpected telemetry packet: {:#?}", header);
        return;
    }

    match Telecommand::from_ccsds_packet(header, &buffer[6..]) {
        Ok(command) => match command.command_type() {
            TelecommandType::Start => {
                let result = api.emit_event(CommandEvent::Start.into());

                if result.is_err() {
                    logln!("Failed to emit Start event");
                }
            }
            TelecommandType::Stop => {
                let result = api.emit_event(CommandEvent::Stop.into());

                if result.is_err() {
                    logln!("Failed to emit Stop event");
                }
            }
            TelecommandType::SetDataOutputRate => {
                match OutputDataRate::from_telecommand_argument(command.argument()) {
                    Ok(output_data_rate) => {
                        let result = context.data_output_rate_queue.send_data(output_data_rate);

                        if result.is_err() {
                            logln!("Failed to send data output rate command to queue");
                        }
                    }
                    Err(msg) => {
                        Telemetry::new_set_data_output_rate_error(SetValueError::InvalidValue)
                            .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });
                        logln!("Could not parse output data rate: {:02X}", msg)
                    }
                }
            }
            TelecommandType::SetAccelerometerScale => {
                match AccelerometerScale::from_telecommand_argument(command.argument()) {
                    Ok(accelerometer_scale) => {
                        let result = context
                            .accelerometer_scale_queue
                            .send_data(accelerometer_scale);

                        if result.is_err() {
                            logln!("Failed to send accelerometer scale command to queue");
                        }
                    }
                    Err(msg) => {
                        Telemetry::new_set_accelerometer_scale_error(SetValueError::InvalidValue)
                            .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });
                        logln!("Could not parse accelerometer scale: {:02X}", msg);
                    }
                }
            }
            TelecommandType::SetGyroscopeScale => {
                match GyroscopeScale::from_telecommand_argument(command.argument()) {
                    Ok(gyroscope_scale) => {
                        let result = context.gyroscope_scale_queue.send_data(gyroscope_scale);

                        if result.is_err() {
                            logln!("Failed to set gyroscope scale command to queue");
                        }
                    }
                    Err(msg) => {
                        Telemetry::new_set_gyroscope_scale_error(SetValueError::InvalidValue)
                            .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });
                        logln!("Could not parse gyroscope scale: {:02X}", msg);
                    }
                }
            }
            TelecommandType::GetExecutionStats => {
                let result = api.emit_event(CommandEvent::GetExecutionStats.into());

                if result.is_err() {
                    logln!("Failed to emit GetExecutionStats event");
                }
            }
        },
        Err(message) => {
            Telemetry::new_invalid_telecommand_error(InvalidTelecommandError::MalformedPacket)
                .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });
            logln!(
                "Received malformed command ({}): {:02X?}\nwith following CCSDS header: {:#?}",
                message,
                buffer,
                header
            );
        }
    };
}
