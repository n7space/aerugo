"""Script that can be used to run Aerugo demonstation application on remote SAMV71 board
connected via OpenOCD probe.

Usage:
Run this script from Aerugo's root directory. Pass the directory name of the demo as
an argument (for example, `samv71-spi-accelerometer`). This should be a preferred way of
running the demos, or should be treated as template for manual execution of any
Aerugo/Calldwell applications.

This script performs following steps:
1. Verify that the demo actually exists
2. Verify that Cargo is installed and project is built successfully (will automatically
   build it if necessary)
3. Start remote GDB server via SSH using credentials from `test_utils` module. See
   `tests/requirements/tests/tests_utils.py` for details.
4. Start RTT server, flash the application and perform the startup process
5. Wait for RTT to be initialized on MCU side and start RTT facilities
6. Execute the rest of demo's code
"""

from __future__ import annotations

import sys
from enum import IntEnum
from typing import TYPE_CHECKING, NoReturn

from calldwell import init_default_logger
from calldwell.gdb_client import GDBClient
from calldwell.rtt_client import RTTClient
from calldwell.rust_helpers import (
    RTT_SECTION_ID,
    RTT_SECTION_SEARCHED_MEMORY_LENGTH,
    RTT_SECTION_SYMBOL_NAME,
    build_cargo_app,
)
from calldwell.ssh_client import SSHClient
from scripts.env import (
    BOARD_GDB_PORT,
    BOARD_LOGIN,
    BOARD_NETWORK_PATH,
    BOARD_PASSWORD,
    BOARD_RTT_PORT,
    BOARD_START_GDB_SERVER_COMMAND,
    BOARD_TARGET_TRIPLE,
    DEMOS_DIRECTORY,
    HOST_GDB_EXECUTABLE,
    LOGGER_INIT_FUNCTION_NAME,
)

if TYPE_CHECKING:
    from pathlib import Path


class ExitReason(IntEnum):
    """Exit reason of current script.
    If the script crashes, check it's exit code and compare to value from this enum."""

    INVALID_ARGUMENTS = 1
    INVALID_DEMO_PATH = 2
    CARGO_NOT_FOUND = 3
    CANNOT_CONNECT_TO_REMOTE_GDB = 4
    CANNOT_START_RTT_SERVER = 5
    DEMO_NOT_BUILT = 6
    CANNOT_LOAD_EXECUTABLE = 7
    CANNOT_RUN_EXECUTABLE = 8
    CANNOT_SET_BREAKPOINT_AT_RTT_INIT = 9
    UNEXPECTED_STOP = 10
    COULD_NOT_FIND_RTT_SYMBOL = 11
    COULD_NOT_SETUP_RTT = 12
    COULD_NOT_START_RTT = 13


def get_args() -> str:
    """Parses and returns script's arguments, exist the program with non-zero exit code
    if arguments are missing or cannot be parsed."""
    if len(sys.argv) != 2:  # noqa: PLR2004 (magic number self-explanatory)
        print(
            "SAMV71 demo runner, starts remote GDB session, configures RTT, "
            "runs demo and prints it's output on stdout",
        )
        print(f"Usage: {sys.argv[0]} demo-name")
        sys.exit(ExitReason.INVALID_ARGUMENTS)

    return sys.argv[1]


def get_demo_path(demo_name: str) -> Path:
    """Returns path to the project directory of provided demo."""
    return DEMOS_DIRECTORY / demo_name


def start_gdb() -> tuple[GDBClient, SSHClient]:
    """Connects to remote setup via SSH and starts GDB server."""
    ssh = SSHClient(BOARD_NETWORK_PATH, BOARD_LOGIN, BOARD_PASSWORD)
    ssh.execute(BOARD_START_GDB_SERVER_COMMAND)

    gdb = GDBClient(HOST_GDB_EXECUTABLE, log_responses=False, log_execution=False)
    remote_gdb_hostname = f"{BOARD_NETWORK_PATH}:{BOARD_GDB_PORT}"
    if not gdb.connect_to_remote(remote_gdb_hostname):
        print(f"Error: Could not connect to remote GDB server @ {remote_gdb_hostname}!")
        sys.exit(ExitReason.CANNOT_CONNECT_TO_REMOTE_GDB)

    return gdb, ssh


