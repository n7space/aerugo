//! This module contains structures related to peripheral clocks configuration.

use crate::pac::pmc::pcr::GCLKCSSSELECT_A;

/// Enumeration representing list of all peripherals that use peripheral clocks managed by Clock Controller.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PeripheralId {
    /// UART 0
    UART0 = 7,
    /// UART 1
    UART1 = 8,
    /// Parallel I/O Controller A
    PIOA = 10,
    /// Parallel I/O Controller B
    PIOB = 11,
    /// Parallel I/O Controller C
    PIOC = 12,
    /// USART 0
    USART0 = 13,
    /// USART 1
    USART1 = 14,
    /// USART 2
    USART2 = 15,
    /// Parallel I/O Controller D
    PIOD = 16,
    /// Parallel I/O Controller E
    PIOE = 17,
    /// Multimedia Card Interface
    HSMCI = 18,
    /// Two Wire Interface 0 HS
    TWIHS0 = 19,
    /// Two Wire Interface 1 HS
    TWIHS1 = 20,
    /// Serial Peripheral Interface 0
    SPI0 = 21,
    /// Synchronous Serial Controller
    SSC = 22,
    /// Timer/Counter 0, Channel 0
    TC0CH0 = 23,
    /// Timer/Counter 0, Channel 1
    TC0CH1 = 24,
    /// Timer/Counter 0, Channel 2
    TC0CH2 = 25,
    /// Timer/Counter 1, Channel 0
    TC1CH0 = 26,
    /// Timer/Counter 1, Channel 1
    TC1CH1 = 27,
    /// Timer/Counter 1, Channel 2
    TC1CH2 = 28,
    /// Analog Front End 0
    AFEC0 = 29,
    /// Digital To Analog Converter
    DACC = 30,
    /// Pulse Width Modulation 0
    PWM0 = 31,
    /// Integrity Check Monitor
    ICM = 32,
    /// Analog Comparator
    ACC = 33,
    /// USB Host / Device Controller
    USBHS = 34,
    /// MCAN Controller 0
    MCAN0 = 35,
    /// MCAN Controller 1
    MCAN1 = 37,
    /// Ethernet MAC
    GMAC = 39,
    /// Analog Front End 1
    AFEC1 = 40,
    /// Two Wire Interface 2 HS
    TWIHS2 = 41,
    /// Serial Peripheral Interface 1
    SPI1 = 42,
    /// Quad I/O Serial Peripheral Interface
    QSPI = 43,
    /// UART 2
    UART2 = 44,
    /// UART 3
    UART3 = 45,
    /// UART 4
    UART4 = 46,
    /// Timer/Counter 2, Channel 0
    TC2CH0 = 47,
    /// Timer/Counter 2, Channel 1
    TC2CH1 = 48,
    /// Timer/Counter 2, Channel 2
    TC2CH2 = 49,
    /// Timer/Counter 3, Channel 0
    TC3CH0 = 50,
    /// Timer/Counter 3, Channel 1
    TC3CH1 = 51,
    /// Timer/Counter 3, Channel 2
    TC3CH2 = 52,
    /// MediaLB
    MLB = 53,
    /// AES
    AES = 56,
    /// True Random Generator
    TRNG = 57,
    /// DMA
    XDMAC = 58,
    /// Camera Interface
    ISI = 59,
    /// Pulse Width Modulation 1
    PWM1 = 60,
    /// Inter-IC Sound controller 0
    I2SC0 = 69,
    /// Inter-IC Sound controller 1
    I2SC1 = 70,
}

/// Structure representing peripheral clock configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PeripheralClockConfig {
    /// True, if peripheral clock is/should be enabled.
    pub enabled: bool,
    /// Generic clock configuration.
    pub generic_clock: GenericClockConfig,
}

impl Default for PeripheralClockConfig {
    /// Returns default, power-up configuration of peripheral clocks.
    fn default() -> Self {
        Self {
            enabled: false,
            generic_clock: Default::default(),
        }
    }
}

/// Structure representing generic clock configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GenericClockConfig {
    /// True, if generic clock is/should be enabled.
    pub enabled: bool,
    /// Generic clock source
    pub source: GenericClockSource,
    /// Generic clock divider.
    pub divider: GenericClockDivider,
}

impl Default for GenericClockConfig {
    /// Returns default, power-up configuration of generic clocks for peripherals.
    fn default() -> Self {
        Self {
            enabled: false,
            source: GenericClockSource::SlowClock,
            divider: GenericClockDivider::from_register_value(0),
        }
    }
}

/// Enumeration representing available generic clock sources.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GenericClockSource {
    /// Slow clock (SLCK).
    SlowClock,
    /// Main clock (MAINCK).
    MainClock,
    /// PLL A clock.
    PLLA,
    /// USB UTMI PLL clock.
    USBPLL,
    /// Master clock (MCK).
    MasterClock,
}

impl From<GCLKCSSSELECT_A> for GenericClockSource {
    fn from(value: GCLKCSSSELECT_A) -> Self {
        match value {
            GCLKCSSSELECT_A::SLOW_CLK => GenericClockSource::SlowClock,
            GCLKCSSSELECT_A::MAIN_CLK => GenericClockSource::MainClock,
            GCLKCSSSELECT_A::PLLA_CLK => GenericClockSource::PLLA,
            GCLKCSSSELECT_A::UPLL_CLK => GenericClockSource::USBPLL,
            GCLKCSSSELECT_A::MCK_CLK => GenericClockSource::MasterClock,
        }
    }
}

impl From<GenericClockSource> for GCLKCSSSELECT_A {
    fn from(value: GenericClockSource) -> Self {
        match value {
            GenericClockSource::SlowClock => GCLKCSSSELECT_A::SLOW_CLK,
            GenericClockSource::MainClock => GCLKCSSSELECT_A::MAIN_CLK,
            GenericClockSource::PLLA => GCLKCSSSELECT_A::PLLA_CLK,
            GenericClockSource::USBPLL => GCLKCSSSELECT_A::UPLL_CLK,
            GenericClockSource::MasterClock => GCLKCSSSELECT_A::MCK_CLK,
        }
    }
}

/// Type representing generic clock divider.
/// Since this prescaler works exactly the same as PCK prescaler,
/// it's going to be re-used.
///
/// For more info, go to [`PCKPrescaler`](super::pck::PCKPrescaler) documentation.
pub type GenericClockDivider = super::pck::PCKPrescaler;
