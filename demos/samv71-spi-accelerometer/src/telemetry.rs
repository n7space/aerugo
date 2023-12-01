use crate::{
    ccsds::{
        ApplicationProcessIdentifier, CCSDSPrimaryHeader, CCSDSPrimaryHeaderBuffer, Identity,
        PacketName, PacketType, PacketVersionNumber, SecondaryHeaderPresence, SequenceControl,
        SequenceFlags, CCSDS_PRIMARY_HEADER_LENGTH,
    },
    DEMO_TELEMETRY_APID,
};
use aerugo::hal::drivers::uart::{Error as IOError, Write};
use lsm6dso::config::data_types::{AngularRate, LinearAcceleration};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TelemetryType {
    AccelerometerData = 0x0A,
    GyroscopeData = 0x0B,
    StartConfirmed = 0x11,
    StartError = 0x12,
    StopConfirmed = 0x21,
    SetDataOutputRateConfirmed = 0x31,
    SetDataOutputRateError = 0x32,
    SetAccelerometerScaleConfirmed = 0x41,
    SetAccelerometerScaleError = 0x42,
    SetGyroscopeScaleConfirmed = 0x51,
    SetGyroscopeScaleError = 0x52,
    ExecutionStatistics = 0x61,
    InvalidTelecommandError = 0xFA,
}

pub const TELEMETRY_DATA_BUFFER_LENGTH: usize = 6;
pub const TELEMETRY_PACKET_LENGTH: usize =
    CCSDS_PRIMARY_HEADER_LENGTH + TELEMETRY_DATA_BUFFER_LENGTH;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Telemetry {
    telemetry_type: TelemetryType,
    data: [u8; TELEMETRY_DATA_BUFFER_LENGTH],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartError {
    DataRateNotSet = 0x01,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetValueError {
    InvalidValue = 0x01,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvalidTelecommandError {
    InvalidCCSDSHeader = 0x01,
    UnexpectedTelemetryReceived = 0x02,
    MalformedPacket = 0x03,
}

impl Telemetry {
    pub fn to_ccsds_packet(&self) -> [u8; TELEMETRY_PACKET_LENGTH] {
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

    pub fn write_ccsds_packet(&self, writer: &mut dyn Write<Error = IOError>) {
        let data = self.to_ccsds_packet();
        writer.write_all(&data).unwrap();
    }

    pub fn new_accelerometer_data(data: LinearAcceleration) -> Self {
        let mut data_buffer = [0u8; 6];
        data_buffer[0..2].copy_from_slice(&data.x.to_le_bytes());
        data_buffer[2..4].copy_from_slice(&data.y.to_le_bytes());
        data_buffer[4..6].copy_from_slice(&data.z.to_le_bytes());

        Self {
            telemetry_type: TelemetryType::AccelerometerData,
            data: data_buffer,
        }
    }

    pub fn new_gyroscope_data(data: AngularRate) -> Self {
        let mut data_buffer = [0u8; 6];
        data_buffer[0..2].copy_from_slice(&data.x.to_le_bytes());
        data_buffer[2..4].copy_from_slice(&data.y.to_le_bytes());
        data_buffer[4..6].copy_from_slice(&data.z.to_le_bytes());

        Self {
            telemetry_type: TelemetryType::GyroscopeData,
            data: data_buffer,
        }
    }

    pub fn new_start_confirmation() -> Self {
        Self::new_empty(TelemetryType::StartConfirmed)
    }

    pub fn new_start_error(error: StartError) -> Self {
        Self::new_error(TelemetryType::StartError, error as u8)
    }

    pub fn new_stop_confirmation() -> Self {
        Self::new_empty(TelemetryType::StopConfirmed)
    }

    pub fn new_set_data_output_rate_confirmation() -> Self {
        Self::new_empty(TelemetryType::SetDataOutputRateConfirmed)
    }

    pub fn new_set_data_output_rate_error(error: SetValueError) -> Self {
        Self::new_error(TelemetryType::SetDataOutputRateError, error as u8)
    }

    pub fn new_set_accelerometer_scale_confirmation() -> Self {
        Self::new_empty(TelemetryType::SetAccelerometerScaleConfirmed)
    }

    pub fn new_set_accelerometer_scale_error(error: SetValueError) -> Self {
        Self::new_error(TelemetryType::SetAccelerometerScaleError, error as u8)
    }

    pub fn new_set_gyroscope_scale_confirmation() -> Self {
        Self::new_empty(TelemetryType::SetGyroscopeScaleConfirmed)
    }

    pub fn new_set_gyroscope_scale_error(error: SetValueError) -> Self {
        Self::new_error(TelemetryType::SetGyroscopeScaleError, error as u8)
    }

    pub fn new_execution_statistics() -> Self {
        Self::new_error(TelemetryType::ExecutionStatistics, 0xDA)
    }

    pub fn new_invalid_telecommand_error(error: InvalidTelecommandError) -> Self {
        Self::new_error(TelemetryType::InvalidTelecommandError, error as u8)
    }

    fn new_empty(telemetry_type: TelemetryType) -> Self {
        Self {
            telemetry_type,
            data: [0u8; TELEMETRY_DATA_BUFFER_LENGTH],
        }
    }

    fn new_error(telemetry_type: TelemetryType, error_code: u8) -> Self {
        Self {
            telemetry_type,
            data: [error_code; TELEMETRY_DATA_BUFFER_LENGTH],
        }
    }
}
