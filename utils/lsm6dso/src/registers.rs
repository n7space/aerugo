//! LSM6DSO registers-related definitions.

/// LSM6DSO registers and their addresses.
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum Register {
    FUNC_CFG_ACCESS = 0x01,
    PIN_CTRL = 0x02,
    FIFO_CTRL1 = 0x07,
    FIFO_CTRL2 = 0x08,
    FIFO_CTRL3 = 0x09,
    FIFO_CTRL4 = 0x0A,
    COUNTER_BDR_REG1 = 0x0B,
    COUNTER_BDR_REG2 = 0x0C,
    INT1_CTRL = 0x0D,
    INT2_CTRL = 0x0E,
    WHO_AM_I = 0x0F,
    CTRL1_XL = 0x10,
    CTRL2_G = 0x11,
    CTRL3_C = 0x12,
    CTRL4_C = 0x13,
    CTRL5_C = 0x14,
    CTRL6_C = 0x15,
    CTRL7_G = 0x16,
    CTRL8_XL = 0x17,
    CTRL9_XL = 0x18,
    CTRL10_C = 0x19,
    ALL_INT_SRC = 0x1A,
    WAKE_UP_SRC = 0x1B,
    TAP_SRC = 0x1C,
    D6D_SRC = 0x1D,
    STATUS_REG = 0x1E,
    OUT_TEMP_L = 0x20,
    OUT_TEMP_H = 0x21,
    OUTX_L_G = 0x22,
    OUTX_H_G = 0x23,
    OUTY_L_G = 0x24,
    OUTY_H_G = 0x25,
    OUTZ_L_G = 0x26,
    OUTZ_H_G = 0x27,
    OUTX_L_A = 0x28,
    OUTX_H_A = 0x29,
    OUTY_L_A = 0x2A,
    OUTY_H_A = 0x2B,
    OUTZ_L_A = 0x2C,
    OUTZ_H_A = 0x2D,
    EMB_FUNC_STATUS_MAINPAGE = 0x35,
    FSM_STATUS_A_MAINPAGE = 0x36,
    FSM_STATUS_B_MAINPAGE = 0x37,
    STATUS_MASTER_MAINPAGE = 0x39,
    FIFO_STATUS1 = 0x3A,
    FIFO_STATUS2 = 0x3B,
    TIMESTAMP0 = 0x40,
    TIMESTAMP1 = 0x41,
    TIMESTAMP2 = 0x42,
    TIMESTAMP3 = 0x43,
    TAP_CFG0 = 0x56,
    TAP_CFG1 = 0x57,
    TAP_CFG2 = 0x58,
    TAP_THS_6D = 0x59,
    INT_DUR2 = 0x5A,
    WAKE_UP_THS = 0x5B,
    WAKE_UP_DUR = 0x5C,
    FREE_FALL = 0x5D,
    MD1_CFG = 0x5E,
    MD2_CFG = 0x5F,
    I2C_BUS_AVB = 0x62,
    INTERNAL_FREQ_FINE = 0x63,
    INT_OIS = 0x6F,
    CTRL1_OIS = 0x70,
    CTRL2_OIS = 0x71,
    CTRL3_OIS = 0x72,
    X_OFS_USR = 0x73,
    Y_OFS_USR = 0x74,
    Z_OFS_USR = 0x75,
    FIFO_DATA_OUT_TAG = 0x78,
    FIFO_DATA_X_L = 0x79,
    FIFO_DATA_X_H = 0x7A,
    FIFO_DATA_Y_L = 0x7B,
    FIFO_DATA_Y_H = 0x7C,
    FIFO_DATA_Z_L = 0x7D,
    FIFO_DATA_Z_H = 0x7E,
}

/// Expected value of WHO_AM_I register
pub const WHO_AM_I_VALUE: u8 = 0x6C;

/// Trait for single-register fields
pub(crate) trait RegisterField
where
    Self: Copy,
{
    /// Field mask, per datasheet (as-in register).
    const MASK: u8;
    /// Offset of the field's LSB to register's LSB.
    const OFFSET: usize;
}

/// Trait for fields that span multiple registers. The order of masks and offsets must be defined
/// respective to the order of registers this field spans, smaller address first.
pub(crate) trait MultiRegisterField<const REGISTER_SPAN: usize = 2>
where
    Self: Copy,
{
    /// Field masks, per datasheet (as-in register).
    const MASKS: [u8; REGISTER_SPAN];
    /// Offsets of the field's LSB from register's LSB.
    const OFFSETS: [usize; REGISTER_SPAN];
}

pub(crate) trait RegisterConversion
where
    Self: Copy + RegisterField,
{
    /// This function should return the value of current register field that can be OR'd with
    /// register's content to set it.
    fn to_reg(self) -> u8;

    /// This function should extract the field's value from the register and return it.
    fn from_reg(reg: u8) -> Self;
}

pub trait MultiRegisterConversion<const REGISTER_SPAN: usize = 2>
where
    Self: Copy + MultiRegisterField<REGISTER_SPAN>,
{
    /// This function should return the value of provided registers as an array. Unused bits should
    /// remain 0.
    fn to_regs(self) -> [u8; REGISTER_SPAN];

    /// This function should extract the field's value from the register and return it.
    fn from_regs(regs: &[u8]) -> Self;
}
