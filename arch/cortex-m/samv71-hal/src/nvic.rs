//! Module containing HAL NVIC driver.
//!
//! This is a wrapper for Cortex-M/PAC NVIC type, because that type has most of it's
//! functionality implemented via trait functions for NVIC structure without `self`
//! parameter (which makes it kind-of static struct/singleton).
//!
//! In SAMV71-HAL every peripheral is a single instance, which allows easy ownership
//! management, therefore Cortex-M/PAC NVIC is wrapped in such type.

use samv71q21_pac as pac;

/// Structure representing NVIC.
pub struct NVIC {
    /// PAC NVIC instance
    nvic: pac::NVIC,
}

impl NVIC {
    /// Creates an instance of NVIC driver.
    ///
    /// # Parameters
    /// * `nvic` - PAC/Cortex-M instance of NVIC.
    pub fn new(nvic: pac::NVIC) -> Self {
        Self { nvic }
    }

    /// Disables provided interrupt.
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to disable.
    #[inline]
    pub fn disable(&mut self, interrupt: Interrupt) {
        pac::NVIC::mask(interrupt)
    }

    /// Enables provided interrupt.
    ///
    /// # Safety
    /// This function is unsafe in mask-based critical sections, as unmasking
    /// an interrupt in those sections not only will forcefully enable the
    /// requested interrupt, despite being in critical section, that IRQ will
    /// also be disabled at the end of critical section due to the fact that
    /// pre-section masks will be restored.
    ///
    /// Cortex-M's `critical-section-single-core` implementation uses PRIMASK
    /// for IRQ management, it does not manipulate NVIC priority mask, so this
    /// warning doesn't apply there, therefore this function is safe to use in
    /// critical sections as long as this specific implementation is used.
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to enable.
    #[inline]
    pub fn enable(&mut self, interrupt: Interrupt) {
        unsafe { pac::NVIC::unmask(interrupt) }
    }

    /// Trigger an interrupt via software.
    ///
    /// This function sets the provided IRQ to "pending" state, triggering it's
    /// handler as soon as it's unmasked (or immediately, if it's unmasked at the
    /// time of request). Does more-or-less the same thing as [`NVIC::pend`]
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to trigger.
    #[inline]
    pub fn trigger(&mut self, interrupt: Interrupt) {
        self.nvic.request(interrupt)
    }

    /// Returns the priority of provided interrupt.
    ///
    /// Lower value means higher priority.
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to query.
    #[inline]
    pub fn get_priority(&self, interrupt: Interrupt) -> InterruptPriority {
        InterruptPriority::from_register_value(pac::NVIC::get_priority(interrupt))
    }

    /// Sets priority of provided interrupt.
    ///
    /// Lower value means higher priority.
    ///
    /// # Safety
    /// This function is unsafe in priority-based critical sections, as changing
    /// priorities may break them similarly to how `enable` may break mask-based
    /// critical sections.
    ///
    /// Cortex-M's `critical-section-single-core` implementation uses PRIMASK,
    /// not BASEMASK, and disables all IRQs unconditionally, therefore it's not
    /// unsafe to use this function inside critical sections as long as this
    /// implementation is used.
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to change priority
    /// * `priority` - New priority
    #[inline]
    pub fn set_priority(&mut self, interrupt: Interrupt, priority: InterruptPriority) {
        unsafe {
            self.nvic
                .set_priority(interrupt, priority.into_register_value())
        }
    }

    /// Returns `true` if interrupt is currently being handled
    /// (is active or pre-empted and stacked).
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to query.
    #[inline]
    pub fn is_active(&self, interrupt: Interrupt) -> bool {
        pac::NVIC::is_active(interrupt)
    }

    /// Returns `true` if interrupt is currently NOT being handled
    /// (is active or pre-empted and stacked).
    ///
    /// Complement of `is_active`.
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to query.
    #[inline]
    pub fn is_inactive(&self, interrupt: Interrupt) -> bool {
        !self.is_active(interrupt)
    }

    /// Returns `true` if interrupt is currently enabled.
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to query.
    #[inline]
    pub fn is_enabled(&self, interrupt: Interrupt) -> bool {
        pac::NVIC::is_enabled(interrupt)
    }

    /// Returns `true` if interrupt is currently disabled.
    ///
    /// Complement of `is_enabled`.
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to query.
    #[inline]
    pub fn is_disabled(&self, interrupt: Interrupt) -> bool {
        !self.is_enabled(interrupt)
    }

    /// Returns `true` if interrupt is currently pending.
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to query.
    #[inline]
    pub fn is_pending(&self, interrupt: Interrupt) -> bool {
        pac::NVIC::is_pending(interrupt)
    }

