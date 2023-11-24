//! embedded-hal traits implementation.
//!
//! # SPI configuration guide for embedded-hal traits usage
//!
//! This should be a separate typestate, but there's no time for that.
//! Embedded-hal implementation is 100% blocking (polling). **SPI interrupts must be disabled in NVIC,
//! but RX/TX-related interrupts must be unmasked in via SPI driver, otherwise every operation
//! will hang.** Performing parallel XDMAC operations while using SPI is obviously very much unsafe.
//!
//! Reader, Writer and StatusReader **must be present in driver's instance**, every operation checks
//! their presence and will fail if those are missing. Flush requires only StatusReader - missing
//! Reader or Writer will not fail it, but all I/O operations require all three of them to be present.

use super::{metadata::SPIMetadata, status_reader::StatusReader, Master, Spi};
pub use embedded_hal::spi::SpiBus;
use embedded_hal::spi::{Error, ErrorKind, ErrorType};

/// SPI errors that may happen while using embedded-hal SPI trait
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SpiError {
    /// SPI reader was taken from driver's instance and it cannot be accessed. It must be present in
    /// the driver to perform embedded-hal operations.
    ReaderNotAvailable,
    /// SPI writer was taken from driver's instance and it cannot be accessed. It must be present in
    /// the driver to perform embedded-hal operations.
    WriterNotAvailable,
    /// SPI status reader was taken from driver's instance and it cannot be accessed. It must be
    /// present in the driver to perform embedded-hal operations.
    StatusReaderNotAvailable,
    /// Mode fault error.
    ModeFault,
    /// Overrun error.
    Overrun,
}

impl Error for SpiError {
    fn kind(&self) -> ErrorKind {
        match self {
            SpiError::ReaderNotAvailable => ErrorKind::Other,
            SpiError::WriterNotAvailable => ErrorKind::Other,
            SpiError::StatusReaderNotAvailable => ErrorKind::Other,
            SpiError::ModeFault => ErrorKind::ModeFault,
            SpiError::Overrun => ErrorKind::Overrun,
        }
    }
}

impl<Instance: SPIMetadata> ErrorType for Spi<Instance, Master> {
    type Error = SpiError;
}

impl<Instance: SPIMetadata> SpiBus<u8> for Spi<Instance, Master> {
    /// Reads data from currently selected SPI device by performing a transaction with dummy (`0`)
    /// data transmitted until `words` is filled with received data.
    /// Blocks until the transaction is finished (last word is received).
    /// Flushing after read is not necessary.
    ///
    /// # Parameters
    /// * `words` - Slice for incoming data. It's length determines transfer length.
    ///
    /// # Returns
    /// Ok(()) on success, [`SpiError`] on SPI error.
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let status_reader = self.get_status_reader_ref()?;
        self.check_io_presence()?;

        for word in words {
            self.transmit_value(0);
            status_reader
                .wait_for_status(|status| status.interrupts.rx_data_register_full, usize::MAX);
            *word = self.get_received_data() as u8;
        }

