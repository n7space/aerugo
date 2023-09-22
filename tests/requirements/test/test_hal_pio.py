"""HAL PMC driver integration test."""

from __future__ import annotations

from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger

TEST_NAME = "test-hal-pio"


def main() -> None:
    """Main function of integration test."""
    _, rtt, ssh = init_test(TEST_NAME)

    wait_for_messages(
        rtt,
        ssh,
        [
            "basic pin functions test successful",
            "pull resistors config test successful",
            "synchronous pin access test successful",
            "push-pull/open-drain config test successful",
            "all tests finished successfully",
        ],
    )

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