def load_and_start_program(gdb: GDBClient, program_path: Path) -> None:
    """Loads the provided executable into target microcontroller's memory via GDB and
    begins it's execution, stopping at `main` function."""
    abs_path = program_path.absolute()
    if not program_path.exists():
        print(f"Error: trying to run non-existent program {abs_path}!")
        sys.exit(ExitReason.DEMO_NOT_BUILT)

    if not gdb.load_executable(str(abs_path)):
        print(f"Error: could not load executable {abs_path}!")
        sys.exit(ExitReason.CANNOT_LOAD_EXECUTABLE)

    if not gdb.start_program():
        print(f"Error: could not start execution of executable {abs_path}!")
        sys.exit(ExitReason.CANNOT_RUN_EXECUTABLE)


def wait_for_rtt_init(gdb: GDBClient) -> None:
    """Sets the breakpoint at function performing RTT initialization and waits until it
    finishes, leaving the program in stopped state, so the RTT can be setup on the host
    side."""
    rtt_init_function_full_name = LOGGER_INIT_FUNCTION_NAME

    if not gdb.set_breakpoint(rtt_init_function_full_name):
        print(
            "Error: Cannot set the breakpoint on RTT init function "
            f"{rtt_init_function_full_name}!",
        )
        sys.exit(ExitReason.CANNOT_SET_BREAKPOINT_AT_RTT_INIT)

    gdb.continue_program()

    if not gdb.wait_for_breakpoint_hit():
        print("Error: Program has stopped, but not because of a breakpoint!")
        sys.exit(ExitReason.UNEXPECTED_STOP)

    gdb.finish_function_execution()


def setup_rtt(gdb: GDBClient) -> RTTClient:
    """Configures the RTT on host side by fetching RTT section symbol address from the
    binary and initializing RTT facilities."""
    if (rtt_symbol := gdb.get_variable(RTT_SECTION_SYMBOL_NAME)) is None:
        print(f"Error: Could not find RTT symbol '{RTT_SECTION_SYMBOL_NAME}' in the binary!")
        sys.exit(ExitReason.COULD_NOT_FIND_RTT_SYMBOL)

    if not gdb.start_rtt_server(BOARD_RTT_PORT, 0):
        print(f"Error: Could not start RTT server @ {BOARD_RTT_PORT}!")
        sys.exit(ExitReason.CANNOT_START_RTT_SERVER)

    if not gdb.setup_rtt(rtt_symbol.address, RTT_SECTION_SEARCHED_MEMORY_LENGTH, RTT_SECTION_ID):
        print(
            f"Could not setup RTT for section @ {rtt_symbol.address} "
            f"searched {RTT_SECTION_SEARCHED_MEMORY_LENGTH} bytes)",
        )
        sys.exit(ExitReason.COULD_NOT_SETUP_RTT)

    if not gdb.start_rtt():
        print("Could not start RTT!")
        sys.exit(ExitReason.COULD_NOT_START_RTT)

    return RTTClient(BOARD_NETWORK_PATH, BOARD_RTT_PORT)


def main() -> NoReturn:
    """Main function of the script."""
    demo_name = get_args()
    demo_path = get_demo_path(demo_name)
    if not demo_path.exists():
        print(
            f"Error: demo '{demo_path.absolute()}' does not exist!\n"
            "Make sure you're running this script from root directory of the project, "
            "and that the demo name is correct!",
        )
        sys.exit(ExitReason.INVALID_DEMO_PATH)

    program_path = build_cargo_app(demo_path, BOARD_TARGET_TRIPLE, release_build=True)
    if program_path is None:
        print("Error: Cargo executable not found!")
        sys.exit(ExitReason.CARGO_NOT_FOUND)

    gdb, ssh = start_gdb()
    load_and_start_program(gdb, program_path)
    wait_for_rtt_init(gdb)
    rtt = setup_rtt(gdb)

    gdb.continue_program()

    while True:
        try:
            print(rtt.receive_string(), end="")
        except KeyboardInterrupt:  # noqa: PERF203 (we don't care about performance here)
            ssh.execute("killall openocd")
            ssh.close()
            sys.exit(0)


if __name__ == "__main__":
    init_default_logger()
    main()
