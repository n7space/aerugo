"""Integration test for event handling from interrupt context"""


from __future__ import annotations

from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger

TEST_NAME = "test-event-interrupt"


def main() -> None:
    """Main function of integration test."""
    _, rtt, ssh = init_test(TEST_NAME)

    wait_for_messages(
        rtt,
        ssh,
        ["TaskA got 42"],
    )

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
