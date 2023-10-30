"""This is an example test suite that can be used for debugging or as template."""

from __future__ import annotations

import logging

from test_utils import finish_test, init_test

from calldwell import init_default_logger

TEST_NAME = "test-hal-xdmac"


def main() -> None:
    """Main function of integration test."""
    _, rtt, ssh = init_test(TEST_NAME)

    while True:
        received_message = rtt.receive_string_stream()
        logging.info(f"RTT> {received_message}")

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
