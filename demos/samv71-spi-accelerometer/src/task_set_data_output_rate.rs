use aerugo::{logln, RuntimeApi};

#[derive(Copy, Clone, Debug)]
pub enum OutputDataRate {
    Odr12_5Hz,
    Odr26Hz,
    Odr52Hz,
    Odr104Hz,
    Odr208Hz,
    Odr416Hz,
    Odr833Hz,
    Odr1667Hz,
    Odr3333Hz,
    Odr6667Hz,
}

#[derive(Default)]
pub struct TaskSetDataOutputRateContext {}

pub fn task_set_data_output_rate(
    output_rate: OutputDataRate,
    _: &mut TaskSetDataOutputRateContext,
    _: &'static dyn RuntimeApi,
) {
    logln!("Set data output rate: {:?}", output_rate)
}
