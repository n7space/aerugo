"""Script that can be used to capture Calldwell's RTT traffic coming from target board.

Usage:
Pass the hostname and port (in `hostname:port` format, for example 192.168.1.1:1234)
as an argument to this script. It will automatically connect to RTT server on the other
end, perform Calldwell's handshake, and start receiving data and printing it to stdout.

Useful for debugging integration tests.
"""

from __future__ import annotations

import sys
from typing import NoReturn

from calldwell import init_default_logger
from calldwell.rtt_client import CalldwellRTTClient
from calldwell.rust_helpers import perform_calldwell_rs_handshake


def get_args() -> tuple[str, int]:
    """Parses and returns script's arguments, exist the program with non-zero exit code
    if arguments are missing or cannot be parsed."""
    if len(sys.argv) != 2:  # noqa: PLR2004 (magic number self-explanatory)
        print("RTT client script")
        print("Connects to RTT server and prints incoming data on stdout")
        print(f"Usage: {sys.argv[0]} hostname:port")
        sys.exit(1)

    full_hostname = sys.argv[1].split(":")

    if len(full_hostname) != 2:  # noqa: PLR2004 (format described in help message)
        print("Invalid hostname")
        sys.exit(2)

    host, port = full_hostname
    return host, int(port)


def listen_to_rtt(host: str, port: int) -> NoReturn:
    """Starts listening to RTT server and prints all received data on stdout."""
    rtt = CalldwellRTTClient(host, port)

    if not perform_calldwell_rs_handshake(rtt):
        print("Couldn't perform Calldwell's RTT handshake")
        sys.exit(0xDEAD)

    while True:
        try:
            print(rtt.receive_string_stream(), end="")
        except KeyboardInterrupt:  # noqa: PERF203 (we don't care about performance here)
            sys.exit(0)


def main() -> NoReturn:
    """Main function of this script."""
    host, port = get_args()
    listen_to_rtt(host, port)


if __name__ == "__main__":
    init_default_logger()
    main()
