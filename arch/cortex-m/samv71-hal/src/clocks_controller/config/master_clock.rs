//! This module contains structures related to Master Clock (MCK) configuration.

/// Structure representing master clock (MCK) configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MasterClockConfig {
    /// Master clock source.
    pub source: MasterClockSource,
    /// Master clock prescaler.
    pub prescaler: MasterClockPrescaler,
    /// Master clock divider.
    pub divider: MasterClockDivider,
}

/// Enumeration representing master clock (MCK) source.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MasterClockSource {}

/// Enumeration representing master clock (MCK) prescaler.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MasterClockPrescaler {}

/// Enumeration representing master clock (MCK) divider.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MasterClockDivider {}
