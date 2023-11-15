//! Implementation of HAL SPI driver.
//!
//! The Serial Peripheral Interface (SPI) circuit is a synchronous serial data link that provides
//! communication with external devices in Host or Client mode. It also enables communication
//! between processors if an external processor is connected to the system.
//!
//! The Serial Peripheral Interface is essentially a shift register that serially transmits data
//! bits to other SPIs. During a data transfer, one SPI system acts as the "Host" which controls
//! the data flow, while the other devices act as "Clients" which have data shifted into and out by
//! the Host. Different CPUs can take turn being Hosts (multiple Host protocol, contrary to single
//! Host protocol where one CPU is always the Host while all of the others are always Clients). One
//! Host can simultaneously shift data into multiple Clients. However, only one Client can drive its
//! output to write data back to the Host at any given time.
//!
//! This driver currently supports:
//! * SPI Master mode
//! * Configuration of
//!     * Word size (8 to 16 bits)
//!     * Clock polarity
//!     * Clock phase
//! * Hardware management of fixed Chip Select signal
//! * Loopback mode
//! * Interrupt configuration and status management
//! * DMA transfers
//!
//! Specifically, it currently does **NOT** support:
//! * SPI Client mode
//! * Variable Chip Select signal management
//! * Mode Fault Detection
//! * Register Write Protection

use core::marker::PhantomData;

use self::{
    config::MasterConfig, metadata::SPIMetadata, reader::Reader, status_reader::StatusReader,
    writer::Writer,
};

pub mod chip_config;
pub mod config;
pub mod interrupts;
pub mod master;
pub mod metadata;
pub mod reader;
pub mod status;
pub mod status_reader;
pub mod writer;

/// Amount of supported chip select signals and configurations.
pub const SUPPORTED_CHIP_AMOUNT: usize = 4;

/// Typestate trait representing generic SPI state.
///
/// This is a super-trait for all SPI states.
pub trait State {}

/// Typestate struct representing SPI in not configured, post-reset state.
pub struct NotConfigured;

/// Typestate struct representing SPI in configured Master state.
pub struct Master {
    /// If this flag is set, then the chip with ID equal to the index is configured and can be used
    /// for transactions. Trying to start a transaction with unconfigured chip should always result
    /// in a runtime error.
    is_chip_select_configured: [bool; SUPPORTED_CHIP_AMOUNT],
}

impl State for NotConfigured {}
impl State for Master {}

/// Structure representing SPI peripheral.
///
/// # Generic parameters
/// * `Instance` - PAC SPI instance.
pub struct Spi<Instance: SPIMetadata, CurrentState: State> {
    /// Status reader instance storage.
    status_reader: Option<StatusReader<Instance>>,
    /// PAC SPI instance metadata.
    _meta: PhantomData<Instance>,
    /// State metadata.
    state: CurrentState,
    /// Reader instance
    reader: Option<Reader<Instance>>,
    /// Writer instance
    writer: Option<Writer<Instance>>,
}

impl<Instance: SPIMetadata> Spi<Instance, NotConfigured> {
    /// Creates new SPI driver instance, consuming PAC SPI instance to prevent creation of multiple
    /// drivers for the same SPI instance.
    ///
    /// # Parameters
    /// * `spi` - PAC SPIx instance, where `x` is the number of SPI peripheral.
    ///
    /// # Returns
    /// SPI driver instance in `NotConfigured` state. It must be converted into usable state using
    /// `into_X` method, for example [`into_master`](Spi::into_master).
    pub fn new(_spi: Instance) -> Self {
        Self {
            status_reader: Some(StatusReader::new()),
            _meta: PhantomData,
            state: NotConfigured,
            reader: Some(Reader::new()),
            writer: Some(Writer::new()),
        }
        .reset()
    }

    /// Configures and enables SPI in Master mode with provided configuration.
    /// Chip-specific settings must be configured with [`Spi::configure_chip`] before starting
    /// transaction. Trying to perform transaction with unconfigured chip will result in runtime
    /// error.
    pub fn into_master(mut self, config: MasterConfig) -> Spi<Instance, Master> {
        Instance::registers().mr.write(|w| {
            w.mstr()
                .master() // Master mode
                .ps()
                .clear_bit() // Fixed peripheral selection
                .pcsdec()
                .clear_bit() // Chip-select signal is connected directly to periph.
                .modfdis()
                .set_bit() // Mode fault detection disabled
                .wdrbt()
                .variant(config.enable_overrun_detection)
                .llb()
                .clear_bit() // Loopback disabled
                .pcs()
                .variant(config.selected_chip.into())
                .dlybcs()
                .variant(config.chip_selection_delay.get())
        });
        self.enable_hardware();

        Spi::transform(
            self,
            Master {
                is_chip_select_configured: [false, false, false, false],
            },
        )
    }
}

impl<Instance: SPIMetadata, CurrentState: State> Spi<Instance, CurrentState> {
    /// Disables SPI and restores the configuration to defaults.
    pub fn reset(mut self) -> Spi<Instance, NotConfigured> {
        self.disable_hardware();
        self.disable_all_irqs();
        self.reset_hardware();
        Spi::transform(self, NotConfigured)
    }

    /// Returns status reader (or None if it's already been taken)
    pub fn take_status_reader(&mut self) -> Option<StatusReader<Instance>> {
        self.status_reader.take()
    }

    /// Puts status reader back into SPI driver instance.
    pub fn return_status_reader(&mut self, status_reader: StatusReader<Instance>) {
        self.status_reader.replace(status_reader);
    }

    /// Returns `true` if status reader is currently stored inside SPI instance.
    pub fn is_status_reader_available(&self) -> bool {
        self.status_reader.is_some()
    }

    /// Triggers a hardware reset of the SPI interface.
    fn reset_hardware(&mut self) {
        Instance::registers().cr.write(|w| w.swrst().set_bit());
    }

    /// Enables SPI.
    fn enable_hardware(&mut self) {
        Instance::registers().cr.write(|w| w.spien().set_bit());
    }

    /// Disables SPI.
    fn disable_hardware(&mut self) {
        Instance::registers().cr.write(|w| w.spidis().set_bit());
    }

    /// Disables all SPI interrupts.
    fn disable_all_irqs(&mut self) {
        Instance::registers().idr.write(|w| {
            w.rdrf()
                .set_bit()
                .tdre()
                .set_bit()
                .modf()
                .set_bit()
                .ovres()
                .set_bit()
                .nssr()
                .set_bit()
                .txempty()
                .set_bit()
                .undes()
                .set_bit()
        })
    }

    /// Transforms SPI into a different state. All state-related fields are reset to default in
    /// this process.
    fn transform<OldState: State>(spi: Spi<Instance, OldState>, new_state: CurrentState) -> Self {
        Self {
            status_reader: spi.status_reader,
            _meta: PhantomData,
            state: new_state,
            reader: spi.reader,
            writer: spi.writer,
        }
    }
}
