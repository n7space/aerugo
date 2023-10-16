"""Script that can be used to listen for incoming UART traffic from target board.
Call without arguments (or with `--help`) to show usage.
"""
import argparse
import logging
import sys

from calldwell import init_default_logger
from calldwell.ssh_client import SSHClient
from calldwell.uart import RemoteUARTConfig, RemoteUARTConnection


def get_user_args() -> argparse.Namespace:
    """Fetch user arguments via `argparse`."""
    parser = argparse.ArgumentParser(
        description="Helper script that listens to incoming UART traffic from target board, and"
        "prints it to stdout",
        exit_on_error=True,
    )

    parser.add_argument(
        "hostname",
        type=str,
        help="Target's hostname, for example 192.168.1.1 or localhost",
    )
    parser.add_argument("login", type=str, help="SSH login of target machine")
    parser.add_argument("password", type=str, help="SSH password of target machine")
    parser.add_argument(
        "device_path",
        type=str,
        help="Path to UART device on target machine, for example /dev/ttyUSB0",
    )
    parser.add_argument(
        "baudrate",
        type=int,
        choices=[
            50,
            75,
            110,
            134,
            150,
            200,
            300,
            600,
            1200,
            1800,
            2400,
            4800,
            9600,
            19200,
            38400,
            57600,
            115200,
            230400,
            460800,
            500000,
            576000,
            921600,
            1000000,
            1152000,
            1500000,
            2000000,
            2500000,
            3000000,
            3500000,
            4000000,
        ],
        help="Target's baudrate, in bits per second. Only standard baudrates are supported.",
    )

    parser.add_argument("--port", type=int, default=19876, help="TCP port for UART connection")

    return parser.parse_args()


def main() -> None:
    """Main function."""
    args = get_user_args()
    ssh = SSHClient(args.hostname, args.login, args.password)
    uart_config = RemoteUARTConfig(
        device_path=args.device_path,
        port=args.port,
        baudrate=args.baudrate,
    )
    uart = RemoteUARTConnection(ssh, uart_config)

    if uart.open_uart():
        logging.info("UART opened, waiting for data...")
    else:
        logging.critical("UART connection couldn't be established, quitting...")
        sys.exit(1)

    while True:
        if (data := uart.read_any()) is not None:
            print(data)


if __name__ == "__main__":
    init_default_logger()
    main()
