use aerugo::{logln, EventId, RuntimeApi};

#[derive(Default)]
pub struct TaskStopMeasurementsContext {}

pub fn task_stop_measurements(
    _: EventId,
    _: &mut TaskStopMeasurementsContext,
    _: &'static dyn RuntimeApi,
) {
    logln!("Stop measurements");
}
