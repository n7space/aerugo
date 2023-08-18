import os
from typing import Tuple
from calldwell.gdb_client import GDBClient
from calldwell.ssh_client import SSHClient
from calldwell.rtt_client import RTTClient
from calldwell.rust_helpers import init_remote_calldwell_rs_session

BOARD_LOGIN = str(os.environ.get("AERUGO_BOARD_LOGIN"))
BOARD_PASSWORD = str(os.environ.get("AERUGO_BOARD_PASSWORD"))
BOARD_HOSTNAME = str(os.environ.get("AERUGO_BOARD_HOSTNAME"))
BOARD_GDB_PORT = int(str(os.environ.get("AERUGO_BOARD_GDB_PORT")))
BOARD_RTT_PORT = int(str(os.environ.get("AERUGO_BOARD_RTT_PORT")))
GDB_EXECUTABLE = "arm-none-eabi-gdb"

MCU_INIT_MESSAGE = "calldwell-rs started"
HOST_HANDSHAKE_MESSAGE = "host handshake requested"
EXPECTED_MCU_HANDSHAKE_MESSAGE = (
    f"{len(HOST_HANDSHAKE_MESSAGE)}:{HOST_HANDSHAKE_MESSAGE}"
)


def init_test(test_binary_path: str) -> Tuple[GDBClient, RTTClient, SSHClient]:
    """Creates SSH connection to target board, initializes Calldwell"""
    ssh = SSHClient(BOARD_HOSTNAME, BOARD_LOGIN, BOARD_PASSWORD)
    ssh.execute("./setup_debugging_sam_clean.sh")

    session = init_remote_calldwell_rs_session(
        gdb_executable=GDB_EXECUTABLE,
        gdb_server_hostname=BOARD_HOSTNAME,
        gdb_server_port=BOARD_GDB_PORT,
        rtt_server_port=BOARD_RTT_PORT,
        path_to_test_executable=test_binary_path,
    )

    if session is None:
        print("TEST FAILED: CANNOT INITIALIZE CALLDWELL SESSION")
        exit(1)

    gdb, rtt = session

    gdb.continue_program()

    init_message = rtt.receive_string()
    if init_message != MCU_INIT_MESSAGE:
        print(
            f"TEST FAILED: MCU SENT INVALID INIT MESSAGE (got: {init_message}, expected: {MCU_INIT_MESSAGE})"
        )
        exit(2)

    rtt.transmit_string(HOST_HANDSHAKE_MESSAGE)

    response = rtt.receive_string()
    if response != EXPECTED_MCU_HANDSHAKE_MESSAGE:
        print(
            f"TEST FAILED: MCU SENT INVALID HANDSHAKE MESSAGE (got: {response}, expected: {EXPECTED_MCU_HANDSHAKE_MESSAGE})"
        )
        exit(3)

    return gdb, rtt, ssh


def finish_test(ssh: SSHClient):
    ssh.execute("pkill openocd")
    ssh.close()
