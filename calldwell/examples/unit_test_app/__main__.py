"""An example project that uses Calldwell to communicate with Rust application."""

from __future__ import annotations

import logging
import os
import sys
from pathlib import Path
from typing import TYPE_CHECKING

from calldwell import init_default_logger
from calldwell.rust_helpers import build_cargo_app, init_remote_calldwell_rs_session

if TYPE_CHECKING:
    from calldwell.gdb_client import GDBClient
    from calldwell.rtt_client import CalldwellRTTClient
    from calldwell.ssh_client import SSHClient

# This example is prepared to run on remote development setup and requires following
# environmental variables:
try:
    BOARD_LOGIN = str(os.environ.get("CALLDWELL_BOARD_LOGIN"))
    """SSH login of the development setup"""
    BOARD_PASSWORD = str(os.environ.get("CALLDWELL_BOARD_PASSWORD"))
    """SSH password of the development setup"""
    BOARD_HOSTNAME = str(os.environ.get("CALLDWELL_BOARD_HOSTNAME"))
    """Development setup's hostname/IP address"""
    BOARD_GDB_EXEC_SCRIPT = str(os.environ.get("CALLDWELL_BOARD_GDB_EXEC_SCRIPT"))
    """Script that should run GDB server on development setup"""
    BOARD_GDB_PORT = int(str(os.environ.get("CALLDWELL_BOARD_GDB_PORT")))
    """GDB port of development's setup GDB server"""
    BOARD_RTT_PORT = int(str(os.environ.get("CALLDWELL_BOARD_RTT_PORT")))
    """Port on which the RTT server will provide RTT data"""
    GDB_EXECUTABLE = str(os.environ.get("CALLDWELL_GDB_EXECUTABLE"))
    """GDB executable name/path"""
    TARGET_TRIPLE = str(os.environ.get("CALLDWELL_TARGET_TRIPLE"))
    """Target platform triple (for example, thumbv7em-none-eabihf)"""
except ValueError as e:
    print("Missing/invalid environmental variables, read the source of this script for details!")
    print(f"Exception: {e}")
    sys.exit(1)


def init_example() -> tuple[GDBClient, CalldwellRTTClient, SSHClient]:
    """Initializes Calldwell session for this example application"""
    project_path = Path(sys.argv[0])

    logging.info("Building the example binary...")
    if (test_bin_path := build_cargo_app(project_path, TARGET_TRIPLE, release_build=True)) is None:
        logging.error("Could not build the binary!")
        sys.exit(100)

    logging.info("Starting GDB server on development setup..")
    session = init_remote_calldwell_rs_session(
        debug_host_network_path=BOARD_HOSTNAME,
        debug_host_login=BOARD_LOGIN,
        debug_host_password=BOARD_PASSWORD,
        gdb_server_port=BOARD_GDB_PORT,
        rtt_server_port=BOARD_RTT_PORT,
        local_gdb_executable=GDB_EXECUTABLE,
        remote_gdb_server_command=BOARD_GDB_EXEC_SCRIPT,
        path_to_test_executable=str(test_bin_path.absolute()),
    )

    if session is None:
        logging.error("Failed to initialize remote Calldwell-rs session!")
        sys.exit(200)

    ssh, gdb, rtt = session

    logging.info("Example started!")
    return gdb, rtt, ssh


def main() -> None:
    """Main function of this example"""
    _, rtt, ssh = init_example()

    message = rtt.receive_string_stream()
    while message != "All tests done!":
        print(message)
        message = rtt.receive_string_stream()

    print(message)
    ssh.close()


if __name__ == "__main__":
    init_default_logger()
    main()
