"""Module containing some boilerplate, common to all tests that use `calldwell-rs` library."""

from typing import Optional, Tuple

from .gdb_client import GDBClient
from .rtt_client import RTTClient

RTT_SECTION_SYMBOL_NAME = "_SEGGER_RTT"
"""Section name of RTT symbol. Hard-coded in `rtt_target` library."""
RTT_SECTION_SEARCHED_MEMORY_LENGTH = 0x800
"""This constant defines the amount of bytes OpenOCD will search, looking for RTT section.
Current value might as well be an overkill, but it works."""
RTT_SECTION_ID = "SEGGER RTT"
"""RTT section ID"""
CALLDWELL_INIT_FUNCTION_NAME = "calldwell::initialize"
"""Name of Calldwell-rs initialization function. Note that this is for `debug` binary,
as mangled `release` names are not supported (yet)."""


def init_remote_calldwell_rs_session(
    gdb_executable: str,
    gdb_server_hostname: str,
    gdb_server_port: int,
    rtt_server_port: int,
    path_to_test_executable: str,
    gdb_timeout: Optional[float] = None,
    log_responses: bool = False,
    log_execution: bool = False,
) -> Optional[Tuple[GDBClient, RTTClient]]:
    """Initializes Calldwell-rs test session by connecting to GDB server (like OpenOCD),
    starting RTT server, flashing the executable, and waiting for `calldwell::initialize`
    to execute.

    This function returns a tuple containing `GDBClient` with program in stopped state right after
    `calldwell:initialize` execution, and `RTTClient`, or `None` if starting the session fails at
    any point.

    This function can also throw one of the `pygdbmi` exceptions, like `GdbTimeoutError`.

    To check what caused the issue, you should check GDBClient logs. Writing proper error handling
    for all the scenarios is certainly possible, but usually the error is unrecoverable and
    should fail the test anyway, so there's no point in proper error handling anyway.

    # Parameters
    * `gdb_executable` - Path to GDB executable
    * `gdb_server_hostname` - Network path to GDB server. If ran locally, use `localhost`
    * `gdb_server_port` - Network port of GDB server
    * `rtt_server_port` - Port for RTT communication that will be opened by GDB
    * `path_to_test_executable` - Path to Calldwell test executable
    * `gdb_timeout` - Timeout for GDBClient, if `None` then default one will be used.
    * `log_responses` - Whether to log GDB/MI responses, or not
    * `log_execution` - Whether to log the execution of GDB commands, or not
    """
    gdb_server_full_hostname = f"{gdb_server_hostname}:{gdb_server_port}"
    gdb = GDBClient(
        gdb_executable,
        gdb_timeout,
        log_responses,
        log_execution,
    )

    if not gdb.connect_to_remote(gdb_server_full_hostname):
        return None

    if not gdb.start_rtt_server(rtt_server_port, 0):
        return None

    rtt = RTTClient(gdb_server_hostname, rtt_server_port)

    if not gdb.load_executable(path_to_test_executable):
        return None

    rtt_symbol = gdb.get_variable(RTT_SECTION_SYMBOL_NAME)
    if rtt_symbol is None:
        return None

    if not gdb.start_program():
        return None

    if not gdb.setup_rtt(rtt_symbol.address, RTT_SECTION_SEARCHED_MEMORY_LENGTH, RTT_SECTION_ID):
        return None

    if not gdb.set_breakpoint(CALLDWELL_INIT_FUNCTION_NAME):
        return None

    gdb.continue_program()

    if not gdb.wait_for_breakpoint_hit():
        return None

    gdb.finish_function_execution()

    if not gdb.start_rtt():
        return None

    return gdb, rtt
