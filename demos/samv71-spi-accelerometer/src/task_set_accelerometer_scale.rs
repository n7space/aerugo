use aerugo::{logln, RuntimeApi};

#[derive(Copy, Clone, Debug)]
pub enum AccelerometerScale {
    INVALID,
    As2,
    As4,
    As8,
    As16,
}

impl From<u8> for AccelerometerScale {
    fn from(val: u8) -> Self {
        match val {
            0x01 => AccelerometerScale::As2,
            0x02 => AccelerometerScale::As4,
            0x03 => AccelerometerScale::As8,
            0x04 => AccelerometerScale::As16,
            _ => AccelerometerScale::INVALID
        }
    }
}

#[derive(Default)]
pub struct TaskSetAccelerometerScaleContext {}

pub fn task_set_accelerometer_scale(
    scale: AccelerometerScale,
    _: &mut TaskSetAccelerometerScaleContext,
    _: &'static dyn RuntimeApi,
) { 
    logln!("Set accelerometer scale: {:?}", scale);
}
