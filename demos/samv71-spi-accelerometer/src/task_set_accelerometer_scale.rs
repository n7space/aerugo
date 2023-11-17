use aerugo::{logln, RuntimeApi};

#[derive(Copy, Clone, Debug)]
pub enum AccelerometerScale {
    As2,
    As4,
    As8,
    As16,
}

impl TryFrom<u8> for AccelerometerScale {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x01 => Ok(AccelerometerScale::As2),
            0x02 => Ok(AccelerometerScale::As4),
            0x03 => Ok(AccelerometerScale::As8),
            0x04 => Ok(AccelerometerScale::As16),
            _ => Err("Got unknown AccelerometerScale value"),
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
