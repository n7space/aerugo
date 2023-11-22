"""This is an example test suite that can be used for debugging or as template."""

from __future__ import annotations

from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger

TEST_NAME = "test-hal-spi"


def main() -> None:
    """Main function of integration test."""
    _, rtt, ssh = init_test(TEST_NAME)

    wait_for_messages(
        rtt,
        ssh,
        [
            "Performing SPI tests...",
            "Beginning SPI configuration tests...",
            "SPI master mode config test started.",
            "SPI master mode config test finished successfully!",
            "SPI status reader test started.",
            "SPI status reader test finished successfully!",
            "SPI interrupt config test started.",
            "SPI interrupt config test finished successfully!",
            "SPI chip config test started.",
            "SPI chip config test finished successfully!",
            "SPI loopback config test started.",
            "SPI loopback config test finished successfully!",
            "SPI reader/writer taking test started.",
            "SPI reader/writer taking test finished successfully!",
            "All SPI configuration tests done!",
            "Beginning SPI communication tests...",
            "Synchronous transfer test started.",
            "Synchronous 8-bit transfer successful!",
            "Synchronous 12-bit transfer successful!",
            "Synchronous transfer test successfully finished!",
            "Interrupt transfer test started.",
            "Interrupt 8-bit transfer successful!",
            "Interrupt 12-bit transfer successful!",
            "Interrupt transfer test successfully finished!",
            "All SPI communication tests done!",
            "Beginning SPI communication tests via embedded-hal traits...",
            "All SPI communication tests via embedded-hal traits done!",
            "Beginning LSM6DSO communication tests w/ XDMAC...",
            "All LSM6DSO communication tests w/ XDMAC done!",
            "All SPI tests done!",
        ],
    )

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
