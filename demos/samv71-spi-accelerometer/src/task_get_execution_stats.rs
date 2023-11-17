use aerugo::{logln, RuntimeApi, EventId};

#[derive(Default)]
pub struct TaskGetExecutionStatsContext {}

pub fn task_get_execution_stats(
    _: EventId,
    _: &mut TaskGetExecutionStatsContext,
    _: &'static dyn RuntimeApi,
) { 
    logln!("Get execution stats")
}
