use aerugo::RuntimeApi;

#[derive(Default)]
pub struct TaskTransmitImuDataContext;

pub fn task_transmit_imu_data(
    _: (),
    _: &mut TaskTransmitImuDataContext,
    _: &'static dyn RuntimeApi,
) {
}
