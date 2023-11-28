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

    logging.info("Sending opcode 0x00")
    uart.write_bytes(bytes([0x10, 0x00, 0xC0, 0x00, 0x00, 0x01, 0x00]))
    sleep(0.1)
    logging.info("Sending opcode 0x10")
    uart.write_bytes(bytes([0x10, 0x00, 0xC0, 0x10, 0x00, 0x01, 0x00]))
    sleep(0.1)
    logging.info("Sending opcode 0x20")
    uart.write_bytes(bytes([0x10, 0x00, 0xC0, 0x20, 0x00, 0x01, 0x00]))
    sleep(0.1)
    logging.info("Sending opcode 0x30")
    uart.write_bytes(bytes([0x10, 0x00, 0xC0, 0x30, 0x00, 0x01, 0x08]))
    sleep(0.1)
    logging.info("Sending opcode 0x40")
    uart.write_bytes(bytes([0x10, 0x00, 0xC0, 0x40, 0x00, 0x01, 0xBE]))
    sleep(0.1)
    logging.info("Sending opcode 0x50")
    uart.write_bytes(bytes([0x10, 0x00, 0xC0, 0x50, 0x00, 0x01, 0xEF]))
    sleep(0.1)
    logging.info("Sending opcode 0x60")
    uart.write_bytes(bytes([0x10, 0x00, 0xC0, 0x60, 0x00, 0x01, 0x00]))
    logging.info("All done!")


if __name__ == "__main__":
    init_default_logger()
    main()
