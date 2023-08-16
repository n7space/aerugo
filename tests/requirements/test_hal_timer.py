import os

import logging
from typing import List, Tuple
from calldwell.gdb_client import GDBClient
from calldwell.ssh_client import SSHClient
from calldwell.rtt_client import RTTClient

BOARD_LOGIN = str(os.environ.get("AERUGO_BOARD_LOGIN"))
BOARD_PASSWORD = str(os.environ.get("AERUGO_BOARD_PASSWORD"))
BOARD_HOSTNAME = str(os.environ.get("AERUGO_BOARD_HOSTNAME"))
BOARD_GDB_PORT = str(os.environ.get("AERUGO_BOARD_GDB_PORT"))
BOARD_RTT_PORT = str(os.environ.get("AERUGO_BOARD_RTT_PORT"))
GDB_EXECUTABLE = "arm-none-eabi-gdb"
TEST_BINARY_PATH = (
    "./testbins/test-hal-timer/target/thumbv7em-none-eabihf/debug/test-hal-timer"
)


def init_test() -> Tuple[GDBClient, RTTClient, SSHClient]:
    ssh = SSHClient(BOARD_HOSTNAME, BOARD_LOGIN, BOARD_PASSWORD)
    ssh.execute("./setup_debugging_sam_clean.sh")

    gdb = GDBClient(GDB_EXECUTABLE, log_responses=False, log_execution=False)
    if not gdb.connect_to_remote(f"{BOARD_HOSTNAME}:{BOARD_GDB_PORT}"):
        print(f"Could not connect to board @ {BOARD_HOSTNAME:{BOARD_GDB_PORT}}")
        exit(1)
    gdb.start_rtt_server(int(BOARD_RTT_PORT), 0)

    rtt = RTTClient(BOARD_HOSTNAME, port=int(BOARD_RTT_PORT))

    gdb.load_executable(TEST_BINARY_PATH)
    rtt_symbol = gdb.get_variable("_SEGGER_RTT")
    if rtt_symbol is None:
        print("COULD NOT FIND RTT SECTION!")
        exit(1)
    gdb.start_program()
    gdb.setup_rtt(rtt_symbol.address, 0x400, "SEGGER RTT")
    gdb.set_breakpoint("calldwell::initialize")
    gdb.continue_program()
    gdb.wait_for_breakpoint_hit()
    gdb.finish_function_execution()
    gdb.start_rtt()
    gdb.continue_program()

    init_message = rtt.receive_stream().decode()
    if init_message != "mcu ready":
        print("TEST FAILED: MCU NOT READY")
        exit(1)

    rtt.transmit_stream("ok".encode())

    return gdb, rtt, ssh


def finish_test(ssh: SSHClient):
    ssh.execute("pkill openocd")
    ssh.close()


def average_difference(values: List[int]) -> float:
    diffs = [j - i for i, j in zip(values[:-1], values[1:])]
    return sum(diffs) / len(diffs)


def main():
    _, rtt, ssh = init_test()

    # Timer should be running by default, and program should output
    # it's overflows via RTT.

    # First 10 messages should contain fast-changing timer IRQ count
    fast_irq_counts: List[int] = list()
    for _ in range(10):
        fast_irq_counts.append(int(rtt.receive_stream().decode()))
    avg_diffs_fast = average_difference(fast_irq_counts)

    # After 10 messages, tasklet should disable the timer, so incoming IRQ counts
    # should not change
    stopped_irq_counts: List[int] = list()
    for _ in range(10):
        stopped_irq_counts.append(int(rtt.receive_stream().decode()))
    avg_diffs_stopped = average_difference(stopped_irq_counts)

    # After another 10 messages, tasklet should switch timer's source to slower one
    # and enable it, returning IRQ count that's changing slower
    slow_irq_counts: List[int] = list()
    for _ in range(10):
        slow_irq_counts.append(int(rtt.receive_stream().decode()))

    avg_diffs_slow = average_difference(slow_irq_counts)

    if avg_diffs_fast <= avg_diffs_slow:
        print("TEST FAILED, FASTER CLOCK IS IN FACT SLOWER")
        exit(2)

    if avg_diffs_stopped != 0:
        print("TEST FAILED, CLOCK DID NOT STOP")

    finish_test(ssh)


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    main()
