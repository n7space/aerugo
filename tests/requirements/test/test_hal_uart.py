"""This is an example test suite that can be used for debugging or as template."""

from __future__ import annotations

import logging
import sys
from typing import TYPE_CHECKING

from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger
from calldwell.uart import RemoteUARTConfig, RemoteUARTConnection
from scripts.env import (
    BOARD_UART_BAUDRATE,
    BOARD_UART_DEVICE,
    BOARD_UART_PORT,
)

if TYPE_CHECKING:
    from calldwell.rtt_client import CalldwellRTTClient
    from calldwell.ssh_client import SSHClient

TEST_NAME = "test-hal-uart"


def main() -> None:
    """Main function of integration test."""
    _, rtt, ssh = init_test(TEST_NAME)

    wait_for_messages(
        rtt,
        ssh,
        [
            "UART baudrate and divider calculation test successful.",
            "UART `Config` creation test successful.",
            "UART `Config` methods test successful.",
            "All UART `Config` and calcs tests successful.",
            "UART configuration test finished successfully.",
            "UART state transition test finished successfully.",
            "UART Reader/Writer test finished successfully.",
            "UART local loopback test finished successfully.",
        ],
    )

    # Open UART connection and perform two-way communication tests if all on-board tests have passed
    # successfully
    uart_config = RemoteUARTConfig(
        device_path=BOARD_UART_DEVICE,
        port=BOARD_UART_PORT,
        baudrate=BOARD_UART_BAUDRATE,
    )
    uart = RemoteUARTConnection(ssh, uart_config)
    if not uart.open_uart():
        logging.critical("TEST FAILED, could not establish UART connection!")
        sys.exit(100)

    perform_uart_io_tests(uart, rtt, ssh, 1024)

    wait_for_messages(
        rtt,
        ssh,
        [
            "All UART functional tests finished successfully.",
            "All UART tests finished successfully.",
        ],
    )

    uart.close_uart()
    finish_test(ssh)


def perform_uart_io_tests(
    uart: RemoteUARTConnection,
    rtt: CalldwellRTTClient,
    ssh: SSHClient,
    data_length: int,
) -> None:
    """Tests UART I/O using UART interface that's connected to target board"""
    # Perform a simple handshake to make sure that the connection is established correctly
    # (baudrates are OK, etc.)
    perform_uart_handshake(uart, rtt, ssh)

    # Reception test (test host -> MCU) with large chunk of data
    uart.write_bytes(bytes([i % 0x100 for i in range(data_length)]))
    wait_for_messages(rtt, ssh, ["Data reception test successful!", "Test data transmitted!"])

    # Transmission test (MCU -> test host) with large chunk of data
    reception_result = uart.read_exact_bytes(data_length)
    if reception_result.is_err:
        logging.critical("TEST FAILED, could not receive all the data from target board!")
        sys.exit(50)

    received_data = reception_result.unwrap()
    for byte, i in zip(received_data, range(data_length)):
        if byte != (expected := i % 0x100):
            logging.critical(
                f"TEST FAILED, unexpected byte @ index {i}, expected {expected}, got {byte}",
            )
            sys.exit(60)

    logging.info("Data transmission test successful!")
    logging.info("All I/O test successful!")


def perform_uart_handshake(
    uart: RemoteUARTConnection,
    rtt: CalldwellRTTClient,
    ssh: SSHClient,
) -> None:
    """Performs a simple UART handshake."""
    handshake_message = b"hello, this is the test suite handshake message!"
    expected_response = "hello back, this is Aerugo's handshake message!"

    wait_for_messages(rtt, ssh, ["Waiting for handshake..."])

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

    wait_for_messages(rtt, ssh, ["Handshake message received, responding..."])

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

    wait_for_messages(rtt, ssh, ["Handshake done!"])

    logging.info("UART handshake successful!")


if __name__ == "__main__":
    init_default_logger()
    main()
