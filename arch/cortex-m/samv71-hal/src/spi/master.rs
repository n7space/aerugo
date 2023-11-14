//! Implementation of SPI driver functionality in Master mode.

use super::{
    chip_select_config::{ChipSelectBehavior, ChipSelectConfig, SerialClockDivider},
    config::SelectedChip,
    interrupts::Interrupts,
    metadata::SPIMetadata,
    reader::Reader,
    writer::Writer,
    Master, Spi,
};

/// Enumeration listing SPI errors
pub enum SpiError {
    /// Transaction tried to be started on unconfigured chip.
    ChipNotConfigured,
}

impl<Instance: SPIMetadata> Spi<Instance, Master> {
    /// Sets interrupts state.
    /// Interrupts set to `true` will be enabled, and set to `false` - disabled.
    pub fn set_interrupts_state(&mut self, interrupts: Interrupts) {
        Instance::registers().ier.write(|w| {
            w.rdrf()
                .variant(interrupts.rx_data_register_full)
                .tdre()
                .variant(interrupts.tx_data_register_empty)
                .modf()
                .variant(interrupts.mode_fault_error)
                .ovres()
                .variant(interrupts.overrun_error)
                .nssr()
                .variant(interrupts.nss_rising)
                .txempty()
                .variant(interrupts.tx_registers_empty)
                .undes()
                .variant(interrupts.underrun_error)
        });

        Instance::registers().idr.write(|w| {
            w.rdrf()
                .variant(!interrupts.rx_data_register_full)
                .tdre()
                .variant(!interrupts.tx_data_register_empty)
                .modf()
                .variant(!interrupts.mode_fault_error)
                .ovres()
                .variant(!interrupts.overrun_error)
                .nssr()
                .variant(!interrupts.nss_rising)
                .txempty()
                .variant(!interrupts.tx_registers_empty)
                .undes()
                .variant(!interrupts.underrun_error)
        });
    }

    /// Returns current interrupts state (enabled/disabled).
    ///
    /// To check the state of events the IRQs indicate, use
    /// [`StatusReader`](super::status_reader::StatusReader).
    pub fn interrupts_state(&self) -> Interrupts {
        Instance::registers().imr.read().into()
    }

    /// Configures specified chip with provided configuration.
    /// Previous configuration is overwritten.
    /// **Chip must be configured before performing a transaction, trying to perform a transaction
    /// before chip configuration will result in runtime error.**
    pub fn configure_chip(&mut self, chip: SelectedChip, config: ChipSelectConfig) {
        let (csaat_bit, csnaat_bit) = match config.chip_select_behavior {
            ChipSelectBehavior::DeactivateAfterLastTransfer => (false, false),
            ChipSelectBehavior::KeepActive => (true, false),
            ChipSelectBehavior::DeactivateAfterEveryTransfer => (false, true),
        };

        Instance::registers().csr[chip as usize].write(|w| {
            w.cpol()
                .variant(config.clock_polarity.into())
                .ncpha()
                .variant(config.clock_phase.into())
                .csnaat()
                .variant(csnaat_bit)
                .csaat()
                .variant(csaat_bit)
                .bits_()
                .variant(config.bits_per_transfer.into())
                .scbr()
                .variant(config.clock_divider.get())
                .dlybs()
                .variant(config.delay_before_first_clock)
                .dlybct()
                .variant(config.delay_between_consecutive_transfers)
        });

        self.state.is_chip_select_configured[chip as usize] = true;
    }

    /// Returns `true` if specified chip is configured, `false` otherwise.
    pub fn is_chip_configured(&self, chip: SelectedChip) -> bool {
        self.state.is_chip_select_configured[chip as usize]
    }

    /// Returns specified chip's configuration, or None if the chip wasn't configured yet.
    pub fn chip_config(&self, chip: SelectedChip) -> Option<ChipSelectConfig> {
        if !self.is_chip_configured(chip) {
            return None;
        }

        let reg = Instance::registers().csr[chip as usize].read();

        let chip_select_behavior = match (reg.csaat().bit(), reg.csnaat().bit()) {
            (true, true) => ChipSelectBehavior::KeepActive,
            (true, false) => ChipSelectBehavior::KeepActive,
            (false, true) => ChipSelectBehavior::DeactivateAfterEveryTransfer,
            (false, false) => ChipSelectBehavior::DeactivateAfterLastTransfer,
        };

        Some(ChipSelectConfig {
            clock_polarity: reg.cpol().variant().into(),
            clock_phase: reg.ncpha().variant().into(),
            chip_select_behavior,
            bits_per_transfer: reg.bits_().variant().unwrap().into(),
            clock_divider: SerialClockDivider::new(reg.scbr().bits()).unwrap(),
            delay_before_first_clock: reg.dlybs().bits(),
            delay_between_consecutive_transfers: reg.dlybct().bits(),
        })
    }

    /// Selects specified chip. If it hasn't been configured, it must be before first transaction.
    /// Chip config is not cleared when chip is changed.
    pub fn change_chip(&mut self, new_chip: SelectedChip) {
        Instance::registers()
            .mr
            .modify(|_, w| w.pcs().variant(new_chip.into()));
    }

    /// Returns currently selected chip.
    pub fn selected_chip(&self) -> SelectedChip {
        Instance::registers()
            .mr
            .read()
            .pcs()
            .variant()
            .unwrap()
            .into()
    }

    /// Returns `true` if currently selected chip has been configured and is ready for transaction.
    pub fn is_current_chip_configured(&self) -> bool {
        self.state.is_chip_select_configured[self.selected_chip() as usize]
    }

    /// Enables loopback mode.
    pub fn enable_loopback(&mut self) {
        Instance::registers().mr.modify(|_, w| w.llb().set_bit());
    }

    /// Disables loopback mode.
    pub fn disable_loopback(&mut self) {
        Instance::registers().mr.modify(|_, w| w.llb().clear_bit());
    }

    /// Returns `true` if loopback mode is currently enabled, `false` otherwise.
    pub fn is_loopback_enabled(&self) -> bool {
        Instance::registers().mr.read().llb().bit_is_set()
    }

    /// Returns an address to SPI RX register for XDMAC.
    pub fn xdmac_rx_address(&mut self) -> *const () {
        Instance::registers().rdr.as_ptr() as *const ()
    }

    /// Returns an address to SPI TX register for XDMAC.
    pub fn xdmac_tx_address(&mut self) -> *const () {
        Instance::registers().tdr.as_ptr() as *const ()
    }

    /// Returns Reader instance, or None if Reader has already been taken.
    pub fn take_reader(&mut self) -> Option<Reader<Instance>> {
        self.reader.take()
    }

    /// Takes Reader and puts it back in SPI instance. After returning, it can be taken again with
    /// [`Spi::take_reader`].
    pub fn return_reader(&mut self, reader: Reader<Instance>) {
        self.reader.replace(reader);
    }

    /// Returns `true` if Reader's instance is currently stored in SPI instance.
    pub fn is_reader_available(&self) -> bool {
        self.reader.is_some()
    }

    /// Returns Writer instance, or None if Writer has already been taken.
    pub fn take_writer(&mut self) -> Option<Writer<Instance>> {
        self.writer.take()
    }

    /// Takes Writer and puts it back in SPI instance. After returning, it can be taken again with
    /// [`Spi::take_writer`].
    pub fn return_writer(&mut self, reader: Writer<Instance>) {
        self.writer.replace(reader);
    }

    /// Returns `true` if Writer's instance is currently stored in SPI instance.
    pub fn is_writer_available(&self) -> bool {
        self.writer.is_some()
    }
}
