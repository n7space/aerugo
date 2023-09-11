"""This script can be used to capture raw RTT traffic coming from target board.

Usage:
Pass the hostname and port (in `hostname:port` format, for example 192.168.1.1:1234)
as an argument to this script. It will automatically connect to RTT server on the other end,
and start receiving data and printing it to stdout.
"""
import sys
from typing import Tuple

from calldwell.rtt_client import RTTClient


def get_args() -> Tuple[str, int]:
    if len(sys.argv) != 2:
        print("RTT client script, connects to RTT server and prints incoming data on stdout")
        print(f"Usage: {sys.argv[0]} hostname:port")
        exit(1)

    full_hostname = sys.argv[1].split(":")

    if len(full_hostname) != 2:
        print("Invalid hostname")
        exit(2)

    host, port = full_hostname
    return host, int(port)


def listen_to_rtt(host: str, port: int):
    rtt = RTTClient(host, port)

    while True:
        try:
            print(rtt.receive_string(), end="")
        except KeyboardInterrupt:
            exit(0)


def main():
    host, port = get_args()
    listen_to_rtt(host, port)


if __name__ == "__main__":
    main()
