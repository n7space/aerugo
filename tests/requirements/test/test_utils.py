import logging
import os
from pathlib import Path
from typing import Tuple

from calldwell.gdb_client import GDBClient
from calldwell.rtt_client import CalldwellRTTClient
from calldwell.rust_helpers import init_remote_calldwell_rs_session, build_cargo_app
from calldwell.ssh_client import SSHClient

BOARD_LOGIN = str(os.environ.get("AERUGO_BOARD_LOGIN"))
BOARD_PASSWORD = str(os.environ.get("AERUGO_BOARD_PASSWORD"))
BOARD_HOSTNAME = str(os.environ.get("AERUGO_BOARD_HOSTNAME"))
BOARD_GDB_PORT = int(str(os.environ.get("AERUGO_BOARD_GDB_PORT")))
BOARD_RTT_PORT = int(str(os.environ.get("AERUGO_BOARD_RTT_PORT")))
GDB_EXECUTABLE = "arm-none-eabi-gdb"
TEST_BINS_DIRECTORY = Path("./testbins")
TARGET_NAME = "thumbv7em-none-eabihf"


def setup_logger():
    logging.basicConfig(
        level=logging.INFO, format="[%(asctime)s] <%(levelname)s|%(name)s>: %(message)s"
    )


def init_test(test_name: str) -> Tuple[GDBClient, CalldwellRTTClient, SSHClient]:
    """Creates SSH connection to target board, initializes Calldwell"""
    project_path = TEST_BINS_DIRECTORY / test_name
    test_binary_path = build_cargo_app(project_path, TARGET_NAME)
    if test_binary_path is None:
        exit(100)

    logging.info("Starting the test, initializing the environment...")
    ssh = SSHClient(BOARD_HOSTNAME, BOARD_LOGIN, BOARD_PASSWORD)
    ssh.execute("./setup_debugging_sam_clean.sh")

    session = init_remote_calldwell_rs_session(
        gdb_executable=GDB_EXECUTABLE,
        gdb_server_hostname=BOARD_HOSTNAME,
        gdb_server_port=BOARD_GDB_PORT,
        rtt_server_port=BOARD_RTT_PORT,
        path_to_test_executable=str(test_binary_path.absolute()),
    )

    if session is None:
        logging.critical("Test failed, cannot initialize Calldwell session")
        exit(1)

    gdb, rtt = session

    logging.info("Environment initialized!")
    return gdb, rtt, ssh


def finish_test(ssh: SSHClient):
    logging.info("TEST SUCCESSFUL!")
    logging.info("Finishing the test, cleaning up environment...")
    ssh.execute("pkill openocd")
    ssh.close()
    logging.info("Environment cleaned up!")
