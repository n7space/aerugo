import os
from typing import Tuple
from calldwell.gdb_client import GDBClient
from calldwell.ssh_client import SSHClient
from calldwell.rtt_client import RTTClient

BOARD_LOGIN = str(os.environ.get("AERUGO_BOARD_LOGIN"))
BOARD_PASSWORD = str(os.environ.get("AERUGO_BOARD_PASSWORD"))
BOARD_HOSTNAME = str(os.environ.get("AERUGO_BOARD_HOSTNAME"))
BOARD_GDB_PORT = str(os.environ.get("AERUGO_BOARD_GDB_PORT"))
BOARD_RTT_PORT = str(os.environ.get("AERUGO_BOARD_RTT_PORT"))
GDB_EXECUTABLE = "arm-none-eabi-gdb"


def init_test(test_binary_path: str) -> Tuple[GDBClient, RTTClient, SSHClient]:
    ssh = SSHClient(BOARD_HOSTNAME, BOARD_LOGIN, BOARD_PASSWORD)
    ssh.execute("./setup_debugging_sam_clean.sh")

    gdb = GDBClient(GDB_EXECUTABLE, log_responses=False, log_execution=False)
    gdb.connect_to_remote(f"{BOARD_HOSTNAME}:{BOARD_GDB_PORT}")
    gdb.start_rtt_server(int(BOARD_RTT_PORT), 0)

    rtt = RTTClient(BOARD_HOSTNAME, port=int(BOARD_RTT_PORT))

    gdb.load_executable(test_binary_path)
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
