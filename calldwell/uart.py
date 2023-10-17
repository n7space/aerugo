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

from option import Err, Ok, Result

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


class UARTError(Enum):
    """Enumeration representing possible UART I/O errors."""

    UART_IS_CLOSED = auto()
    INVALID_TIMEOUT = auto()
    TIMEOUT_HIT = auto()
    INVALID_LENGTH = auto()


class RemoteUARTConnection:
    """Remote UART connection"""

    DEFAULT_CHUNK_SIZE = 4096
    """Default maximum length of received data."""

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
        self._rx_buffer = bytearray()

    @property
    def config(self: RemoteUARTConnection) -> RemoteUARTConfig:
        """Returns UART configuration"""
        return self._config

    @property
    def is_open(self: RemoteUARTConnection) -> bool:
        """Returns `True` if port is currently open"""
        return self._is_open

    def open_uart(self: RemoteUARTConnection, timeout: float = 3.0) -> bool:
        """Opens UART port on remote, and connects to it via socat.
        Returns `True` if UART connection has been established, `False` if UART is already open or
        if connection cannot be established."""
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
        """Closes socat connection and UART port on remote.
        Returns `True` if connection has been closed, `False` if UART is not open."""
        if not self._is_open or self._uart_socket is None:
            return False

        if self._uart_socket is not None:
            self._uart_socket.shutdown(socket.SHUT_RDWR)
            self._uart_socket.close()
            self._uart_socket = None
        self._close_socat_bridge()

        self._is_open = False
        return True

    def write_bytes(
        self: RemoteUARTConnection,
        data: bytes,
        timeout_seconds: float = 5.0,
    ) -> tuple[int, UARTError | None]:
        """Transmits some data via UART.
        Returns a tuple. First element is always present and indicates how many bytes have been
        sent. Second element is present only on error, and indicates what went wrong."""
        if (not self._is_open) or (self._uart_socket is None):
            return 0, UARTError.UART_IS_CLOSED

        if timeout_seconds < 0:
            return 0, UARTError.INVALID_TIMEOUT

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

        if elapsed_time >= timeout_seconds:
            return sent_bytes_sum, UARTError.TIMEOUT_HIT

        return sent_bytes_sum, None

    def write_string(
        self: RemoteUARTConnection,
        message: str,
        timeout_seconds: float = 3.0,
    ) -> tuple[int, UARTError | None]:
        """Transmits an UTF-8 string via UART. Will throw an exception on invalid string.
        Returns a tuple. First element is always present and indicates how many **bytes** have been
        sent. Second element is present only on error, and indicates what went wrong."""
        return self.write_bytes(message.encode("utf-8"), timeout_seconds)

    def read_bytes(
        self: RemoteUARTConnection,
        timeout_seconds: float = 3.0,
        maximum_length: int = DEFAULT_CHUNK_SIZE,
    ) -> Result[bytes, UARTError]:
        """Reads any available amount of bytes from UART.
        Returns new data immediately when available.
        Returns `UARTError` on timeout or invalid arguments."""
        if (not self._is_open) or (self._uart_socket is None):
            return Err(UARTError.UART_IS_CLOSED)

        if timeout_seconds < 0:
            return Err(UARTError.INVALID_TIMEOUT)

        if maximum_length <= 0:
            return Err(UARTError.INVALID_LENGTH)

        # First, let's try checking if there's any new data from UART in non-blocking fashion.
        # We do that by reading any available bytes into internal buffer, as there might be
        # some data left from previous operations, which we don't want to discard and should
        # return first.
        # If that operation returns an error other than TIMEOUT_HIT, we should abort, as it's an
        # unexpected error that should be handled by the user.
        read_to_buffer_result = self._read_bytes_to_internal_buffer_non_blocking(maximum_length)
        if read_to_buffer_result.is_err:
            return Err(read_to_buffer_result.unwrap_err())

        if len(buffered_data := self._take_internal_buffer_content()) > 0:
            return Ok(buffered_data)

        # If internal RX buffer is empty, we can safely fetch new data directly from socket.
        return self._read_bytes_from_socket(timeout_seconds, maximum_length)

    def read_exact_bytes(
        self: RemoteUARTConnection,
        length: int,
        timeout_seconds: float = 3.0,
    ) -> Result[bytes, UARTError]:
        """Reads exact amount of bytes from UART.
        Returns new data immediately when available.
        Returns `Err(UARTError)` on timeout or invalid arguments."""
        read_bytes = bytearray()

        while len(read_bytes) < length:
            remaining_bytes = length - len(read_bytes)
            read_bytes_result = self.read_bytes(timeout_seconds, remaining_bytes)
            if read_bytes_result.is_err:
                # Propagate any hard error
                return Err(read_bytes_result.unwrap_err())

            read_bytes.extend(read_bytes_result.unwrap())

        return Ok(bytes(read_bytes))

    def read_string(
        self: RemoteUARTConnection,
        terminator: bytes,
        timeout_seconds: float = 3.0,
        maximum_length: int = DEFAULT_CHUNK_SIZE,
    ) -> Result[str, UARTError]:
        """Reads a string terminated by specified byte(s).
        Returns a string immediately when available.
        Returns `Err(UARTError)` on invalid arguments or timeout.
        May throw an exception if received string (decoded after receiving all the bytes) is not a
        valid UTF-8 string."""
        # Similarly to `read_bytes`, fetch any data that's immediately available into internal
        # buffer, and return if it fails (timeout is silenced in non-blocking function).
        buffer_read_result = self._read_bytes_to_internal_buffer_non_blocking(maximum_length)
        if buffer_read_result.is_err:
            return Err(buffer_read_result.unwrap_err())

        if (terminator_index := self._rx_buffer.find(terminator)) >= 0:
            return Ok(
                self._take_bytes_out_of_rx_buffer(terminator_index + len(terminator)).decode(
                    "UTF-8",
                ),
            )

        # If a valid string isn't yet available in the buffer, try to read it again, but in
        # blocking mode. Repeat until timeout is hit to make sure that data in chunks is
        # received correctly.

        while (
            buffer_read_result := self._read_bytes_to_internal_buffer(
                timeout_seconds,
                maximum_length,
            )
        ).is_ok:
            if (terminator_index := self._rx_buffer.find(terminator)) >= 0:
                return Ok(
                    self._take_bytes_out_of_rx_buffer(terminator_index + len(terminator)).decode(
                        "UTF-8",
                    ),
                )

        # If `read_result` is not an integer, then it's an error which should be propagated.
        # Received data will stay in RX buffer until it's taken via `read_bytes` or some other
        # function.
        return Err(buffer_read_result.unwrap_err())

    def _take_bytes_out_of_rx_buffer(self: RemoteUARTConnection, amount: int) -> bytes:
        extracted_bytes = self._rx_buffer[0:amount]
        self._rx_buffer = self._rx_buffer[amount:]
        return extracted_bytes

    def _read_bytes_to_internal_buffer_non_blocking(
        self: RemoteUARTConnection,
        maximum_length: int,
    ) -> Result[int, UARTError]:
        """Read any available bytes to internal buffer.
        If there's no bytes to read, returns `Ok(0)` immediately.
        Returns amount of read bytes, or `Err(UARTError)` if UART is not opened."""
        read_result = self._read_bytes_to_internal_buffer(0, maximum_length)

        # Silence timeout, as we don't care about it in non-blocking operation.
        if read_result.is_err and read_result.unwrap_err() == UARTError.TIMEOUT_HIT:
            return Ok(0)

        return read_result

    def _read_bytes_to_internal_buffer(
        self: RemoteUARTConnection,
        timeout_seconds: float,
        maximum_length: int,
    ) -> Result[int, UARTError]:
        """Reads any available data from UART into internal RX buffer.
        Returns the amount of bytes put in RX buffer, or `Err(UARTError)` in case of invalid
        arguments or when timeout is hit."""
        reading_result = self._read_bytes_from_socket(timeout_seconds, maximum_length)
        if reading_result.is_ok:
            data = reading_result.unwrap()
            self._rx_buffer.extend(data)
            return Ok(len(data))
        return Err(reading_result.unwrap_err())

    def _read_bytes_from_socket(
        self: RemoteUARTConnection,
        timeout_seconds: float,
        maximum_length: int,
    ) -> Result[bytes, UARTError]:
        """Reads any available data from UART and returns it, or returns `Err(UARTError)` in case of
        invalid arguments or when timeout is hit."""
        if (not self._is_open) or (self._uart_socket is None):
            return Err(UARTError.UART_IS_CLOSED)

        if timeout_seconds < 0:
            return Err(UARTError.INVALID_TIMEOUT)

        try:
            readable, _, _ = select.select([self._uart_socket], [], [], timeout_seconds)
            if readable:
                return Ok(bytes(self._uart_socket.recv(maximum_length)))
        except OSError:  # timeout
            pass

        return Err(UARTError.TIMEOUT_HIT)

    def _take_internal_buffer_content(self: RemoteUARTConnection) -> bytes:
        """Returns the whole content of internal buffer, and clears it."""
        buffered_data = bytes(self._rx_buffer.copy())
        self._rx_buffer.clear()
        return buffered_data

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
        # If you get an exception there, fix your destruction order manually.
        # It may happen if SSH session is killed before UART.
        # Make sure you close UART before closing the SSH session.
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
