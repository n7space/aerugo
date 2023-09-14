"""Module containing RTT-related classes, which also provide an easy-to-use layer of
abstraction over Calldwell streams/messages."""

from __future__ import annotations

import socket
from enum import IntEnum


class RTTClient:
    """Class acting as RTT front-end. Provides buffered, bidirectional communication with
    debugged program. Can also be used as a convenient base for custom protocols."""

    def __init__(self: RTTClient, host: str, port: int, default_chunk_size: int = 1024) -> None:
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

    def close(self: RTTClient) -> None:
        """Closes the RTT connection gracefully."""
        self._socket.shutdown(socket.SHUT_RDWR)
        self._socket.close()

    def receive(self: RTTClient) -> bytes:
        """Receives raw data via RTT"""
        self._receive()
        data = self._data_buffer.copy()
        self._data_buffer.clear()
        return data

    def transmit(self: RTTClient, data: bytes) -> None:
        """Transmits raw data via RTT"""
        self._transmit(data)

    def receive_string(self: RTTClient) -> str:
        """Receives raw data via RTT and decodes it into UTF-8 string"""
        return self.receive().decode("utf-8")

    def transmit_string(self: RTTClient, data: str) -> None:
        """Encodes the data into UTF-8 string and transmits it via RTT"""
        self.transmit(data.encode("utf-8"))

    def _receive(self: RTTClient, chunk_size: int | None = None) -> None:
        """Receives raw data from RTT target to internal buffer"""
        if chunk_size is None:
            chunk_size = self._default_chunk_size

        received_bytes = self._socket.recv(chunk_size)
        self._data_buffer.extend(received_bytes)

    def _transmit(self: RTTClient, data: bytes) -> None:
        """Transmits raw data to RTT target."""
        self._socket.send(data)


class CalldwellRTTClient(RTTClient):
    """Class providing bidirectional communication with program using Calldwell streams"""

    class StreamMarker(IntEnum):
        """Enumeration listing Calldwell stream markers"""

        START = 0xDD
        END = 0xEE

    def receive_bytes_stream(self: CalldwellRTTClient) -> bytes:
        """Receives data via Calldwell stream from RTT target"""
        stream_data = self._extract_stream_data_from_recv_buffer()
        while stream_data is None:
            self._receive()
            stream_data = self._extract_stream_data_from_recv_buffer()

        return stream_data

    def transmit_bytes_stream(self: CalldwellRTTClient, data: bytes) -> None:
        """Transmits data via Calldwell stream to RTT target"""
        self._transmit_stream_marker(CalldwellRTTClient.StreamMarker.START)
        self._transmit(data)
        self._transmit_stream_marker(CalldwellRTTClient.StreamMarker.END)

    def receive_string_stream(self: CalldwellRTTClient) -> str:
        """Receives an UTF-8 string via Calldwell stream from RTT target"""
        return self.receive_bytes_stream().decode("utf-8")

    def transmit_string_stream(self: CalldwellRTTClient, message: str) -> None:
        """Transmits an UTF-8 string via Calldwell stream to RTT target"""
        self.transmit_bytes_stream(message.encode("utf-8"))

    def _extract_stream_data_from_recv_buffer(self: CalldwellRTTClient) -> bytes | None:
        """Looks for valid Calldwell stream in reception buffer, and returns it's data if found"""
        start_marker_index = self._data_buffer.find(CalldwellRTTClient.StreamMarker.START)
        if start_marker_index == -1:
            return None

        end_marker_index = self._data_buffer.find(
            CalldwellRTTClient.StreamMarker.END,
            start_marker_index,
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

    def _transmit_stream_marker(self: CalldwellRTTClient, marker: StreamMarker) -> None:
        # byteorder doesn't matter, but mypy asks for it
        self._transmit(marker.to_bytes(length=1, signed=False, byteorder="big"))
