import logging
from typing import List

from test_utils import finish_test, init_test

TEST_NAME = "test-hal-timer"


def average_difference(values: List[int]) -> float:
    diffs = [j - i for i, j in zip(values[:-1], values[1:])]
    return sum(diffs) / len(diffs)


def main():
    _, rtt, ssh = init_test(TEST_NAME)

    # Timer should be running by default, and program should output
    # it's overflow count via RTT.

    # First 10 messages should contain fast-changing timer IRQ count
    fast_irq_counts: List[int] = list()
    for _ in range(10):
        fast_irq_counts.append(int(rtt.receive_bytes_stream().decode()))
    avg_diffs_fast = average_difference(fast_irq_counts)
    print(f"Average differences (fast clock): {avg_diffs_fast:.03f}")

    # After 10 messages, tasklet should disable the timer, so incoming IRQ counts
    # should not change
    stopped_irq_counts: List[int] = list()
    for _ in range(10):
        stopped_irq_counts.append(int(rtt.receive_bytes_stream().decode()))
    avg_diffs_stopped = average_difference(stopped_irq_counts)
    print(f"Average differences (clock stopped): {avg_diffs_stopped:.03f}")

    # After another 10 messages, tasklet should switch timer's source to slower one
    # and enable it, returning IRQ count that's changing slower
    slow_irq_counts: List[int] = list()
    for _ in range(10):
        slow_irq_counts.append(int(rtt.receive_bytes_stream().decode()))

    avg_diffs_slow = average_difference(slow_irq_counts)
    print(f"Average differences (slow clock): {avg_diffs_slow:.03f}")

    if avg_diffs_fast <= avg_diffs_slow:
        print("TEST FAILED: FASTER CLOCK IS IN FACT SLOWER")
        exit(2)

    if avg_diffs_stopped != 0:
        print("TEST FAILED: CLOCK DID NOT STOP")

    finish_test(ssh)


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    main()
