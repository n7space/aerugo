use crate::{
    config::templates::register_enum,
    registers::{FromRegister, ToRegister},
};

register_enum!(AccelerometerDataRate [mask=0xF0, offset=4] {
    PowerDown = 0b0000,
    /// This is 1.6Hz only in low-power mode. Otherwise, it's 12.5Hz.
    Rate1_6Hz = 0b1011,
    Rate12_5Hz = 0b0001,
    Rate26Hz = 0b0010,
    Rate52Hz = 0b0011,
    Rate104Hz = 0b0100,
    Rate208Hz = 0b0101,
    Rate416Hz = 0b0110,
    Rate833Hz = 0b0111,
    Rate1667Hz = 0b1000,
    Rate3333Hz = 0b1001,
    Rate6667Hz = 0b1010,
});

register_enum!(AccelerometerScale [mask=0x0C, offset=2] {
    Scale2g = 0b00,
    /// This is 16g only when full-scale mode is active. Otherwise, it's 2g.
    Scale16g = 0b01,
    Scale4g = 0b10,
    Scale8g = 0b11,
});

register_enum!(AccelerometerOutputSelection [mask=0x02, offset=1] {
    FirstStageFilter = 0,
    LPF2SecondFilter = 1,
});

register_enum!(GyroscopeDataRate [mask=0xF0, offset=4] {
    PowerDown = 0b0000,
    Rate12_5Hz = 0b0001,
    Rate26Hz = 0b0010,
    Rate52Hz = 0b0011,
    Rate104Hz = 0b0100,
    Rate208Hz = 0b0101,
    Rate416Hz = 0b0110,
    Rate833Hz = 0b0111,
    Rate1667Hz = 0b1000,
    Rate3333Hz = 0b1001,
    Rate6667Hz = 0b1010,
});

register_enum!(GyroscopeScale [mask=0x0E, offset=1] {
    Scale125dps = 0b001,
    Scale250dps = 0b000,
    Scale500dps = 0b010,
    Scale1000dps = 0b100,
    Scale2000dps = 0b110,
});

register_enum!(RebootMemoryContent [mask=0x80, offset=7] {
    Yes = 1,
});

register_enum!(IrqActivationLevel [mask=0x20, offset=5] {
    ActiveHigh = 0,
    ActiveLow = 1,
});

register_enum!(IrqPinMode [mask=0x20, offset=5] {
    PushPull = 0,
    OpenDrain = 1,
});

register_enum!(SoftwareReset [mask=0x01, offset=0] {
    Yes = 1,
});

register_enum!(DataReadyState [mask=0x08, offset=3] {
    Disabled = 0,
    Enabled = 1,
});

register_enum!(GyroscopeTestMode [mask=0x0C, offset=2] {
    Normal = 0b00,
    Positive = 0b01,
    Negative = 0b11,
});

register_enum!(AccelerometerTestMode [mask=0x03, offset=0] {
    Normal = 0b00,
    Positive = 0b01,
    Negative = 0b10,
});

pub type AccelerometerConfigBuffer = u8;
pub type GyroscopeConfigBuffer = u8;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AccelerometerConfig {
    pub data_rate: AccelerometerDataRate,
    pub scale: AccelerometerScale,
    pub output_selection: AccelerometerOutputSelection,
}

impl From<AccelerometerConfig> for AccelerometerConfigBuffer {
    fn from(config: AccelerometerConfig) -> Self {
        config.output_selection.to_reg() | config.scale.to_reg() | config.data_rate.to_reg()
    }
}

impl From<AccelerometerConfigBuffer> for AccelerometerConfig {
    fn from(value: AccelerometerConfigBuffer) -> Self {
        AccelerometerConfig {
            data_rate: AccelerometerDataRate::from_reg(value),
            scale: AccelerometerScale::from_reg(value),
            output_selection: AccelerometerOutputSelection::from_reg(value),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GyroscopeConfig {
    pub data_rate: GyroscopeDataRate,
    pub scale: GyroscopeScale,
}

impl From<GyroscopeConfig> for GyroscopeConfigBuffer {
    fn from(config: GyroscopeConfig) -> Self {
        config.scale.to_reg() | config.data_rate.to_reg()
    }
}

impl From<GyroscopeConfigBuffer> for GyroscopeConfig {
    fn from(value: GyroscopeConfigBuffer) -> Self {
        GyroscopeConfig {
            data_rate: GyroscopeDataRate::from_reg(value),
            scale: GyroscopeScale::from_reg(value),
        }
    }
}
