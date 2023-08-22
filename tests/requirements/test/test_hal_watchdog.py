import logging

from test_utils import finish_test, init_test

TEST_NAME = "test-hal-watchdog"


def main():
    gdb, rtt, ssh = init_test(TEST_NAME)

    expected_messages = [
        "short task started",
        "short task ended",
        "long task started",
    ]

    for message in expected_messages:
        received_message = rtt.receive_bytes_stream().decode()
        print(received_message)
        if received_message != message:
            print(
                "TEST FAILED: UNEXPECTED MESSAGE RECEIVED "
                f"(expecting {message}, got {received_message})"
            )
            finish_test(ssh)
            exit(2)

    # Default watchdog timeout is 16s. Watchdog in this test is set to 3s, but timeout must be
    # few seconds higher to compensate for communication delays and MCU clock inaccuracies.
    gdb.wait_for_reset(timeout=10)

    finish_test(ssh)


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    main()