        Ok(())
    }

    /// Writes provided `words` to currently selected SPI device.
    /// Reads and discards the received data.
    /// Blocks until the transaction is finished (last word is put in TX register).
    /// You may want to flush after this operation to make sure that the last byte is actually
    /// transmitted.
    ///
    /// # Parameters
    /// * `words` - Slice of words to be transmitted.
    ///
    /// # Remarks
    /// This function may behave unexpectedly in some scenarios, as it waits for RX data register
    /// full event, instead of TX data register empty event, as the latter doesn't happen after
    /// first word is transmitted for unknown reasons in some SPI configurations. The issue may
    /// happen if the device expects the write to be a single transaction, and SPI is configured in
    /// a way that will make every word to be a separate transaction with CS deactivations in
    /// between. Verify if this function works for your scenario, or use `transfer`/`transfer_in_place`
    /// for writing.
    ///
    /// # Returns
    /// Ok(()) on success, [`SpiError`] on SPI error.
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        let status_reader = self.get_status_reader_ref()?;
        self.check_io_presence()?;

        for &word in words {
            self.transmit_value(word as u16);
            // This should, in theory, check the state of TX data register. However, when it does
            // that, the transfer hangs at 2nd word, as the flag never rises again. A workaround
            // is to wait for RX data flag, as it'll be risen after the word is transmitted, but it's
            // not guaranteed to work in every case (as it doesn't guarantee that the transfer will
            // be a single SPI transaction, which may sometimes be necessary).
            status_reader
                .wait_for_status(|status| status.interrupts.rx_data_register_full, usize::MAX);
            // Dummy read to prevent overrun error
            self.get_received_data();
        }

        Ok(())
    }

    /// Performs an SPI transfer to currently selected device.
    /// Blocks until the transaction is finished (last word is received, which also implies that
    /// last word is transmitted).
    /// Flushing is not necessary after this operation.
    ///
    /// # Parameters
    /// * `read` - Slice for incoming data.
    /// * `write` - Slice of words to be transmitted.
    ///
    /// # Returns
    /// Ok(()) on success, [`SpiError`] on SPI error.
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let status_reader = self.get_status_reader_ref()?;
        self.check_io_presence()?;

        // At the start of transaction, if it's length is >= 2, two words must be put into SPI
        // buffers to fill both the FIFO and TX holding register. TX holding register must be fed
        // as quickly as possible, and at the same time, data from RX holding register must be
        // read as soon as the RX full flag rises.

        // In other words, the loop must feed the TX register and read from RX register at the same
        // time, as soon as possible, so a normal iter-chain approach is replaced with following:
        let mut read_index = 0usize;
        let mut write_index = 0usize;
        let mut write_in_progress = true;
        let mut read_in_progress = true;

        while write_in_progress || read_in_progress {
            let status = status_reader.status();

            if write_in_progress && status.interrupts.tx_data_register_empty {
                if write_index < write.len() {
                    self.transmit_value(write[write_index] as u16);
                    write_index += 1;
                }

                // Check post-increment to immediately notify about end-of-write
                if write_index >= write.len() {
                    write_in_progress = false;
                }
            }

            if read_in_progress && status.interrupts.rx_data_register_full {
                if read_index < read.len() {
                    read[read_index] = self.get_received_data() as u8;
                    read_index += 1;
                }

                // Check post-increment to immediately notify about end-of-read
                if read_index >= read.len() {
                    read_in_progress = false;
                }
            }

            if status.interrupts.overrun_error {
                return Err(SpiError::Overrun);
            }

            if status.interrupts.mode_fault_error {
                return Err(SpiError::ModeFault);
            }
        }

        Ok(())
    }

    /// Performs an SPI transfer to currently selected device.
    /// Blocks until the transaction is finished (last word is received, which also implies that
    /// last word is transmitted).
    /// Flushing is not necessary after this operation.
    ///
    /// # Parameters
    /// * `words` - Slice used for transfer, should contain data to be transmitted and will be
    ///             filled with received data.
    ///
    /// # Returns
    /// Ok(()) on success, [`SpiError`] on SPI error.
    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let status_reader = self.get_status_reader_ref()?;
        self.check_io_presence()?;

        // Since the buffer is shared, we cannot just re-use the `transfer` implementation.
        // Instead, index-based approach will be used. Memory should never be violated, as data
        // is always transmitted before being received.
        let mut read_index = 0usize;
        let mut write_index = 0usize;
        let mut write_in_progress = true;
        let mut read_in_progress = true;

        while write_in_progress || read_in_progress {
            let status = status_reader.status();

            if write_in_progress && status.interrupts.tx_data_register_empty {
                if write_index < words.len() {
                    self.transmit_value(words[write_index] as u16);
                    write_index += 1;
                }

                // Check post-increment to immediately notify about end-of-write
                if write_index >= words.len() {
                    write_in_progress = false;
                }
            }

            if read_in_progress && status.interrupts.rx_data_register_full {
                if read_index < words.len() {
                    words[read_index] = self.get_received_data() as u8;
                    read_index += 1;
                }

                // Check post-increment to immediately notify about end-of-read
                if read_index >= words.len() {
                    read_in_progress = false;
                }
            }

            if status.interrupts.overrun_error {
                return Err(SpiError::Overrun);
            }

            if status.interrupts.mode_fault_error {
                return Err(SpiError::ModeFault);
            }
        }

        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.get_status_reader_ref()?
            .wait_for_status(|status| status.interrupts.tx_registers_empty, usize::MAX);
        Ok(())
    }
}

/// Additional functions used by SpiBus
impl<Instance: SPIMetadata> Spi<Instance, Master> {
    /// Returns a reference to status reader, or `SpiError` if it's missing.
    ///
    /// This function is to be used in `embedded-hal` implementation.
    fn get_status_reader_ref(&self) -> Result<&StatusReader<Instance>, SpiError> {
        self.status_reader
            .as_ref()
            .ok_or(SpiError::StatusReaderNotAvailable)
    }

    /// Checks if both reader and writer are currently stored in SPI object, returns [`SpiError`]
    /// if either is missing.
    ///
    /// Reader and writer are not used directly due to issues with ownership.
    ///
    /// This function is to be used in `embedded-hal` implementation.
    fn check_io_presence(&self) -> Result<(), SpiError> {
        if !self.is_reader_available() {
            return Err(SpiError::ReaderNotAvailable);
        }
        if !self.is_writer_available() {
            return Err(SpiError::WriterNotAvailable);
        }
        Ok(())
    }

    /// Sets the value that will be transmitted via SPI by putting it in TX data register.
    ///
    /// # Remarks
    /// There's no runtime check for value width, so you have to make sure that you don't try to
    /// send data wider than configured data width for currently selected chip.
    fn transmit_value(&self, value: u16) {
        Instance::registers().tdr.write(|w| w.td().variant(value));
    }

    /// Returns the data currently stored in RX data register.
    ///
    /// Before reading the data you should make sure that it's available by checking
    /// `rx_data_register_full` field of [`SpiStatus`](super::status::SpiStatus) structure.
    fn get_received_data(&self) -> u16 {
        Instance::registers().rdr.read().rd().bits()
    }
}
