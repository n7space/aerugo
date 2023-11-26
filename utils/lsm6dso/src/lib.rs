#![no_std]
//! LSM6DSO driver library.
//!
//! This library is made specifically for usage with Aerugo SAMV71 HAL. For this reason, it
//! requires [`SpiBus`] instead of [`SpiDevice`](embedded_hal::spi::SpiDevice), as SAMV71 HAL
//! currently has only the [`SpiBus`] trait implementation.
//!
//! When SAMV71 HAL will support [`SpiDevice`](embedded_hal::spi::SpiDevice), this library should
//! be refactored to use it, instead of the whole bus.

extern crate derive_more;
extern crate embedded_hal;
extern crate paste;

mod bounded_int;
pub mod config;
pub(crate) mod registers;

use config::{
    control::{
        AccelerometerConfig, AccelerometerConfigBuffer, GyroscopeConfig, GyroscopeConfigBuffer,
    },
    fifo::{FifoConfig, FifoConfigBuffer},
    interrupts::{INT1Interrupts, INT2Interrupts, InterruptConfigBuffer},
};
use registers::{Register, WHO_AM_I_VALUE};

pub use embedded_hal::spi::SpiBus;

/// LSM6DSO driver structure.
///
/// # Generic parameters
/// * `SPI` - An SPI bus instance that LSM6DSO driver will use to communicate with the sensor
pub struct LSM6DSO<SPI: SpiBus, const BUFFER_SIZE: usize = 32> {
    /// SPI instance.
    spi: SPI,
    /// Buffer for internal operations
    buffer: [u8; BUFFER_SIZE],
}

/// Constant representing a mask applied to value being read from LSM6DSO via SPI
const READ_REQUEST_MASK: u8 = 0x80;

impl<SPI: SpiBus, const BUFFER_SIZE: usize> LSM6DSO<SPI, { BUFFER_SIZE }> {
    /// Creates new LSM6DSO instance. Consumes the SPI bus.
    ///
    /// SPI must be configured with following settings for LSM6DSO to work correctly:
    /// * embedded-hal compatible config must be applied (NVIC SPI IRQ off, RX/TX SPI IRQs enabled)
    /// * Clock polarity: high when inactive
    /// * Clock phase: data changed on leading edge
    /// * Chip select behavior: Deactivate after last transfer
    /// * Bits per transfer: 8
    /// * Serial clock divider: Serial clock must be less than 10MHz
    /// * Delay before first clock and consecutive transfers: 0
    pub fn new(spi: SPI) -> Result<Self, SPI::Error> {
        assert!(
            BUFFER_SIZE >= 2,
            "LSM6DSO buffer must be at least 2 bytes long"
        );
        let mut lsm = Self {
            spi,
            buffer: [0; BUFFER_SIZE],
        };
        // First transaction usually fails to communicate with LSM and returns junk, so it's done
        // here to prevent failing user operations.
        lsm.id()?;
        Ok(lsm)
    }

    /// Returns the ID read from WHO_AM_I register of the sensor.
    /// Should be equal to [`WHO_AM_I_VALUE`].
    pub fn id(&mut self) -> Result<u8, SPI::Error> {
        self.read_register(Register::WHO_AM_I)
    }

    /// Returns `true` if sensor responds with valid ID.
    pub fn is_alive(&mut self) -> Result<bool, SPI::Error> {
        Ok(self.id()? == WHO_AM_I_VALUE)
    }

    pub fn set_fifo_config(&mut self, config: FifoConfig) -> Result<(), SPI::Error> {
        let fifo_config_regs: FifoConfigBuffer = config.into();
        self.write_registers(Register::FIFO_CTRL1, &fifo_config_regs)?;
        Ok(())
    }

    pub fn get_fifo_config(&mut self) -> Result<FifoConfig, SPI::Error> {
        let mut buffer = [0u8, 0, 0, 0];
        self.read_registers(Register::FIFO_CTRL1, &mut buffer)?;
        Ok(buffer.into())
    }

    pub fn set_int1_interrupts(&mut self, interrupts: INT1Interrupts) -> Result<(), SPI::Error> {
        let config_reg: InterruptConfigBuffer = interrupts.into();
        self.write_register(Register::INT1_CTRL, config_reg)?;
        Ok(())
    }

    pub fn get_int1_interrupts(&mut self) -> Result<INT1Interrupts, SPI::Error> {
        Ok(self.read_register(Register::INT1_CTRL)?.into())
    }