    /// Returns `true` if interrupt is currently not pending.
    ///
    /// Complement of `is_pending`.
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to query.
    #[inline]
    pub fn is_not_pending(&self, interrupt: Interrupt) -> bool {
        pac::NVIC::is_pending(interrupt)
    }

    /// Force interrupt into pending state.
    ///
    /// Does more-or-less the same thing as [`NVIC::trigger`].
    ///
    /// # Parameters
    /// * `interrupt` - Interrupt to force into pending state.
    #[inline]
    pub fn pend(&mut self, interrupt: Interrupt) {
        pac::NVIC::pend(interrupt)
    }

    /// Clears interrupt's pending state.
    ///
    /// Can be used to prevent IRQ from executing after quitting from
    /// critical section, if it was triggered during it.
    #[inline]
    pub fn unpend(&mut self, interrupt: Interrupt) {
        pac::NVIC::unpend(interrupt)
    }
}

/// Structure representing interrupt priority.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InterruptPriority(u8);

impl InterruptPriority {
    /// Minimum priority for SAMV71
    const MINIMUM_PRIORITY: u8 = 7;
    /// Offset of the priority in NVIC registers.
    const PRIORITY_OFFSET: u8 = 5;

    /// Creates a new instance of interrupt priority.
    /// **Lower priority value means higher priority**.
    /// **IRQ with priority = 3 has higher priority than IRQ with priority = 5**.
    ///
    /// Valid priorities are in (0..=7) range. If priority outside of this
    /// range is provided, this function will return `None`.
    ///
    /// # Parameters
    /// * `priority` - Priority value.
    ///
    /// # Returns
    /// `Some(InterruptPriority)` if valid priority was provided.
    /// `None` otherwise.
    pub const fn new(priority: u8) -> Option<Self> {
        if priority > Self::MINIMUM_PRIORITY {
            return None;
        }

        Some(Self(priority))
    }

    /// Returns numerical value of the priority.
    #[inline]
    pub const fn value(&self) -> u8 {
        self.0
    }

    /// Transforms InterruptPriority into a value that can be written directly
    /// into NVIC IRQ priority register.
    #[inline]
    pub(super) const fn into_register_value(self) -> u8 {
        self.value() << Self::PRIORITY_OFFSET
    }

    /// Transforms a priority from register into an InterruptPriority object.
    ///
    /// # Safety
    /// This function will panic if, for some reason, an invalid priority
    /// is read from NVIC registers. If that ever happens, you should contact
    /// the developers of this module.
    #[inline]
    pub(super) fn from_register_value(value: u8) -> Self {
        Self::new(value >> Self::PRIORITY_OFFSET).unwrap()
    }
}

impl TryFrom<u8> for InterruptPriority {
    type Error = ();

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        InterruptPriority::new(value).ok_or(())
    }
}

impl From<InterruptPriority> for u8 {
    #[inline]
    fn from(value: InterruptPriority) -> Self {
        value.value()
    }
}

