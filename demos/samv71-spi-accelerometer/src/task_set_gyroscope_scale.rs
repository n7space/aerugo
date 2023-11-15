use aerugo::{logln, RuntimeApi};

#[derive(Copy, Clone, Debug)]
pub enum GyroscopeScale {
    Invalid,
    Gs120,
    Gs250,
    Gs500,
    Gs1000,
    Gs2000,
}

impl From<u8> for GyroscopeScale {
    fn from(val: u8) -> Self {
        match val {
            0x01 => GyroscopeScale::Gs120,
            0x02 => GyroscopeScale::Gs250,
            0x03 => GyroscopeScale::Gs500,
            0x04 => GyroscopeScale::Gs1000,
            0x05 => GyroscopeScale::Gs2000,
            _ => GyroscopeScale::Invalid,
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
