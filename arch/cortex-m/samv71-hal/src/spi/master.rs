//! Implementation of SPI driver functionality in Master mode.

use super::{metadata::SPIMetadata, Master, Spi};

impl<Instance: SPIMetadata> Spi<Instance, Master> {}
