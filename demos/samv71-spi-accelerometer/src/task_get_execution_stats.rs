use aerugo::{logln, EventId, RuntimeApi};

use crate::{telemetry::Telemetry, UART_WRITER_STORAGE};

#[derive(Default)]
pub struct TaskGetExecutionStatsContext {}

pub fn task_get_execution_stats(
    _: EventId,
    _: &mut TaskGetExecutionStatsContext,
    _: &'static dyn RuntimeApi,
) {
    Telemetry::new_execution_statistics()
        .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });
    logln!("Get execution stats")
}
