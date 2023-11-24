pub struct INT1Interrupts {
    pub data_ready: bool,
    pub counter_bdr: bool,
    pub fifo_full: bool,
    pub fifo_overrun: bool,
    pub fifo_threshold: bool,
    pub boot: bool,
    pub gyro_data_ready: bool,
    pub accel_data_ready: bool,
}

pub struct INT2Interrupts {
    pub counter_bdr: bool,
    pub fifo_full: bool,
    pub fifo_overrun: bool,
    pub fifo_threshold: bool,
    pub temperature_data_ready: bool,
    pub gyro_data_ready: bool,
    pub accel_data_ready: bool,
}

pub type InterruptConfigBuffer = u8;

impl From<INT1Interrupts> for InterruptConfigBuffer {
    fn from(config: INT1Interrupts) -> Self {
        config.accel_data_ready as u8
            | ((config.gyro_data_ready as u8) << 1)
            | ((config.boot as u8) << 2)
            | ((config.fifo_threshold as u8) << 3)
            | ((config.fifo_overrun as u8) << 4)
            | ((config.fifo_full as u8) << 5)
            | ((config.counter_bdr as u8) << 6)
            | ((config.data_ready as u8) << 7)
    }
}

impl From<INT2Interrupts> for InterruptConfigBuffer {
    fn from(config: INT2Interrupts) -> Self {
        config.accel_data_ready as u8
            | ((config.gyro_data_ready as u8) << 1)
            | ((config.temperature_data_ready as u8) << 2)
            | ((config.fifo_threshold as u8) << 3)
            | ((config.fifo_overrun as u8) << 4)
            | ((config.fifo_full as u8) << 5)
            | ((config.counter_bdr as u8) << 6)
    }
}

impl From<InterruptConfigBuffer> for INT1Interrupts {
    fn from(value: InterruptConfigBuffer) -> Self {
        INT1Interrupts {
            data_ready: value & 0x80 != 0,
            counter_bdr: value & 0x40 != 0,
            fifo_full: value & 0x20 != 0,
            fifo_overrun: value & 0x10 != 0,
            fifo_threshold: value & 0x08 != 0,
            boot: value & 0x04 != 0,
            gyro_data_ready: value & 0x02 != 0,
            accel_data_ready: value & 0x01 != 0,
        }
    }
}

impl From<InterruptConfigBuffer> for INT2Interrupts {
    fn from(value: InterruptConfigBuffer) -> Self {
        INT2Interrupts {
            counter_bdr: value & 0x40 != 0,
            fifo_full: value & 0x20 != 0,
            fifo_overrun: value & 0x10 != 0,
            fifo_threshold: value & 0x08 != 0,
            temperature_data_ready: value & 0x04 != 0,
            gyro_data_ready: value & 0x02 != 0,
            accel_data_ready: value & 0x01 != 0,
        }
    }
}
