use aerugo::{logln, EventId, RuntimeApi};

use crate::{telemetry::Telemetry, TASKLET_MAP, UART_WRITER_STORAGE};

#[derive(Default)]
pub struct TaskGetExecutionStatsContext {}

pub fn task_get_execution_stats(
    _: EventId,
    _: &mut TaskGetExecutionStatsContext,
    api: &'static dyn RuntimeApi,
) {
    let tasklet_map = unsafe { &TASKLET_MAP.unwrap() };
    for (name, id) in tasklet_map {
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
