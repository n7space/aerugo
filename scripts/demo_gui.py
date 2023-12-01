"""This script connects to Aerugo's demo board, configures the IMU and begins receiving the data
and plotting it in real-time."""
from __future__ import annotations

import logging
import struct
import sys
from dataclasses import dataclass
from enum import IntEnum
from math import ceil
from typing import Any, cast

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.animation import FuncAnimation
from option import Err, Ok, Result

from calldwell import init_default_logger
from calldwell.ssh_client import SSHClient
from calldwell.uart import RemoteUARTConfig, RemoteUARTConnection, UARTError
from scripts.env import (
    BOARD_LOGIN,
    BOARD_NETWORK_PATH,
    BOARD_PASSWORD,
    BOARD_UART_DEVICE,
    BOARD_UART_PORT,
)

# == Configuration constants ==
DEMO_UART_BAUDRATE = 57_600
PLOT_WINDOW_DURATION_SECONDS = 10
PLOT_UPDATE_INTERVAL_MS = 100
# == End of configuration constants ==


def main() -> None:
    """Main function."""
    ssh = SSHClient(BOARD_NETWORK_PATH, BOARD_LOGIN, BOARD_PASSWORD)
    uart_config = RemoteUARTConfig(
        device_path=BOARD_UART_DEVICE,
        port=BOARD_UART_PORT,
        baudrate=DEMO_UART_BAUDRATE,
    )
    uart = RemoteUARTConnection(ssh, uart_config)

    if uart.open_uart():
        logging.info("UART opened")
    else:
        logging.critical("UART connection couldn't be established, quitting...")
        sys.exit(1)

    start_demo(uart)
    plot_incoming_data(uart)


class TelecommandType(IntEnum):
    """Telecommand type"""

    START = 0x10
    STOP = 0x20
    SET_DATA_OUTPUT_RATE = 0x30
    SET_ACCELEROMETER_SCALE = 0x40
    SET_GYROSCOPE_SCALE = 0x50
    GET_EXECUTION_STATS = 0x60


class DataRate(IntEnum):
    """IMU data rate"""

    ODR_12_5HZ = 0x01
    ODR_26HZ = 0x02
    ODR_52HZ = 0x03
    ODR_104HZ = 0x04
    ODR_208HZ = 0x05
    ODR_416HZ = 0x06
    ODR_833HZ = 0x07
    ODR_1667HZ = 0x08
    ODR_3333HZ = 0x09
    ODR_6667HZ = 0x0A

    def to_hertz(self: DataRate) -> float:
        """Returns the data rate in hertz"""
        return {
            DataRate.ODR_12_5HZ: 12.5,
            DataRate.ODR_26HZ: 26,
            DataRate.ODR_52HZ: 52,
            DataRate.ODR_104HZ: 104,
            DataRate.ODR_208HZ: 208,
            DataRate.ODR_416HZ: 416,
            DataRate.ODR_833HZ: 833,
            DataRate.ODR_1667HZ: 1666.666,
            DataRate.ODR_3333HZ: 3333.333,
            DataRate.ODR_6667HZ: 6666.666,
        }[self]


class AccelerometerScale(IntEnum):
    """Accelerometer scale"""

    SCALE_2G = 0x01
    SCALE_4G = 0x02
    SCALE_8G = 0x03
    SCALE_16G = 0x04


class GyroscopeScale(IntEnum):
    """Gyroscope scale"""

    SCALE_125DPS = 0x01
    SCALE_250DPS = 0x02
    SCALE_500DPS = 0x03
    SCALE_1000DPS = 0x04
    SCALE_2000DPS = 0x05


class TelemetryType(IntEnum):
    """Telemetry type"""

    ACCELEROMETER_DATA = 0x0A
    GYROSCOPE_DATA = 0x0B
    START_CONFIRMED = 0x11
    START_ERROR = 0x12
    STOP_CONFIRMED = 0x21
    SET_DATA_OUTPUT_RATE_CONFIRMED = 0x31
    SET_DATA_OUTPUT_RATE_ERROR = 0x32
    SET_ACCELEROMETER_SCALE_CONFIRMED = 0x41
    SET_ACCELEROMETER_SCALE_ERROR = 0x42
    SET_GYROSCOPE_SCALE_CONFIRMED = 0x51
    SET_GYROSCOPE_SCALE_ERROR = 0x52
    EXECUTION_STATISTICS = 0x61
    INVALID_TELECOMMAND_ERROR = 0xFA


