//! Implementation of SPI driver functionality in Master mode.

use super::{
    chip_select_config::{ChipSelectBehavior, ChipSelectConfig, SerialClockDivider},
    config::SelectedChip,
    interrupts::Interrupts,
    metadata::SPIMetadata,
    Master, Spi,
};

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
}
