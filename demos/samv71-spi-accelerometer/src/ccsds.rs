//! Module with items related to CCSDS packets.

use crate::bounded_int::BoundedU16;

/// CCSDS packet's primary header structure.
pub struct CCSDSPrimaryHeader {
    /// Packet version number
    pub version_number: PacketVersionNumber,
    /// Packet identification
    pub identity: Identity,
    /// Packet sequence control
    pub sequence_control: SequenceControl,
    /// Packet data length
    pub data_length: u16,
}

/// A segment of CCSDS packet's primary header that identifies the packet.
pub struct Identity {
    /// Packet type
    pub packet_type: PacketType,
    /// Secondary header flag
    pub secondary_header_presence: SecondaryHeaderPresence,
    /// Application process identified
    pub apid: ApplicationProcessIdentifier,
}

/// A segment of CCSDS packet's primary header that provides information about packet's ID
pub struct SequenceControl {
    /// Sequence flags
    pub flags: SequenceFlags,
    /// Packet sequence count or packet name
    pub name: PacketName,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketVersionNumber {
    CCSDSVersion1 = 0b000,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketType {
    Telemetry = 0,
    Telecommand = 1,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum SecondaryHeaderPresence {
    NotPresent = 0,
    Present = 1,
}

pub type ApplicationProcessIdentifier = BoundedU16<0, 0x7FF>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum SequenceFlags {
    UserDataContinuation = 0b00,
    UserDataFirstSegment = 0b01,
    UserDataLastSegment = 0b10,
    UnsegmentedUserData = 0b11,
}

pub type PacketName = BoundedU16<0, 0x3FFF>;

pub type PacketPrimaryHeaderBuffer = [u8; 6];

impl From<PacketPrimaryHeaderBuffer> for CCSDSPrimaryHeader {
    fn from(value: PacketPrimaryHeaderBuffer) -> Self {
        todo!()
    }
}

impl From<CCSDSPrimaryHeader> for PacketPrimaryHeaderBuffer {
    fn from(value: CCSDSPrimaryHeader) -> Self {
        todo!()
    }
}
