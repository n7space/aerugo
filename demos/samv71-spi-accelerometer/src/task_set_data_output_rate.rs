use aerugo::{logln, RuntimeApi};

#[derive(Copy, Clone, Debug)]
pub enum OutputDataRate {
    INVALID,
    ODR12,
    ODR26,
    ODR52,
    ODR104,
    ODR208,
    ODR416,
    ODR833,
    ODR1666,
    ODR3332,
    ODR6664,
}

impl From<u8> for OutputDataRate {
    fn from(val: u8) -> Self {
        match val {
            0x01 => OutputDataRate::ODR12,
            0x02 => OutputDataRate::ODR26,
            0x03 => OutputDataRate::ODR52,
            0x04 => OutputDataRate::ODR104,
            0x05 => OutputDataRate::ODR208,
            0x06 => OutputDataRate::ODR416,
            0x07 => OutputDataRate::ODR833,
            0x08 => OutputDataRate::ODR1666,
            0x09 => OutputDataRate::ODR3332,
            0x0A => OutputDataRate::ODR6664,
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
