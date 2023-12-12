"""This is an example test suite that can be used for debugging or as template."""

from __future__ import annotations

from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger

TEST_NAME = "test-hal-fpu"


def main() -> None:
    """Main function of integration test."""
    _, rtt, ssh = init_test(TEST_NAME)

    wait_for_messages(
        rtt,
        ssh,
        [
            "Performing FPU tests...",
            "Enabling FPU...",
            "FPU enabled!",
            "Lazy stacking is enabled by default!",
            "Default context configuration test successful!",
            "FPU context configuration test successful!",
            "FPU state checking test successful!",
            "All FPU tests done!",
        ],
    )

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
