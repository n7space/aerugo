"""This is an example test suite that can be used for debugging or as template."""

from __future__ import annotations

from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger

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

    wait_for_messages(
        rtt,
        ssh,
        [
            "All UART functional tests finished successfully.",
            "All UART tests finished successfully.",
        ],
    )

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
