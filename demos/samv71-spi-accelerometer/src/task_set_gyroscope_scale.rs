use aerugo::{logln, RuntimeApi};

#[derive(Copy, Clone, Debug)]
pub enum GyroscopeScale {
    Gs120,
    Gs250,
    Gs500,
    Gs1000,
    Gs2000,
}

impl TryFrom<u8> for GyroscopeScale {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x01 => Ok(GyroscopeScale::Gs120),
            0x02 => Ok(GyroscopeScale::Gs250),
            0x03 => Ok(GyroscopeScale::Gs500),
            0x04 => Ok(GyroscopeScale::Gs1000),
            0x05 => Ok(GyroscopeScale::Gs2000),
            _ => Err("Got unknown GyroscopeScale value"),
        }
    }
}

#[derive(Default)]
pub struct TaskSetGyroscopeScaleContext {}

pub fn task_set_gyroscope_scale(
    scale: GyroscopeScale,
    _: &mut TaskSetGyroscopeScaleContext,
    _: &'static dyn RuntimeApi,
) { 
    logln!("Set gyroscope scale: {:?}", scale);
}
