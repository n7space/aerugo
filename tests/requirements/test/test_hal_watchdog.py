import logging

from test_utils import finish_test, init_test, setup_logger

TEST_NAME = "test-hal-watchdog"


def main():
    gdb, rtt, ssh = init_test(TEST_NAME)

    expected_messages = [
        "short task started",
        "short task ended",
        "long task started",
    ]

    for message in expected_messages:
        received_message = rtt.receive_string_stream()
        logging.info(f"RTT> {received_message}")
        if received_message != message:
            logging.critical(
                "TEST FAILED: UNEXPECTED MESSAGE RECEIVED "
                f"(expecting {message}, got {received_message})"
            )
            finish_test(ssh)
            exit(2)

    logging.info("Expecting a watchdog-induced MCU reset now...")
    # Default watchdog timeout is 16s. Watchdog in this test is set to 3s, but timeout must be
    # few seconds higher to compensate for communication delays and MCU clock inaccuracies.
    gdb.wait_for_reset(timeout=10)
    logging.info("Watchdog-induced reset detected!")

    finish_test(ssh)


if __name__ == "__main__":
    setup_logger()
    main()