CCSDS_HEADER_LENGTH = 6
TELEMETRY_DATA_LENGTH = 6
TELEMETRY_PACKET_LENGTH = CCSDS_HEADER_LENGTH + TELEMETRY_DATA_LENGTH
PLOT_UPDATE_FREQUENCY = 1000 / PLOT_UPDATE_INTERVAL_MS
PLOT_WINDOW_SIZE = int(PLOT_WINDOW_DURATION_SECONDS * PLOT_UPDATE_FREQUENCY)
DATA_OUTPUT_RATE: DataRate | None = None


@dataclass
class Vec3D:
    """3D vector"""

    x: int
    y: int
    z: int

    def __add__(self: Vec3D, other: Vec3D) -> Vec3D:
        return Vec3D(self.x + other.x, self.y + other.y, self.z + other.z)

    def __truediv__(self: Vec3D, other: int) -> Vec3D:
        return Vec3D(int(self.x / other), int(self.y / other), int(self.z / other))

    @staticmethod
    def average_of(vectors: list[Vec3D]) -> Vec3D:
        """Returns average calculated from vectors on the list"""
        return sum(vectors, Vec3D(0, 0, 0)) / len(vectors)


@dataclass
class Telemetry:
    """Telemetry"""

    telemetry_type: TelemetryType
    payload: bytes

    @staticmethod
    def from_bytes(data: bytes) -> Telemetry:
        """Creates telemetry instance from byte array"""
        return Telemetry(telemetry_type=TelemetryType(data[3]), payload=data[CCSDS_HEADER_LENGTH:])

    def is_error(self: Telemetry) -> bool:
        """Returns `True` if telemetry contains error code"""
        return self.telemetry_type in (
            TelemetryType.START_ERROR,
            TelemetryType.SET_DATA_OUTPUT_RATE_ERROR,
            TelemetryType.SET_ACCELEROMETER_SCALE_ERROR,
            TelemetryType.SET_GYROSCOPE_SCALE_ERROR,
            TelemetryType.INVALID_TELECOMMAND_ERROR,
        )

    def is_success(self: Telemetry) -> bool:
        """Returns `True` if telemetry contains operation confirmation"""
        return self.telemetry_type in (
            TelemetryType.START_CONFIRMED,
            TelemetryType.STOP_CONFIRMED,
            TelemetryType.SET_DATA_OUTPUT_RATE_CONFIRMED,
            TelemetryType.SET_ACCELEROMETER_SCALE_CONFIRMED,
            TelemetryType.SET_GYROSCOPE_SCALE_CONFIRMED,
        )

    def is_data(self: Telemetry) -> bool:
        """Returns `True` if telemetry contains data"""
        return self.telemetry_type in (
            TelemetryType.ACCELEROMETER_DATA,
            TelemetryType.GYROSCOPE_DATA,
            TelemetryType.EXECUTION_STATISTICS,
        )

    def get_imu_data(self: Telemetry) -> Vec3D | None:
        """Returns data from IMU, or None if telemetry doesn't contain data"""
        if self.telemetry_type not in (
            TelemetryType.ACCELEROMETER_DATA,
            TelemetryType.GYROSCOPE_DATA,
        ):
            return None

        x, y, z = struct.unpack("<hhh", self.payload)
        return Vec3D(x, y, z)

    def get_execution_stats(self: Telemetry) -> int | None:
        """Returns execution statistics, or None if telemetry doesn't contain execution
        statistics"""
        if self.telemetry_type is not TelemetryType.EXECUTION_STATISTICS:
            return None

        return 42


def create_demo_telecommand(command: TelecommandType, payload: int) -> bytes:
    """Creates a telecommand for demo app with specified opcode and payload byte"""
    return bytes([0x10, 0xAA, 0xC0, int(command), 0x00, 0x01, int(payload)])


def receive_telemetry(uart: RemoteUARTConnection) -> Result[Telemetry, UARTError]:
    """Reads telemetry packet from UART and returns it"""
    response_bytes = uart.read_exact_bytes(TELEMETRY_PACKET_LENGTH)
    if response_bytes.is_err:
        return Err(response_bytes.unwrap_err())
    return Ok(Telemetry.from_bytes(response_bytes.unwrap()))


def send_telecommand(uart: RemoteUARTConnection, command: TelecommandType, payload: int) -> bool:
    """Constructs and sends a telecommand via UART"""
    logging.info(f"Sending {command} with payload {payload}")
    _, transmission_error = uart.write_bytes(create_demo_telecommand(command, payload))
    if transmission_error is not None:
        logging.error(f"An error occurred while sending telecommand: {transmission_error}")
        return False

    return True


