import logging

from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger

TEST_NAME = "test-hal-watchdog"


def main():
    gdb, rtt, ssh = init_test(TEST_NAME)

    wait_for_messages(
        rtt,
        ssh,
        [
            "short task started",
            "short task ended",
            "long task started",
        ],
    )

    logging.info("Expecting a watchdog-induced MCU reset now...")
    # Default watchdog timeout is 16s. Watchdog in this test is set to 5s, but timeout must be
    # few seconds higher to compensate for communication delays and MCU clock inaccuracies.
    gdb.wait_for_reset(timeout=10)
    logging.info("Watchdog-induced reset detected!")

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
