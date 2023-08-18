import logging
from test_utils import *

TEST_BINARY_PATH = (
    "./testbins/test-hal-watchdog/target/thumbv7em-none-eabihf/debug/test-hal-watchdog"
)


def main():
    gdb, rtt, ssh = init_test(TEST_BINARY_PATH)

    expected_messages = [
        "short task started",
        "short task ended",
        "long task started",
    ]

    for message in expected_messages:
        received_message = rtt.receive_bytes().decode()
        print(received_message)
        if received_message != message:
            print(
                f"TEST FAILED: UNEXPECTED MESSAGE RECEIVED (expecting {message}, got {received_message})"
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
