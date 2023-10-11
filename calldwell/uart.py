"""Module containing classes for UART management.
UART port is configured using `stty` and forwarded to TCP port using `socat`.

Convenience functions that create remote UART TCP socket and connect to it using
local socket are provided."""

from __future__ import annotations

import select
import socket
import time
from dataclasses import dataclass
from enum import Enum, auto
from typing import TYPE_CHECKING, cast

from .utils import wait_until_true

if TYPE_CHECKING:
    from .ssh_client import SSHClient


class StopBits(Enum):
    """Enumeration representing the amount of stop bits"""

    ONE = 1
    TWO = 2


class Parity(Enum):
    """Enumeration representing parity bit configuration"""

    NONE = auto()
    EVEN = auto()
    ODD = auto()


@dataclass
class RemoteUARTConfig:
    """Remote UART configuration"""

    device_path: str
    """Path to UART device on target's filesystem, for example /dev/ttyUSB0"""
    port: int
    """TCP port for socat's UART connection"""
    baudrate: int
    data_bits: int = 8
    stop_bits: StopBits = StopBits.ONE
    parity: Parity = Parity.NONE


class UARTConnectionError(Exception):
    """Exception indicating UART connection error"""


class BrokenSocketError(Exception):
    """Exception indicating an error involving broken TCP socket"""

    def __init__(self: BrokenSocketError, when: str) -> None:
        super().__init__(f"Socket connection broken when {when}")


class RemoteUARTConnection:
    """Remote UART connection"""

    def __init__(
        self: RemoteUARTConnection,
        ssh_connection: SSHClient,
        config: RemoteUARTConfig,
    ) -> None:
        self._config = config
        self._ssh = ssh_connection

        self._socat_pid = 0
        self._uart_socket: socket.socket | None = None
        self._is_open = False

    @property
    def config(self: RemoteUARTConnection) -> RemoteUARTConfig:
        """Returns UART configuration"""
        return self._config

    @property
    def is_open(self: RemoteUARTConnection) -> bool:
        """Returns `True` if port is currently open"""
        return self._is_open

    def open_uart(self: RemoteUARTConnection, timeout: float = 3.0) -> bool:
        """Opens UART port on remote, and connects to it via socat"""
        if self._is_open:
            return False

        socket_location = (
            self._ssh.host,
            self._config.port,
        )

        self._open_socat_bridge(self._config)

        # Now, socat should listen to connection on port specified in UART config, so we should be
        # able to connect it via TCP socket
        self._uart_socket = socket.socket()
        self._uart_socket.setblocking(False)

        def check_connection_status() -> bool:
            # We just created a new socket, so `cast` here is appropriate, as it cannot be None
            return cast(socket.socket, self._uart_socket).connect_ex(socket_location) == 0

        if wait_until_true(check_connection_status, timeout_seconds=timeout) is not None:
            self._is_open = True
            return True

        return False

    def close_uart(self: RemoteUARTConnection) -> bool:
        """Closes socat connection and UART port on remote"""
        if not self._is_open or self._uart_socket is None:
            return False

        if self._uart_socket is not None:
            self._uart_socket.shutdown(socket.SHUT_RDWR)
            self._uart_socket.close()
            self._uart_socket = None
        self._close_socat_bridge()

        self._is_open = False
        return True

    def write(self: RemoteUARTConnection, data: bytes, timeout_seconds: float = 5.0) -> int | None:
        """Transmits some data to UART"""
        if not self._is_open or self._uart_socket is None or timeout_seconds <= 0:
            return None

        start_time = time.time()
        elapsed_time = 0.0
        sent_bytes_sum = 0

        while len(data) > 0 and elapsed_time < timeout_seconds:
            _, writable, _ = select.select([], [self._uart_socket], [], timeout_seconds)

            if writable:
                if (sent := self._uart_socket.send(data)) <= 0:
                    raise BrokenSocketError(when="trying to send data")
                sent_bytes_sum += sent
                data = data[sent:]

            elapsed_time = time.time() - start_time

        return sent_bytes_sum

    def read_any(
        self: RemoteUARTConnection,
        timeout_seconds: float = 5.0,
        maximum_length: int = 255,
    ) -> bytes | None:
        """Reads any available amount of bytes from UART.
        Returns new data immediately when available.
        Returns `None` on timeout"""
        if not self._is_open or self._uart_socket is None or timeout_seconds <= 0:
            return None

        received_data = None

        try:
            readable, _, _ = select.select([self._uart_socket], [], [], timeout_seconds)
            if readable:
                received_data = self._uart_socket.recv(maximum_length)
        except OSError:
            pass

        return received_data

    def _open_socat_bridge(
        self: RemoteUARTConnection,
        config: RemoteUARTConfig,
    ) -> None:
        stty_command = " ".join(["stty", *self._generate_stty_arguments(config)])
        socat_command = " ".join(["socat", *self._generate_socat_arguments(config)])

        # In order to create socat bridge for UART, stty must be executed first.
        _, streams = self._ssh.execute(stty_command)
        # We can handler stty errors like that, but socat will block permanently if
        # we try to read stderr.
        if streams.stderr.readable():
            error_log = streams.stderr.readlines()
            if len(error_log) != 0:
                raise UARTConnectionError(error_log)

        # Store socat's PID for future cleanup
        self._socat_pid, streams = self._ssh.execute(socat_command)

    def _close_socat_bridge(self: RemoteUARTConnection) -> None:
        if self._socat_pid != 0:
            self._ssh.execute(f"kill {self._socat_pid}")
            self._socat_pid = 0

    def __del__(self: RemoteUARTConnection) -> None:
        self.close_uart()

    @staticmethod
    def _generate_socat_arguments(config: RemoteUARTConfig) -> list[str]:
        return [
            f"{config.device_path},b{config.baudrate},rawer,iexten=0,icanon=0,echo=0",
            f"TCP-L:{config.port},reuseaddr",
        ]

    @staticmethod
    def _parity_to_stty_arg(parity: Parity) -> list[str]:
        if parity == Parity.EVEN:
            return ["parenb", "-parodd"]

        if parity == Parity.ODD:
            return ["parenb", "parodd"]

        return ["-parenb"]

    @staticmethod
    def _generate_stty_arguments(config: RemoteUARTConfig) -> list[str]:
        stop_bits_arg = "-cstopb" if config.stop_bits == StopBits.ONE else "cstopb"
        parity_arg = RemoteUARTConnection._parity_to_stty_arg(config.parity)

        return [
            "-F",
            config.device_path,
            str(config.baudrate),
            f"cs{config.data_bits}",
            stop_bits_arg,
            *parity_arg,
        ]
