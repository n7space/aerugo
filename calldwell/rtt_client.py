"""Module containing RTT-related classes, which also provide an easy-to-use layer of
abstraction over Calldwell streams/messages."""

import socket
from enum import IntEnum
from typing import Optional


class RTTClient:
    """Class acting as RTT front-end. Provides buffered, bidirectional communication with
    debugged program. Can also be used as a convenient base for custom protocols."""

    def __init__(self, host: str, port: int, default_chunk_size: int = 1024) -> None:
        """Create instance of RTT client. Connects to RTT server via TCP socket.

        # Parameters
        * `host` - either a hostname or IP address of RTT server
        * `port` - port of RTT server
        * `default_chunk_size` - Chunk size used for receiving data.
        """
        self._socket = socket.socket()
        self._socket.connect((host, port))
        self._default_chunk_size = default_chunk_size
        self._data_buffer = bytearray()

    def close(self) -> None:
        """Closes the RTT connection gracefully."""
        self._socket.shutdown(socket.SHUT_RDWR)
        self._socket.close()

    def receive(self) -> bytes:
        self._receive()
        data = self._data_buffer.copy()
        self._data_buffer.clear()
        return data

    def transmit(self, data: bytes) -> None:
        self._transmit(data)

    def receive_string(self) -> str:
        return self.receive().decode("utf-8")

    def transmit_string(self, data: str) -> None:
        self.transmit(data.encode("utf-8"))

    def _receive(self, chunk_size: Optional[int] = None) -> None:
        """Receives raw data from RTT target to internal buffer"""
        if chunk_size is None:
            chunk_size = self._default_chunk_size

        received_bytes = self._socket.recv(chunk_size)
        self._data_buffer.extend(received_bytes)

    def _transmit(self, data: bytes) -> None:
        """Transmits raw data to RTT target."""
        self._socket.send(data)


class CalldwellRTTClient(RTTClient):
    """Class providing bidirectional communication with program using Calldwell streams"""

    class StreamMarker(IntEnum):
        """Enumeration listing Calldwell stream markers"""

        Start = 0xDD
        End = 0xEE

    def receive_bytes_stream(self) -> bytes:
        """Receives data via Calldwell stream from RTT target"""
        stream_data = self._extract_stream_data_from_recv_buffer()
        while stream_data is None:
            self._receive()
            stream_data = self._extract_stream_data_from_recv_buffer()

        return stream_data

    def transmit_bytes_stream(self, data: bytes) -> None:
        """Transmits data via Calldwell stream to RTT target"""
        self._transmit_stream_marker(CalldwellRTTClient.StreamMarker.Start)
        self._transmit(data)
        self._transmit_stream_marker(CalldwellRTTClient.StreamMarker.End)

    def receive_string_stream(self) -> str:
        """Receives an UTF-8 string via Calldwell stream from RTT target"""
        return self.receive_bytes_stream().decode("utf-8")

    def transmit_string_stream(self, message: str) -> None:
        """Transmits an UTF-8 string via Calldwell stream to RTT target"""
        self.transmit_bytes_stream(message.encode("utf-8"))

    def _extract_stream_data_from_recv_buffer(self) -> Optional[bytes]:
        """Looks for valid Calldwell stream in reception buffer, and returns it's data if found"""
        start_marker_index = self._data_buffer.find(CalldwellRTTClient.StreamMarker.Start)
        if start_marker_index == -1:
            return None

        end_marker_index = self._data_buffer.find(
            CalldwellRTTClient.StreamMarker.End, start_marker_index
        )
        if end_marker_index == -1:
            return None

        stream_data = self._data_buffer[start_marker_index + 1 : end_marker_index]

        # Remove everything up to end marker from the buffer
        if end_marker_index != len(self._data_buffer):
            self._data_buffer = self._data_buffer[end_marker_index + 1 :]
        else:
            self._data_buffer.clear()

        return bytes(stream_data)

    def _transmit_stream_marker(self, marker: StreamMarker) -> None:
        # byteorder doesn't matter, but mypy asks for it
        self._transmit(marker.to_bytes(length=1, signed=False, byteorder="big"))
