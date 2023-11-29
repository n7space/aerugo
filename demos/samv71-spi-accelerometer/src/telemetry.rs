use crate::{
    ccsds::{
        ApplicationProcessIdentifier, CCSDSPrimaryHeader, CCSDSPrimaryHeaderBuffer, Identity,
        PacketName, PacketType, PacketVersionNumber, SecondaryHeaderPresence, SequenceControl,
        SequenceFlags, CCSDS_PRIMARY_HEADER_LENGTH,
    },
    DEMO_TELEMETRY_APID,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TelemetryType {
    StartConfirmed = 0x11,
    StartError = 0x12,
    StopConfirmed = 0x21,
    SetDataOutputRateConfirmed = 0x31,
    SetDataOutputRateError = 0x32,
    SetAccelerometerScaleConfirmed = 0x41,
    SetAccelerometerScaleError = 0x42,
    SetGyroscopeScaleConfirmed = 0x51,
    SetGyroscopeScaleError = 0x52,
    GetExecutionStatsConfirmed = 0x61,
}

pub const TELEMETRY_DATA_BUFFER_LENGTH: usize = 6;
pub const TELEMETRY_PACKET_LENGTH: usize =
    CCSDS_PRIMARY_HEADER_LENGTH + TELEMETRY_DATA_BUFFER_LENGTH;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Telemetry {
    telemetry_type: TelemetryType,
    data: [u8; TELEMETRY_DATA_BUFFER_LENGTH],
}

impl Telemetry {
    pub fn into_ccsds_packet(self) -> [u8; TELEMETRY_PACKET_LENGTH] {
        let header: CCSDSPrimaryHeaderBuffer = CCSDSPrimaryHeader {
            version_number: PacketVersionNumber::CCSDSVersion1,
            identity: Identity {
                packet_type: PacketType::Telemetry,
                secondary_header_presence: SecondaryHeaderPresence::NotPresent,
                apid: ApplicationProcessIdentifier::new(DEMO_TELEMETRY_APID).unwrap(),
            },
            sequence_control: SequenceControl {
                flags: SequenceFlags::UnsegmentedUserData,
                name: PacketName::new(self.telemetry_type as u16).unwrap(),
            },
            data_length: TELEMETRY_DATA_BUFFER_LENGTH as u16,
        }
        .into();

        let mut packet = [0u8; TELEMETRY_PACKET_LENGTH];
        packet[0..CCSDS_PRIMARY_HEADER_LENGTH].copy_from_slice(&header);
        packet[CCSDS_PRIMARY_HEADER_LENGTH..].copy_from_slice(&self.data);
        packet
    }
}
