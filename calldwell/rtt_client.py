import socket
from typing import Optional


class RTTClient:
    """Class acting as RTT front-end. Provides bidirectional communication with debugged program."""

    def __init__(self, host: str, port: int, default_chunk_size: int = 1024):
        """Create instance of RTT client. Connects to RTT server via TCP socket.

        # Parameters
        * `host` - either a hostname or IP address of RTT server
        * `port` - port of RTT server
        * `default_chunk_size` - Chunk size used for receiving data.
        """
        self._socket = socket.socket()
        self._socket.connect((host, port))
        self._default_chunk_size = default_chunk_size

    def close(self):
        """Closes the RTT connection gracefully."""
        self._socket.shutdown(socket.SHUT_RDWR)
        self._socket.close()

    def receive(self, chunk_size: Optional[int] = None) -> bytes:
        """Receive data from RTT target"""
        if chunk_size is None:
            chunk_size = self._default_chunk_size

        return self._socket.recv(chunk_size)

    def transmit(self, data: bytes):
        """Transmit data to RTT target."""
        self._socket.send(data)
