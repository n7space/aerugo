//! This module contains structures related to peripheral clocks configuration.

/// Enumeration representing list of all peripherals that use peripheral clocks managed by Clock Controller.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Peripheral {
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
    /// Timer/Counter 0
    TC0 = 23,
    /// Timer/Counter 1
    TC1 = 24,
    /// Timer/Counter 2
    TC2 = 25,
    /// Timer/Counter 3
    TC3 = 26,
    /// Timer/Counter 4
    TC4 = 27,
    /// Timer/Counter 5
    TC5 = 28,
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
    /// Timer/Counter 6
    TC6 = 47,
    /// Timer/Counter 7
    TC7 = 48,
    /// Timer/Counter 8
    TC8 = 49,
    /// Timer/Counter 9
    TC9 = 50,
    /// Timer/Counter 10
    TC10 = 51,
    /// Timer/Counter 11
    TC11 = 52,
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
