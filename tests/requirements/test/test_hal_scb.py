"""HAL SCB driver integration test."""

from __future__ import annotations

import logging
import sys

from test_utils import finish_test, init_test, wait_for_messages, wait_for_uart_messages

from calldwell import init_default_logger
from calldwell.uart import RemoteUARTConfig, RemoteUARTConnection
from scripts.env import (
    BOARD_UART_DEVICE,
    BOARD_UART_PORT,
)

TEST_NAME = "test-hal-scb"


def main() -> None:
    """Main function of integration test."""
    _, rtt, ssh = init_test(TEST_NAME)

    # Open UART connection and perform a handshake with test app
    uart_config = RemoteUARTConfig(
        device_path=BOARD_UART_DEVICE,
        port=BOARD_UART_PORT,
        baudrate=57600,
    )
    uart = RemoteUARTConnection(ssh, uart_config)
    if not uart.open_uart():
        logging.critical("TEST FAILED, could not establish UART connection!")
        sys.exit(1000)

    perform_uart_handshake(uart)

    wait_for_messages(
        rtt,
        ssh,
        [
            "I-Cache management test successful",
        ],
    )

    wait_for_uart_messages(
        uart,
        ssh,
        [
            "D-Cache management test successful",
            "All SCB tests finished successfully!",
        ],
    )

    uart.close_uart()
    finish_test(ssh)


def perform_uart_handshake(uart: RemoteUARTConnection) -> None:
    """Performs simple UART handshake with test app"""
    handshake_message = b"Hello!"
    expected_response = "World!"

    logging.info("Performing UART handshake...")
    written_bytes, tx_error = uart.write_bytes(handshake_message)

    if tx_error is not None:
        logging.critical(
            f"TEST FAILED, an error occurred while sending handshake message: {tx_error}",
        )
        sys.exit(10)

    if written_bytes != len(handshake_message):
        logging.critical(
            f"TEST FAILED, handshake message is not fully transmitted (sent {written_bytes} out of "
            f"{len(handshake_message)} bytes)",
        )
        sys.exit(20)

    response = uart.read_string(b"!")

    if response.is_err:
        logging.critical(
            f"TEST FAILED, an error occurred while receiving handshake response: "
            f"{response.unwrap_err()}",
        )
        sys.exit(30)

    if (response_message := response.unwrap()) != expected_response:
        logging.critical(
            f"TEST FAILED, invalid handshake response\nexpected '{expected_response}'\ngot "
            f"'{response_message}'",
        )
        sys.exit(40)

    logging.info("UART handshake successful!")


if __name__ == "__main__":
    init_default_logger()
    main()
