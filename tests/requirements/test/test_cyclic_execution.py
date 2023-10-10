"""Integration test for cyclic execution"""


from __future__ import annotations

from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger

TEST_NAME = "test-cyclic-execution"


def main() -> None:
    """Main function of integration test."""
    _, rtt, ssh = init_test(TEST_NAME)

    wait_for_messages(
        rtt,
        ssh,
        ["TaskA executed at 2s", "TaskA executed at 3s", "TaskA executed at 4s"],
    )

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
