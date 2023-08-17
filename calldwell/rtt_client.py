"""Module containing RTT-related classes, which also provide an easy-to-use layer of
abstraction over Calldwell streams/messages."""

import socket
from enum import IntEnum
from typing import Optional


class RTTClient:
    """Class acting as RTT front-end. Provides bidirectional communication with debugged program."""

    class StreamMarker(IntEnum):
        """Enumeration listing Calldwell stream markers"""

        Start = 0xDD
        End = 0xEE

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

    def receive_stream(self) -> bytes:
        """Receives data via Calldwell stream from RTT target"""
        stream_data = self._extract_stream_data_from_recv_buffer()
        while stream_data is None:
            self._receive()
            stream_data = self._extract_stream_data_from_recv_buffer()

        return stream_data

    def transmit_stream(self, data: bytes) -> None:
        """Transmits data via Calldwell stream to RTT target"""
        self._transmit_stream_marker(RTTClient.StreamMarker.Start)
        self._transmit(data)
        self._transmit_stream_marker(RTTClient.StreamMarker.End)

    def _extract_stream_data_from_recv_buffer(self) -> Optional[bytes]:
        """Looks for valid Calldwell stream in reception buffer, and returns it's data if found"""
        start_marker_index = self._data_buffer.find(RTTClient.StreamMarker.Start)
        if start_marker_index == -1:
            return None

        end_marker_index = self._data_buffer.find(RTTClient.StreamMarker.End, start_marker_index)
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
        self._transmit(marker.to_bytes(length=1, signed=False))

    def _receive(self, chunk_size: Optional[int] = None) -> None:
        """Receives raw data from RTT target to internal buffer"""
        if chunk_size is None:
            chunk_size = self._default_chunk_size

        received_bytes = self._socket.recv(chunk_size)
        self._data_buffer.extend(received_bytes)

    def _transmit(self, data: bytes) -> None:
        """Transmits raw data to RTT target."""
        self._socket.send(data)
