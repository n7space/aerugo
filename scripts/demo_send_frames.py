"""Helper script for Aerugo SPI accelerometer demonstration application that sends testing frames"""
import logging
import sys
from time import sleep

from calldwell import init_default_logger
from calldwell.ssh_client import SSHClient
from calldwell.uart import RemoteUARTConfig, RemoteUARTConnection
from scripts.env import (
    BOARD_LOGIN,
    BOARD_NETWORK_PATH,
    BOARD_PASSWORD,
    BOARD_UART_DEVICE,
    BOARD_UART_PORT,
)

ECHO_EXAMPLE_BAUDRATE = 57600


def main() -> None:
    """Main function."""
    ssh = SSHClient(BOARD_NETWORK_PATH, BOARD_LOGIN, BOARD_PASSWORD)
    uart_config = RemoteUARTConfig(
        device_path=BOARD_UART_DEVICE,
        port=BOARD_UART_PORT,
        baudrate=ECHO_EXAMPLE_BAUDRATE,
    )
    uart = RemoteUARTConnection(ssh, uart_config)

    if uart.open_uart():
        logging.info("UART opened")
    else:
        logging.critical("UART connection couldn't be established, quitting...")
        sys.exit(1)

    logging.info("Sending malformed packet")
    uart.write_bytes(bytes([0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]))
    logging.info("Malformed packet sent, waiting for response")
    response = uart.read_exact_bytes(12).unwrap()
    logging.info(f"Response: {response.hex(' ').upper()}")

    for opcode in range(0x00, 0x80, 0x10):
        logging.info(f"Sending command {opcode:02X}")
        uart.write_bytes(bytes([0x10, 0x00, 0xC0, opcode, 0x00, 0x01, 0x00]))
        logging.info("Command sent, waiting for response")
        response = uart.read_exact_bytes(12).unwrap()
        logging.info(f"Response: {response.hex(' ').upper()}")
        sleep(0.5)


if __name__ == "__main__":
    init_default_logger()
    main()
