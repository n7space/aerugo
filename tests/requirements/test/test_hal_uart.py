"""This is an example test suite that can be used for debugging or as template."""

from __future__ import annotations

import logging
import sys
from typing import TYPE_CHECKING

from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger
from calldwell.uart import RemoteUARTConfig, RemoteUARTConnection
from scripts.env import BOARD_UART_BAUDRATE, BOARD_UART_DEVICE, BOARD_UART_PORT

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

    perform_uart_io_tests(uart, rtt, ssh)

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
) -> None:
    """Tests UART I/O using UART interface that's connected to target board"""
    # Perform a simple handshake to make sure that the connection is established correctly
    # (baudrates are OK, etc.)
    perform_uart_handshake(uart)
    wait_for_messages(rtt, ssh, ["UART handshake done!"])


def perform_uart_handshake(uart: RemoteUARTConnection) -> None:
    """Performs a simple UART handshake."""
    handshake_message = b"hello, this is the test suite handshake message!"
    expected_response = "hello back, this is Aerugo's handshake message!"
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