def send_and_validate_telecommand(
    uart: RemoteUARTConnection,
    command: TelecommandType,
    payload: int,
) -> bool:
    """Constructs and sends a telecommand via UART, and then waits for it's confirmation"""
    if not send_telecommand(uart, command, payload):
        return False

    response_result = receive_telemetry(uart)
    if response_result.is_err:
        logging.error(f"Could not receive data from demo app: {response_result.unwrap_err()}")
        return False

    response = response_result.unwrap()
    logging.info(f"Received telemetry response: {response}")
    return response.is_success()


def start_demo(uart: RemoteUARTConnection) -> None:
    """Configures the demo application"""
    send_and_validate_telecommand(
        uart,
        TelecommandType.SET_ACCELEROMETER_SCALE,
        AccelerometerScale.SCALE_8G,
    )

    send_and_validate_telecommand(
        uart,
        TelecommandType.SET_GYROSCOPE_SCALE,
        GyroscopeScale.SCALE_250DPS,
    )

    global DATA_OUTPUT_RATE  # noqa: PLW0603 pylint: disable=global-statement
    DATA_OUTPUT_RATE = DataRate.ODR_26HZ
    send_and_validate_telecommand(uart, TelecommandType.SET_DATA_OUTPUT_RATE, DATA_OUTPUT_RATE)
    send_and_validate_telecommand(uart, TelecommandType.START, 0x00)


def _print_data_from_demo(uart: RemoteUARTConnection) -> None:
    """Continuously prints the data from demo application, call it right after starting the
    measurements"""
    while True:
        telemetry = receive_telemetry(uart).unwrap()
        data_kind = (
            "acceleration"
            if telemetry.telemetry_type == TelemetryType.ACCELEROMETER_DATA
            else "angular rotation"
        )
        print(f"Received {data_kind} data: {telemetry.get_imu_data()}")


@dataclass
class AxisWindow:
    """Class representing a window for plot axis"""

    data: Any
    current_index: int

    def add_value(self: AxisWindow, value: int) -> bool:
        """Adds value at the end of the window.
        If window was full and has shifted, returns `False`.
        If window was not full, returns `True`."""
        if self.current_index == len(self.data):
            self.data = np.roll(self.data, -1)
            self.data[-1] = value
            return False

        self.data[self.current_index] = value
        self.current_index += 1
        return True


X_AXIS_DATA = AxisWindow(
    data=np.linspace(0, PLOT_WINDOW_SIZE, PLOT_WINDOW_SIZE),
    current_index=0,
)
ACCEL_DATA_X = AxisWindow(
    data=np.linspace(0, 0, PLOT_WINDOW_SIZE),
    current_index=0,
)
ACCEL_DATA_Y = AxisWindow(
    data=np.linspace(0, 0, PLOT_WINDOW_SIZE),
    current_index=0,
)
ACCEL_DATA_Z = AxisWindow(
    data=np.linspace(0, 0, PLOT_WINDOW_SIZE),
    current_index=0,
)
GYRO_DATA_X = AxisWindow(
    data=np.linspace(0, 0, PLOT_WINDOW_SIZE),
    current_index=0,
)
GYRO_DATA_Y = AxisWindow(
    data=np.linspace(0, 0, PLOT_WINDOW_SIZE),
    current_index=0,
)
GYRO_DATA_Z = AxisWindow(
    data=np.linspace(0, 0, PLOT_WINDOW_SIZE),
    current_index=0,
)


def add_sample_to_plots(accelerometer_data: Vec3D, gyroscope_data: Vec3D) -> bool:
    """Returns `True` if adding a payload caused the window to widen"""
    ACCEL_DATA_X.add_value(accelerometer_data.x)
    ACCEL_DATA_Y.add_value(accelerometer_data.y)
    ACCEL_DATA_Z.add_value(accelerometer_data.z)

    GYRO_DATA_X.add_value(gyroscope_data.x)
    GYRO_DATA_Y.add_value(gyroscope_data.y)
    return GYRO_DATA_Z.add_value(gyroscope_data.z)


def process_measurements(measurements: list[Telemetry]) -> tuple[Vec3D, Vec3D]:
    """Processes a list of telecommands with measurements and returns averaged accelerometer and
    gyroscope data"""

    accelerometer_measurements = [
        cast(Vec3D, measurement.get_imu_data())
        for measurement in measurements
        if measurement.telemetry_type == TelemetryType.ACCELEROMETER_DATA
    ]
    gyroscope_measurements = [
        cast(Vec3D, measurement.get_imu_data())
        for measurement in measurements
        if measurement.telemetry_type == TelemetryType.GYROSCOPE_DATA
    ]

    accel_avg = (
        Vec3D.average_of(accelerometer_measurements)
        if len(accelerometer_measurements) > 0
        else Vec3D(0, 0, 0)
    )
    gyro_avg = (
        Vec3D.average_of(gyroscope_measurements)
        if len(gyroscope_measurements) > 0
        else Vec3D(0, 0, 0)
    )

    return accel_avg, gyro_avg


