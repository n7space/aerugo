use aerugo::{logln, RuntimeApi};

#[derive(Copy, Clone, Debug)]
pub enum OutputDataRate {
    INVALID,
    Odr12,
    Odr26,
    Odr52,
    Odr104,
    Odr208,
    Odr416,
    Odr833,
    Odr1666,
    Odr3332,
    Odr6664,
}

impl From<u8> for OutputDataRate {
    fn from(val: u8) -> Self {
        match val {
            0x01 => OutputDataRate::Odr12,
            0x02 => OutputDataRate::Odr26,
            0x03 => OutputDataRate::Odr52,
            0x04 => OutputDataRate::Odr104,
            0x05 => OutputDataRate::Odr208,
            0x06 => OutputDataRate::Odr416,
            0x07 => OutputDataRate::Odr833,
            0x08 => OutputDataRate::Odr1666,
            0x09 => OutputDataRate::Odr3332,
            0x0A => OutputDataRate::Odr6664,
            _ => OutputDataRate::INVALID,
        }
    }
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
