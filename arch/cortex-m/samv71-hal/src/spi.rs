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

use self::metadata::SPIMetadata;

pub mod master;
pub mod metadata;
pub mod status;
pub mod status_reader;

/// Typestate trait representing generic SPI state.
///
/// This is a super-trait for all SPI states.
pub trait State {}

/// Typestate struct representing SPI in not configured, post-reset state.
pub struct NotConfigured;

/// Typestate struct representing SPI in configured Master state.
pub struct Master;

impl State for NotConfigured {}
impl State for Master {}

/// Structure representing SPI peripheral.
///
/// # Generic parameters
/// * `Instance` - PAC SPI instance.
pub struct Spi<Instance: SPIMetadata, CurrentState: State> {
    /// PAC SPI instance metadata.
    _meta: PhantomData<Instance>,
    /// State metadata.
    _state: PhantomData<CurrentState>,
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
            _meta: PhantomData,
            _state: PhantomData,
        }
    }
}

impl<Instance: SPIMetadata, AnyState: State> Spi<Instance, AnyState> {}
