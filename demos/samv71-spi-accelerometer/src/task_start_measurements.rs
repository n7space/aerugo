use aerugo::{logln, EventId, RuntimeApi};

#[derive(Default)]
pub struct TaskStartMeasurementsContext {}

pub fn task_start_measurements(
    _: EventId,
    _: &mut TaskStartMeasurementsContext,
    _: &'static dyn RuntimeApi,
) {
    logln!("Start measurements");
}