    pub fn set_int2_interrupts(&mut self, interrupts: INT2Interrupts) -> Result<(), SPI::Error> {
        let config_reg: InterruptConfigBuffer = interrupts.into();
        self.write_register(Register::INT2_CTRL, config_reg)?;
        Ok(())
    }

    pub fn get_int2_interrupts(&mut self) -> Result<INT2Interrupts, SPI::Error> {
        Ok(self.read_register(Register::INT2_CTRL)?.into())
    }

    pub fn set_accelerometer_config(
        &mut self,
        config: AccelerometerConfig,
    ) -> Result<(), SPI::Error> {
        let config_reg: AccelerometerConfigBuffer = config.into();
        self.write_register(Register::CTRL1_XL, config_reg)?;
        Ok(())
    }

    pub fn get_accelerometer_config(&mut self) -> Result<AccelerometerConfig, SPI::Error> {
        Ok(self.read_register(Register::CTRL1_XL)?.into())
    }

    pub fn set_gyroscope_config(&mut self, config: GyroscopeConfig) -> Result<(), SPI::Error> {
        let config_reg: GyroscopeConfigBuffer = config.into();
        self.write_register(Register::CTRL2_G, config_reg)?;
        Ok(())
    }

    pub fn get_gyroscope_config(&mut self) -> Result<GyroscopeConfig, SPI::Error> {
        Ok(self.read_register(Register::CTRL2_G)?.into())
    }

    pub fn reboot_memory_content(&mut self) -> Result<(), SPI::Error> {
        const REBOOT_MEMORY_BIT_MASK: u8 = 0x80;
        let ctrl_reg = self.read_register(Register::CTRL3_C)? | REBOOT_MEMORY_BIT_MASK;
        self.write_register(Register::CTRL3_C, ctrl_reg)?;
        Ok(())
    }

    pub fn software_reset(&mut self) -> Result<(), SPI::Error> {
        const RESET_BIT_MASH: u8 = 0x01;
        let ctrl_reg = self.read_register(Register::CTRL3_C)? | RESET_BIT_MASH;
        self.write_register(Register::CTRL3_C, ctrl_reg)?;
        Ok(())
    }

    pub fn get_reg(&mut self) -> u8 {
        self.read_register(Register::CTRL1_XL).unwrap()
    }

    /// Reads the value from a single LSM6DSO register and returns it.
    fn read_register(&mut self, register: Register) -> Result<u8, SPI::Error> {
        let mut data_buffer = [READ_REQUEST_MASK | (register as u8), 0];
        self.spi.transfer_in_place(&mut data_buffer)?;
        Ok(data_buffer[1])
    }

    /// Writes a value to LSM6DSO register.
    fn write_register(&mut self, register: Register, value: u8) -> Result<(), SPI::Error> {
        let mut write_request = [register as u8, value];
        self.spi.transfer_in_place(&mut write_request)?;
        Ok(())
    }

    /// Read the value from multiple registers, starting with specified one.
    /// Amount of read registers depends on the size of the buffer slice.
    /// Buffer must be at least 2 bytes long, otherwise this function will panic.
    fn read_registers(
        &mut self,
        first_register: Register,
        buffer: &mut [u8],
    ) -> Result<(), SPI::Error> {
        let user_buffer_length = buffer.len();
        // at least 1 byte for data
        assert!(user_buffer_length > 0, "provided buffer is too small");
        // +1 byte for address
        assert!(
            BUFFER_SIZE > user_buffer_length,
            "LSM6DSO buffer is too small for this operation"
        );

        // Prepare transfer
        self.buffer[0] = READ_REQUEST_MASK | (first_register as u8);
        // Get data from sensor
        self.spi
            .transfer_in_place(&mut self.buffer[0..user_buffer_length])?;
        // Copy to user's buffer
        buffer.copy_from_slice(&self.buffer[1..=user_buffer_length]);

        Ok(())
    }

    /// Writes multiple values to LSM6DSO registers.
    fn write_registers(
        &mut self,
        first_register: Register,
        values: &[u8],
    ) -> Result<(), SPI::Error> {
        let user_buffer_length = values.len();
        // at least 1 byte for data
        assert!(user_buffer_length > 0, "provided buffer is too small");
        // +1 byte for address
        assert!(
            BUFFER_SIZE > user_buffer_length,
            "LSM6DSO buffer is too small for this operation"
        );

        // Prepare transfer
        self.buffer[0] = first_register as u8;
        self.buffer[1..=user_buffer_length].copy_from_slice(values);
        // Write data
        self.spi
            .transfer_in_place(&mut self.buffer[0..=user_buffer_length])?;
        Ok(())
    }
}
