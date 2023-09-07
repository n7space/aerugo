//! This module contains structures related to Master Clock (MCK) configuration.

use crate::pac::pmc::mckr::{CSSSELECT_A, MDIVSELECT_A, PRESSELECT_A};

/// Structure representing master clock (MCK) configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MasterClockConfig {
    /// Master clock source.
    pub source: MasterClockSource,
    /// Master clock prescaler.
    pub prescaler: ProcessorClockPrescaler,
    /// Master clock divider.
    pub divider: MasterClockDivider,
}

/// Enumeration representing master clock (MCK) source.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MasterClockSource {
    /// Slow clock (SLCK).
    SlowClock,
    /// Main clock (MAINCK).
    MainClock,
    /// PLL A clock.
    PLLA,
    /// USB UTMI PLL clock.
    USBPLL,
}

impl From<CSSSELECT_A> for MasterClockSource {
    fn from(value: CSSSELECT_A) -> Self {
        match value {
            CSSSELECT_A::SLOW_CLK => MasterClockSource::SlowClock,
            CSSSELECT_A::MAIN_CLK => MasterClockSource::MainClock,
            CSSSELECT_A::PLLA_CLK => MasterClockSource::PLLA,
            CSSSELECT_A::UPLL_CLK => MasterClockSource::USBPLL,
        }
    }
}

impl From<MasterClockSource> for CSSSELECT_A {
    fn from(value: MasterClockSource) -> Self {
        match value {
            MasterClockSource::SlowClock => CSSSELECT_A::SLOW_CLK,
            MasterClockSource::MainClock => CSSSELECT_A::MAIN_CLK,
            MasterClockSource::PLLA => CSSSELECT_A::PLLA_CLK,
            MasterClockSource::USBPLL => CSSSELECT_A::UPLL_CLK,
        }
    }
}

/// Enumeration representing master clock (MCK) prescaler.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProcessorClockPrescaler {
    /// Don't divide processor clock
    NoDivision = 0,
    /// Divide processor clock by 2.
    DivBy2 = 1,
    /// Divide processor clock by 3.
    DivBy3 = 7,
    /// Divide processor clock by 4.
    DivBy4 = 2,
    /// Divide processor clock by 8.
    DivBy8 = 3,
    /// Divide processor clock by 16.
    DivBy16 = 4,
    /// Divide processor clock by 32.
    DivBy32 = 5,
    /// Divide processor clock by 64.
    DivBy64 = 6,
}

impl From<PRESSELECT_A> for ProcessorClockPrescaler {
    fn from(value: PRESSELECT_A) -> Self {
        match value {
            PRESSELECT_A::CLK_1 => ProcessorClockPrescaler::NoDivision,
            PRESSELECT_A::CLK_2 => ProcessorClockPrescaler::DivBy2,
            PRESSELECT_A::CLK_4 => ProcessorClockPrescaler::DivBy4,
            PRESSELECT_A::CLK_8 => ProcessorClockPrescaler::DivBy8,
            PRESSELECT_A::CLK_16 => ProcessorClockPrescaler::DivBy16,
            PRESSELECT_A::CLK_32 => ProcessorClockPrescaler::DivBy32,
            PRESSELECT_A::CLK_64 => ProcessorClockPrescaler::DivBy64,
            PRESSELECT_A::CLK_3 => ProcessorClockPrescaler::DivBy3,
        }
    }
}

impl From<ProcessorClockPrescaler> for PRESSELECT_A {
    fn from(value: ProcessorClockPrescaler) -> Self {
        match value {
            ProcessorClockPrescaler::NoDivision => PRESSELECT_A::CLK_1,
            ProcessorClockPrescaler::DivBy2 => PRESSELECT_A::CLK_2,
            ProcessorClockPrescaler::DivBy3 => PRESSELECT_A::CLK_3,
            ProcessorClockPrescaler::DivBy4 => PRESSELECT_A::CLK_4,
            ProcessorClockPrescaler::DivBy8 => PRESSELECT_A::CLK_8,
            ProcessorClockPrescaler::DivBy16 => PRESSELECT_A::CLK_16,
            ProcessorClockPrescaler::DivBy32 => PRESSELECT_A::CLK_1,
            ProcessorClockPrescaler::DivBy64 => PRESSELECT_A::CLK_64,
        }
    }
}

/// Enumeration representing master clock (MCK) divider.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MasterClockDivider {
    /// Don't divide master clock.
    NoDivision = 0,
    /// Divide master clock by 2.
    DivBy2 = 1,
    /// Divide master clock by 3.
    DivBy3 = 3,
    /// Divide master clock by 4.
    DivBy4 = 2,
}

impl From<MDIVSELECT_A> for MasterClockDivider {
    fn from(value: MDIVSELECT_A) -> Self {
        match value {
            MDIVSELECT_A::EQ_PCK => MasterClockDivider::NoDivision,
            MDIVSELECT_A::PCK_DIV2 => MasterClockDivider::DivBy2,
            MDIVSELECT_A::PCK_DIV4 => MasterClockDivider::DivBy4,
            MDIVSELECT_A::PCK_DIV3 => MasterClockDivider::DivBy3,
        }
    }
}

impl From<MasterClockDivider> for MDIVSELECT_A {
    fn from(value: MasterClockDivider) -> Self {
        match value {
            MasterClockDivider::NoDivision => MDIVSELECT_A::EQ_PCK,
            MasterClockDivider::DivBy2 => MDIVSELECT_A::PCK_DIV2,
            MasterClockDivider::DivBy3 => MDIVSELECT_A::PCK_DIV3,
            MasterClockDivider::DivBy4 => MDIVSELECT_A::PCK_DIV4,
        }
    }
}
