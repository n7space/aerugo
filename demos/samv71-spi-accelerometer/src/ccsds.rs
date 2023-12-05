//! Module with items related to CCSDS packets. Contains definition of CCSDS primary header, and
//! functions for conversion from/to byte buffer.
//!
//! This implementation assumes that bit 0 of the header is the MSB of the first received byte.
//!
//! Header's structure is implemented as defined in CCSDS 133.0-B-2 Recommended Standard, section 4,
//! which is available here: https://public.ccsds.org/Pubs/133x0b2e1.pdf

use bitfield_enum::{
    BitFieldFromByte, BitFieldToByte, BitFieldTryFromByte, MultiByteBitField,
    MultiByteBitFieldFromBytes, MultiByteBitFieldToBytes,
};

use crate::bitfield_enum::bitfield_enum;
use crate::bounded_int::BoundedU16;

/// CCSDS packet's primary header structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Identity {
    /// Packet type
    pub packet_type: PacketType,
    /// Secondary header flag
    pub secondary_header_presence: SecondaryHeaderPresence,
    /// Application process identified
    pub apid: ApplicationProcessIdentifier,
}

/// A segment of CCSDS packet's primary header that provides information about packet's ID
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SequenceControl {
    /// Sequence flags
    pub flags: SequenceFlags,
    /// Packet sequence count or packet name
    pub name: PacketName,
}

bitfield_enum!(PacketVersionNumber [mask=0xE0, offset=5] {
    CCSDSVersion1 = 0b000,
});

bitfield_enum!(PacketType [mask=0x10, offset=4] {
    Telemetry = 0,
    Telecommand = 1,
});

bitfield_enum!(SecondaryHeaderPresence [mask=0x08, offset=3] {
    NotPresent = 0,
    Present = 1,
});

pub type ApplicationProcessIdentifier = BoundedU16<0, 0x7FF>;
/// Constant representing the APID of an idle CCSDS packet.
#[allow(dead_code)]
pub const IDLE_PACKET_APID: ApplicationProcessIdentifier =
    ApplicationProcessIdentifier::new_saturated(ApplicationProcessIdentifier::HIGH);

impl MultiByteBitField for ApplicationProcessIdentifier {
    const MASKS: [u8; 2] = [0x07, 0xFF];
    const OFFSETS: [usize; 2] = [0, 0];
}

impl MultiByteBitFieldToBytes for ApplicationProcessIdentifier {
    fn to_bytes(self) -> [u8; 2] {
        [(self.get() >> 8) as u8, (self.get() & 0xFF) as u8]
    }
}

impl MultiByteBitFieldFromBytes for ApplicationProcessIdentifier {
    fn from_bytes(bytes: &[u8]) -> Self {
        let msb = (bytes[0] & Self::MASKS[0]) >> Self::OFFSETS[0];
        let lsb = (bytes[1] & Self::MASKS[1]) >> Self::OFFSETS[1];
        let value = u16::from_be_bytes([msb, lsb]);
        // This should never fail, as every value of the field is valid.
        ApplicationProcessIdentifier::new(value).unwrap()
    }
}

bitfield_enum!(SequenceFlags [mask=0xC0, offset=6] {
    UserDataContinuation = 0b00,
    UserDataFirstSegment = 0b01,
    UserDataLastSegment = 0b10,
    UnsegmentedUserData = 0b11,
});

pub type PacketName = BoundedU16<0, 0x3FFF>;

impl MultiByteBitField for PacketName {
    const MASKS: [u8; 2] = [0x3F, 0xFF];
    const OFFSETS: [usize; 2] = [0, 0];
}

impl MultiByteBitFieldToBytes for PacketName {
    fn to_bytes(self) -> [u8; 2] {
        [(self.get() >> 8) as u8, (self.get() & 0xFF) as u8]
    }
}

impl MultiByteBitFieldFromBytes for PacketName {
    fn from_bytes(bytes: &[u8]) -> Self {
        let msb = (bytes[0] & Self::MASKS[0]) >> Self::OFFSETS[0];
        let lsb = (bytes[1] & Self::MASKS[1]) >> Self::OFFSETS[1];
        let value = u16::from_be_bytes([msb, lsb]);
        // This should never fail, as every value of the field is valid.
        PacketName::new(value).unwrap()
    }
}

pub const CCSDS_PRIMARY_HEADER_LENGTH: usize = 6;
pub type CCSDSPrimaryHeaderBuffer = [u8; CCSDS_PRIMARY_HEADER_LENGTH];

/// Every error contains the raw value of the field it failed to recognize.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PacketParsingError {
    InvalidVersionNumber(u8),
}

impl TryFrom<&CCSDSPrimaryHeaderBuffer> for CCSDSPrimaryHeader {
    type Error = PacketParsingError;

    fn try_from(buffer: &CCSDSPrimaryHeaderBuffer) -> Result<Self, Self::Error> {
        let version_number = PacketVersionNumber::try_from_byte(buffer[0])
            .map_err(|err| PacketParsingError::InvalidVersionNumber(err.input))?;

        Ok(Self {
            version_number,
            identity: Identity {
                packet_type: PacketType::from_byte(buffer[0]),
                secondary_header_presence: SecondaryHeaderPresence::from_byte(buffer[0]),
                apid: ApplicationProcessIdentifier::from_bytes(&buffer[0..=1]),
            },
            sequence_control: SequenceControl {
                flags: SequenceFlags::from_byte(buffer[2]),
                name: PacketName::from_bytes(&buffer[2..=3]),
            },
            data_length: u16::from_be_bytes([buffer[4], buffer[5]]),
        })
    }
}

impl From<CCSDSPrimaryHeader> for CCSDSPrimaryHeaderBuffer {
    fn from(header: CCSDSPrimaryHeader) -> Self {
        let apid = header.identity.apid.to_bytes();
        let name = header.sequence_control.name.to_bytes();
        let data_length = header.data_length.to_be_bytes();

        [
            header.version_number.to_byte()
                | header.identity.packet_type.to_byte()
                | header.identity.secondary_header_presence.to_byte()
                | apid[0],
            apid[1],
            header.sequence_control.flags.to_byte() | name[0],
            name[1],
            data_length[0],
            data_length[1],
        ]
    }
}
