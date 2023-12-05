use aerugo::{logln, EventId, RuntimeApi};

use crate::{telemetry::Telemetry, TaskletMap, UART_WRITER_STORAGE};

pub struct TaskGetExecutionStatsContext {
    pub tasklet_map: TaskletMap,
}

pub fn task_get_execution_stats(
    _: EventId,
    context: &mut TaskGetExecutionStatsContext,
    api: &'static dyn RuntimeApi,
) {
    for (name, id) in &context.tasklet_map {
        match api.get_execution_statistics(id) {
            Some(stats) => {
                Telemetry::new_execution_statistics(*name, stats)
                    .write_ccsds_packet(unsafe { UART_WRITER_STORAGE.as_mut().unwrap() });
                logln!("Execution stats for tasklet {:?} sent!", name);
            }
            None => {
                logln!("Could not fetch execution stats for tasklet {:?}", name);
            }
        }
    }
}