def fetch_new_data(uart: RemoteUARTConnection) -> None:
    """This function is responsible for fetching new data from demo app and placing it appropriately
    in plot buffers (ACCEL/GYRO_DATA_X/Y/Z). Plot is refreshed much slower than data is received,
    and since the data is buffered it can be fetched "on demand". Therefore, this function
    fetches all the samples that should've been buffered during since last update and averages them
    to create new data point on the plot."""

    if DATA_OUTPUT_RATE is None:
        msg = "data output rate must be set before starting the plot"
        raise RuntimeError(msg)

    # we want to fetch at least 1 sample per sensor each update
    data_to_fetch = max(ceil(DATA_OUTPUT_RATE.to_hertz() * (PLOT_UPDATE_INTERVAL_MS / 1000)), 1)
    # *2 because we have 2 sensors
    logging.info(f"Data to fetch: {data_to_fetch}, ODR: {DATA_OUTPUT_RATE.to_hertz()}")
    measurements = [receive_telemetry(uart).unwrap() for _ in range(data_to_fetch * 2)]
    average_acceleration, average_rotation = process_measurements(measurements)
    add_sample_to_plots(average_acceleration, average_rotation)

    X_AXIS_DATA.current_index = min(ACCEL_DATA_Z.current_index, GYRO_DATA_Z.current_index)


def plot_incoming_data(uart: RemoteUARTConnection) -> None:
    """Creates a plot that's updated in real-time by fetching the data from demo application"""
    figure, axes = plt.subplots()

    accel_plot_x = axes.plot(0, 0, label="a (x) [m/s]", color="red")[0]
    accel_plot_y = axes.plot(0, 0, label="a (y) [m/s]", color="darkred")[0]
    accel_plot_z = axes.plot(0, 0, label="a (z) [m/s]", color="coral")[0]

    gyro_plot_x = axes.plot(0, 0, label="g (x) [deg/s]", color="royalblue")[0]
    gyro_plot_y = axes.plot(0, 0, label="g (y) [deg/s]", color="blue")[0]
    gyro_plot_z = axes.plot(0, 0, label="g (z) [deg/s]", color="indigo")[0]

    axes.set(
        xlim=[0, PLOT_WINDOW_SIZE],
        ylim=[-65535, 65535],
        xlabel="Time",
        ylabel="Value",
    )
    axes.legend()

    def update_plot(_frame: int) -> tuple[Any, Any, Any, Any, Any, Any]:
        """Plot update function"""
        fetch_new_data(uart)

        accel_plot_x.set_xdata(X_AXIS_DATA.data[: X_AXIS_DATA.current_index])
        accel_plot_x.set_ydata(ACCEL_DATA_X.data[: X_AXIS_DATA.current_index])
        accel_plot_y.set_xdata(X_AXIS_DATA.data[: X_AXIS_DATA.current_index])
        accel_plot_y.set_ydata(ACCEL_DATA_Y.data[: X_AXIS_DATA.current_index])
        accel_plot_z.set_xdata(X_AXIS_DATA.data[: X_AXIS_DATA.current_index])
        accel_plot_z.set_ydata(ACCEL_DATA_Z.data[: X_AXIS_DATA.current_index])

        gyro_plot_x.set_xdata(X_AXIS_DATA.data[: X_AXIS_DATA.current_index])
        gyro_plot_x.set_ydata(GYRO_DATA_X.data[: X_AXIS_DATA.current_index])
        gyro_plot_y.set_xdata(X_AXIS_DATA.data[: X_AXIS_DATA.current_index])
        gyro_plot_y.set_ydata(GYRO_DATA_Y.data[: X_AXIS_DATA.current_index])
        gyro_plot_z.set_xdata(X_AXIS_DATA.data[: X_AXIS_DATA.current_index])
        gyro_plot_z.set_ydata(GYRO_DATA_Z.data[: X_AXIS_DATA.current_index])

        return (accel_plot_x, accel_plot_y, accel_plot_z, gyro_plot_x, gyro_plot_y, gyro_plot_z)

    _anim = FuncAnimation(
        fig=figure,
        func=update_plot,
        frames=9999,
        interval=PLOT_UPDATE_INTERVAL_MS,
        repeat=False,
    )
    plt.show()


if __name__ == "__main__":
    init_default_logger()
    main()