/// Enumeration representing SAMV71 interrupts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// IRQ 0 - Supply Controller
    SUPC = 0,
    /// IRQ 1 - Reset Controller
    RSTC = 1,
    /// IRQ 2 - Real Time Clock
    RTC = 2,
    /// IRQ 3 - Real Time Timer
    RTT = 3,
    /// IRQ 4 - Watchdog Timer
    WDT = 4,
    /// IRQ 5 - Power Management Controller
    PMC = 5,
    /// IRQ 6 - Enhanced Embedded Flash Controller
    EFC = 6,
    /// IRQ 7 - UART 0
    UART0 = 7,
    /// IRQ 8 - UART 1
    UART1 = 8,
    /// IRQ 10 - Parallel I/O Controller A
    PIOA = 10,
    /// IRQ 11 - Parallel I/O Controller B
    PIOB = 11,
    /// IRQ 12 - Parallel I/O Controller C
    PIOC = 12,
    /// IRQ 13 - USART 0
    USART0 = 13,
    /// IRQ 14 - USART 1
    USART1 = 14,
    /// IRQ 15 - USART 2
    USART2 = 15,
    /// IRQ 16 - Parallel I/O Controller D
    PIOD = 16,
    /// IRQ 17 - Parallel I/O Controller E
    PIOE = 17,
    /// IRQ 18 - Multimedia Card Interface
    HSMCI = 18,
    /// IRQ 19 - Two Wire Interface 0 HS
    TWIHS0 = 19,
    /// IRQ 20 - Two Wire Interface 1 HS
    TWIHS1 = 20,
    /// IRQ 21 - Serial Peripheral Interface 0
    SPI0 = 21,
    /// IRQ 22 - Synchronous Serial Controller
    SSC = 22,
    /// IRQ 23 - Timer/Counter 0, Channel 0 (TC0)
    TC0CH0 = 23,
    /// IRQ 24 - Timer/Counter 0, Channel 1 (TC1)
    TC0CH1 = 24,
    /// IRQ 25 - Timer/Counter 0, Channel 2 (TC2)
    TC0CH2 = 25,
    /// IRQ 26 - Timer/Counter 1, Channel 0 (TC3)
    TC1CH0 = 26,
    /// IRQ 27 - Timer/Counter 1, Channel 1 (TC4)
    TC1CH1 = 27,
    /// IRQ 28 - Timer/Counter 1, Channel 2 (TC5)
    TC1CH2 = 28,
    /// IRQ 29 - Analog Front End 0
    AFEC0 = 29,
    /// IRQ 30 - Digital To Analog Converter
    DACC = 30,
    /// IRQ 31 - Pulse Width Modulation 0
    PWM0 = 31,
    /// IRQ 32 - Integrity Check Monitor
    ICM = 32,
    /// IRQ 33 - Analog Comparator
    ACC = 33,
    /// IRQ 34 - USB Host / Device Controller
    USBHS = 34,
    /// IRQ 35 - MCAN Controller 0 Interrupt 0
    MCAN0_INT0 = 35,
    /// IRQ 36 - MCAN Controller 0 Interrupt 1
    MCAN0_INT1 = 36,
    /// IRQ 37 - MCAN Controller 1 Interrupt 0
    MCAN1_INT0 = 37,
    /// IRQ 38 - MCAN Controller 1 Interrupt 1
    MCAN1_INT1 = 38,
    /// IRQ 39 - Ethernet MAC
    GMAC = 39,
    /// IRQ 40 - Analog Front End 1
    AFEC1 = 40,
    /// IRQ 41 - Two Wire Interface 2 HS
    TWIHS2 = 41,
    /// IRQ 42 - Serial Peripheral Interface 1
    SPI1 = 42,
    /// IRQ 43 - Quad I/O Serial Peripheral Interface
    QSPI = 43,
    /// IRQ 44 - UART 2
    UART2 = 44,
    /// IRQ 45 - UART 3
    UART3 = 45,
    /// IRQ 46 - UART 4
    UART4 = 46,
    /// IRQ 47 - Timer/Counter 2, Channel 0 (TC6)
    TC2CH0 = 47,
    /// IRQ 48 - Timer/Counter 2, Channel 1 (TC7)
    TC2CH1 = 48,
    /// IRQ 49 - Timer/Counter 2, Channel 2 (TC8)
    TC2CH2 = 49,
    /// IRQ 50 - Timer/Counter 3, Channel 0 (TC9)
    TC3CH0 = 50,
    /// IRQ 51 - Timer/Counter 3, Channel 1 (TC10)
    TC3CH1 = 51,
    /// IRQ 52 - Timer/Counter 3, Channel 2 (TC11)
    TC3CH2 = 52,
    /// IRQ 53 - MediaLB IRQ 0
    MLB0 = 53,
    /// IRQ 54 - MediaLB IRQ 1
    MLB1 = 54,
    /// IRQ 56 - AES
    AES = 56,
    /// IRQ 57 - True Random Generator
    TRNG = 57,
    /// IRQ 58 - DMA
    XDMAC = 58,
    /// IRQ 59 - Camera Interface
    ISI = 59,
    /// IRQ 60 - Pulse Width Modulation 1
    PWM1 = 60,
    /// IRQ 61 - Floating Point Unit
    FPU = 61,
    /// IRQ 63 - Reinforced Secure Watchdog Timer
    RSWDT = 63,
    /// IRQ 64 - Cache ECC Warning
    CCW = 64,
    /// IRQ 65 - Cache ECC Fault
    CCF = 65,
    /// IRQ 66 - GMAC Queue 1
    GMAC_Q1 = 66,
    /// IRQ 67 - GMAC Queue 2
    GMAC_Q2 = 67,
    /// IRQ 68 - Floating Point Unit IXC
    IXC = 68,
    /// IRQ 69 - Inter-IC Sound controller 0
    I2SC0 = 69,
    /// IRQ 70 - Inter-IC Sound controller 1
    I2SC1 = 70,
    /// IRQ 71 - GMAC Queue 3
    GMAC_Q3 = 71,
    /// IRQ 72 - GMAC Queue 4
    GMAC_Q4 = 72,
    /// IRQ 73 - GMAC Queue 5
    GMAC_Q5 = 73,
}

/// Implementation of InterruptNumber for HAL's Interrupt type.
///
/// # Safety
/// IRQ IDs are taken from PAC crate, and consulted with datasheet.
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline]
    fn number(self) -> u16 {
        self as u16
    }
}
