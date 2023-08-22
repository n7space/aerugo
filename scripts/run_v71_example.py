import logging
import sys
from enum import IntEnum
from pathlib import Path
from typing import Tuple

from tests.requirements.test.test_utils import (
    BOARD_GDB_PORT,
    BOARD_HOSTNAME,
    BOARD_LOGIN,
    BOARD_PASSWORD,
    BOARD_RTT_PORT,
    GDB_EXECUTABLE,
    build_cargo_app,
)

from calldwell.gdb_client import GDBClient
from calldwell.rtt_client import RTTClient
from calldwell.rust_helpers import (
    RTT_SECTION_ID,
    RTT_SECTION_SEARCHED_MEMORY_LENGTH,
    RTT_SECTION_SYMBOL_NAME,
)
from calldwell.ssh_client import SSHClient

# This script should be run from project's root dir, not from `scripts/`!
EXAMPLES_DIR_PATH = Path("./examples")
RTT_INIT_FUNCTION_NAME = "init_rtt"


class ExitReason(IntEnum):
    INVALID_ARGUMENTS = 1
    INVALID_EXAMPLE_PATH = 2
    CARGO_NOT_FOUND = 3
    CANNOT_CONNECT_TO_REMOTE_GDB = 4
    CANNOT_START_RTT_SERVER = 5
    EXAMPLE_NOT_BUILT = 6
    CANNOT_LOAD_EXECUTABLE = 7
    CANNOT_RUN_EXECUTABLE = 8
    CANNOT_SET_BREAKPOINT_AT_RTT_INIT = 9
    UNEXPECTED_STOP = 10
    COULD_NOT_FIND_RTT_SYMBOL = 11
    COULD_NOT_SETUP_RTT = 12
    COULD_NOT_START_RTT = 13


def get_args() -> str:
    if len(sys.argv) != 2:
        print(
            "SAMV71 example runner, starts remote GDB session, configures RTT, "
            "runs example and prints it's output on stdout"
        )
        print(f"Usage: {sys.argv[0]} example-name")
        exit(ExitReason.INVALID_ARGUMENTS)

    example_name = sys.argv[1]
    return example_name


def get_example_path(example_name: str) -> Path:
    return EXAMPLES_DIR_PATH / example_name


def start_gdb() -> Tuple[GDBClient, SSHClient]:
    ssh = SSHClient(BOARD_HOSTNAME, BOARD_LOGIN, BOARD_PASSWORD)
    ssh.execute("./setup_debugging_sam_clean.sh")

    gdb = GDBClient(GDB_EXECUTABLE, log_responses=False, log_execution=False)
    remote_gdb_hostname = f"{BOARD_HOSTNAME}:{BOARD_GDB_PORT}"
    if not gdb.connect_to_remote(remote_gdb_hostname):
        print(f"Error: Could not connect to remote GDB server @ {remote_gdb_hostname}!")
        exit(ExitReason.CANNOT_CONNECT_TO_REMOTE_GDB)

    return gdb, ssh


def load_and_start_program(gdb: GDBClient, program_path: Path):
    abs_path = program_path.absolute()
    if not program_path.exists():
        print(f"Error: trying to run non-existent program {abs_path}!")
        exit(ExitReason.EXAMPLE_NOT_BUILT)

    if not gdb.load_executable(str(abs_path)):
        print(f"Error: could not load executable {abs_path}!")
        exit(ExitReason.CANNOT_LOAD_EXECUTABLE)

    if not gdb.start_program():
        print(f"Error: could not start execution of executable {abs_path}!")
        exit(ExitReason.CANNOT_RUN_EXECUTABLE)


def wait_for_rtt_init(gdb: GDBClient, example_name: str):
    rtt_init_function_full_name = (
        f"{example_name.replace('-', '_')}::{RTT_INIT_FUNCTION_NAME}"
    )

    if not gdb.set_breakpoint(rtt_init_function_full_name):
        print(
            f"Error: Cannot set the breakpoint on RTT init function {rtt_init_function_full_name}!"
        )
        exit(ExitReason.CANNOT_SET_BREAKPOINT_AT_RTT_INIT)

    gdb.continue_program()

    if not gdb.wait_for_breakpoint_hit():
        print(f"Error: Program has stopped, but not because of a breakpoint!")
        exit(ExitReason.UNEXPECTED_STOP)

    gdb.finish_function_execution()


def setup_rtt(gdb: GDBClient) -> RTTClient:
    rtt_symbol = gdb.get_variable(RTT_SECTION_SYMBOL_NAME)
    if rtt_symbol is None:
        print(
            f"Error: Could not find RTT symbol '{RTT_SECTION_SYMBOL_NAME}' in the binary!"
        )
        exit(ExitReason.COULD_NOT_FIND_RTT_SYMBOL)

    if not gdb.start_rtt_server(BOARD_RTT_PORT, 0):
        print(f"Error: Could not start RTT server @ {BOARD_RTT_PORT}!")
        exit(ExitReason.CANNOT_START_RTT_SERVER)

    if not gdb.setup_rtt(
        rtt_symbol.address, RTT_SECTION_SEARCHED_MEMORY_LENGTH, RTT_SECTION_ID
    ):
        print(
            f"Could not setup RTT for section @ {rtt_symbol.address} "
            f"searched {RTT_SECTION_SEARCHED_MEMORY_LENGTH} bytes)"
        )
        exit(ExitReason.COULD_NOT_SETUP_RTT)

    if not gdb.start_rtt():
        print("Could not start RTT!")
        exit(ExitReason.COULD_NOT_START_RTT)

    return RTTClient(BOARD_HOSTNAME, BOARD_RTT_PORT)


def main():
    example_name = get_args()
    example_path = get_example_path(example_name)
    if not example_path.exists():
        print(
            f"Error: example '{example_path.absolute()}' does not exist!\n"
            "Make sure you're running this script from root directory of the project, "
            "and that the example name is correct!"
        )
        exit(ExitReason.INVALID_EXAMPLE_PATH)

    program_path = build_cargo_app(example_path, release_build=True)
    if program_path is None:
        print(f"Error: Cargo executable not found!")
        exit(ExitReason.CARGO_NOT_FOUND)

    gdb, ssh = start_gdb()
    load_and_start_program(gdb, program_path)
    wait_for_rtt_init(gdb, example_name)
    rtt = setup_rtt(gdb)

    gdb.continue_program()

    while True:
        try:
            print(rtt.receive_string(), end="")
        except KeyboardInterrupt:
            ssh.execute("killall openocd")
            ssh.close()
            exit(0)


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    main()
