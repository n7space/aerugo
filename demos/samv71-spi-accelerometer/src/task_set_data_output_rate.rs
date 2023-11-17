use aerugo::{logln, RuntimeApi};

#[derive(Copy, Clone, Debug)]
pub enum OutputDataRate {
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

impl TryFrom<u8> for OutputDataRate {
    type Error = &'static str;
        
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x01 => Ok(OutputDataRate::Odr12),
            0x02 => Ok(OutputDataRate::Odr26),
            0x03 => Ok(OutputDataRate::Odr52),
            0x04 => Ok(OutputDataRate::Odr104),
            0x05 => Ok(OutputDataRate::Odr208),
            0x06 => Ok(OutputDataRate::Odr416),
            0x07 => Ok(OutputDataRate::Odr833),
            0x08 => Ok(OutputDataRate::Odr1666),
            0x09 => Ok(OutputDataRate::Odr3332),
            0x0A => Ok(OutputDataRate::Odr6664),
            _ => Err("Got unknown OutpuDataRate value")
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
