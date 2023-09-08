from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger

TEST_NAME = "test-hal-clocks-controller"


def main():
    _, rtt, ssh = init_test(TEST_NAME)

    wait_for_messages(
        rtt,
        ssh,
        [
            "status test successful",
            "interrupts config test successful",
            "master clock test successful",
        ],
    )

    main_rc_measurement = rtt.receive_string_stream()
    print(main_rc_measurement)

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
