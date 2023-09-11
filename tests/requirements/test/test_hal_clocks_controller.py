import logging
from typing import Tuple

from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger
from calldwell.rtt_client import CalldwellRTTClient

TEST_NAME = "test-hal-clocks-controller"


def is_main_rc_frequency_valid(
    rtt: CalldwellRTTClient, expected_frequency_mhz: int
) -> Tuple[bool, int]:
    measured_frequency_raw = rtt.receive_string_stream()
    logging.info(
        f"Received measured internal RC oscillator frequency [Hz]: {measured_frequency_raw}"
    )

    measured_frequency = int(measured_frequency_raw)
    expected_frequency_hz = expected_frequency_mhz * 1_000_000
    min_frequency = expected_frequency_hz - 1_000_000
    max_frequency = expected_frequency_hz + 1_000_000

    return (min_frequency < measured_frequency < max_frequency), measured_frequency


def main():
    gdb, rtt, ssh = init_test(TEST_NAME)

    wait_for_messages(
        rtt,
        ssh,
        [
            "status test successful",
            "interrupts config test successful",
        ],
    )

    # Now main RC frequency test will proceed
    try:
        for frequency in [12, 8, 4, 12]:
            is_valid, measured_frequency = is_main_rc_frequency_valid(rtt, frequency)
            if not is_valid:
                logging.critical(
                    "TEST FAILED, measured frequency is outside of 'expected frequency"
                    f"+/- 1MHz' range! Expected {frequency}MHz, got {measured_frequency}"
                )
    except ValueError:
        # Catching ValueError means that the test panicked at some point.
        logging.critical("TEST FAILED, APPLICATION PANICKED")

    wait_for_messages(
        rtt,
        ssh,
        [
            "master clock test successful",
            "processor clock status test successful",
            "programmable clocks config test successful",
            "peripheral clocks config test successful",
            "all tests finished successfully",
        ],
    )

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
