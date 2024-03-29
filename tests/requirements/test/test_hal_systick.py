"""HAL Systick driver integration test."""

from __future__ import annotations

import logging
import sys

from test_utils import finish_test, init_test

from calldwell import init_default_logger

TEST_NAME = "test-hal-systick"


def average_difference(values: list[int]) -> float:
    """Returns average difference between values on the list."""
    diffs = [j - i for i, j in zip(values[:-1], values[1:])]
    return sum(diffs) / len(diffs)


def main() -> None:
    """Main function of integration test."""
    _, rtt, ssh = init_test(TEST_NAME)

    # Systick should be running by default, and program should output
    # it's overflow count via RTT.

    # First 10 messages should contain fast-changing timer IRQ count
    fast_irq_counts: list[int] = [int(rtt.receive_bytes_stream().decode()) for _ in range(10)]
    avg_diffs_fast = average_difference(fast_irq_counts)
    logging.info(f"Average differences (fast clock): {avg_diffs_fast:.03f}")

    # After 10 messages, tasklet should disable systick, so incoming IRQ counts
    # should not change
    stopped_irq_counts: list[int] = [int(rtt.receive_bytes_stream().decode()) for _ in range(10)]
    avg_diffs_stopped = average_difference(stopped_irq_counts)
    logging.info(f"Average differences (clock stopped): {avg_diffs_stopped:.03f}")

    # After another 10 messages, tasklet should systick timer's frequency to slower one
    # and enable it, returning IRQ count that's changing slower
    slow_irq_counts: list[int] = [int(rtt.receive_bytes_stream().decode()) for _ in range(10)]
    avg_diffs_slow = average_difference(slow_irq_counts)
    logging.info(f"Average differences (slow clock): {avg_diffs_slow:.03f}")

    if avg_diffs_fast <= avg_diffs_slow:
        logging.critical("TEST FAILED: FASTER SYSTICK IS IN FACT SLOWER")
        sys.exit(2)

    if avg_diffs_stopped != 0:
        logging.critical("TEST FAILED: SYSTICK DID NOT STOP")

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
