import logging
from typing import List, Optional

from gdb_interface import GDBInterface
from gdb_responses import GDBResponsesList, ProgramSymbol


class GDBClient:
    """Class acting as GDB front-end. Provides high-level functionality.
    Made with OpenOCD in mind, but should *mostly* work with other backends."""

    DEFAULT_TIMEOUT = 10.0
    """Default GDB operation timeout, in seconds"""

    def __init__(
        self,
        gdb_executable: str = "gdb",
        default_timeout: Optional[float] = None,
        log_responses: bool = True,
    ):
        """Initialize GDB Client. Creates and initializes a GDBInterface instance internally.

        # Parameters
        * `gdb_executable` - Path to GDB executable, passed to GDBInterface() constructor
        * `default_timeout` - Default timeout for GDB operations, passed to GDBInterface()
                              constructor. If None, `GDBClient.DEFAULT_TIMEOUT` is used instead.
        * `log_responses` - If true, all meaningful responses will be logged to default log sink.
        """
        if default_timeout is None:
            default_timeout = self.DEFAULT_TIMEOUT

        self._logger = logging.getLogger(self.__class__.__name__)
        self._should_log_responses = log_responses
        self._interface = GDBInterface(gdb_executable, default_timeout)
        self._timeout = default_timeout

        self._logger.info("GDBClient instance created!")

    def connect_to_remote(self, hostname: str) -> bool:
        """Connects to remote GDB instance.
        Returns `True` on successful connection, `False` otherwise.

        # Parameters
        * `hostname` - Hostname of remote target, usually in `host:port` format.
        """
        self._logger.info(f"Connecting to remote GDB instance @ '{hostname}'...")
        self._interface.execute(f"target extended-remote {hostname}")
        return self._wait_for_command_response(
            "Connected!", "Connection failed, check the hostname!"
        )

    def select_executable(self, executable_path: str) -> bool:
        """Selects an executable file to be used by GDB via `file` command.
        Returns `True` on successful selection, `False` otherwise.

        # Parameters
        * `executable_path` [str] - Path to debugged file on local system.
        """
        self._logger.info(f"Selecting '{executable_path}' as debugged program...")
        self._interface.execute(f"file {executable_path}")
        return self._wait_for_command_response(
            "Executable selected!", "Selecting executable failed, check the path!"
        )

    def load_executable(self, executable_path: Optional[str] = None) -> bool:
        """Loads an executable file to target memory. If `executable_path` is provided, it will also
        select it, otherwise it must be selected manually before calling this function.
        Returns `True` if loading was successful, `False` otherwise.

        # Parameters
        * `executable_path` [Optional[str]] - Optional path to debugged executable file that will
                                              be loaded. If not provided, must be set via
                                              `select_executable()` before calling this function.
        """
        self._logger.info("Loading executable into memory...")

        if executable_path is not None:
            if not self.select_executable(executable_path):
                self._logger.error("Loading executable failed, because it cannot be selected.")
                return False

        self._interface.execute("load")
        return self._wait_for_command_response("Executable loaded!", "Loading executable failed!")

    def restart_platform(self, reset_type: str = "init") -> bool:
        """Resets the platform via GDB.
        Returns `True` if restarting was successful, `False` otherwise.

        # Parameters
        * `reset_type` [str] - Type of reset to be performed, can be "run", "halt" or "init".
        """
        self._logger.info("Restarting the platform...")
        self._interface.execute(f"monitor reset {reset_type}")
        return self._wait_for_command_response(
            "Platform restarted", "Restarting the platform has failed!"
        )

    def pause_program(self, force: bool = False):
        """Stops the execution of debugged program immediately.
        Does nothing if program is already stopped, unless `force` is `True`."""
        if not force and not self._interface.program_state().is_running:
            self._logger.warning("Cannot stop execution of already stopped program!")
            return

        self._logger.info("Pausing execution...")
        self._interface.interrupt()
        self._wait_for_stopped()
        self._logger.info("Execution paused!")

    def continue_program(self, force: bool = False):
        """Continues the execution of debugged program.
        Does nothing if program is already running, unless `force` is `True."""
        if not force and self._interface.program_state().is_running:
            self._logger.warning("Cannot continue execution of already running program!")
            return

        self._logger.info("Continuing execution...")
        self._interface.execute("-exec-continue")
        self._wait_for_running()
        self._logger.info("Execution continued!")

    def start_program(self, break_on: Optional[str] = "main"):
        """Reset the target and start execution of loaded program.
        Program must be loaded with `load_executable()`, or
        selected with `select_executable()`.

        # Parameters
        * `break_on` - Name of the function on which the program will stop.
                       This can be used to perform some preparations after the program has
                       started and initialized the hardware, but before the actual logic
                       started happening. If `None`, program will just start execution
                       without stopping.
        """
        self._logger.info("Starting program from it's entry point...")
        self.restart_platform()
        if break_on is not None:
            self.set_breakpoint(break_on)
            self._logger.info(f"Init breakpoint placed on '{break_on}'")
        self._interface.execute("-exec-run")
        self._wait_for_running()
        self._logger.info("Program is running!")

        if break_on is not None:
            self._logger.info("Waiting for breakpoint...")
            self.wait_for_breakpoint_hit()
            self._logger.info(
                f"Program stopped on '{str(self._interface.program_state().program_frame)}'"
                ", ready to continue!"
            )

    def set_breakpoint(
        self,
        location: str,
        temporary: bool = True,
    ) -> bool:
        """Set a breakpoint at specified location.

        # Parameters
        * `location` - Location of the breakpoint.
        * `temporary` - If `True`, breakpoint will be deleted after first hit.
        """
        arguments = "-t -h" if temporary else "-h"
        self._interface.execute(f"-break-insert {arguments} {location}")
        return self._wait_for_command_response("Breakpoint inserted!", "Cannot insert breakpoint!")

    def set_regex_breakpoint(self, pattern: str) -> bool:
        """Set a breakpoint at location specified by regular expression.
        Uses `rbreak` GDB command. Cannot specify hardware/temporary breakpoint via this
        function - if you need that, you must use `set_breakpoint`. All breakpoints set via
        this functions are permanent and must be deleted manually."""
        self._interface.execute(f"rbreak {pattern}")
        return self._wait_for_command_response(
            "Regex breakpoint inserted!", "Cannot insert breakpoint!"
        )

    def wait_for_breakpoint_hit(self, timeout: Optional[float] = None) -> bool:
        """Waits until a breakpoint is hit.
        Returns `True` if stopped by breakpoint, `False` if stopped by other means.
        Raises a `pygdbmi.constants.GdbTimeoutError` on timeout.

        # Parameters
        * `timeout` - Time, in seconds, to wait for the breakpoint. If `None`, default timeout
                      will be used instead.
        """
        if timeout is None:
            timeout = self._timeout

        while not self._interface.program_state().stopped_by_breakpoint():
            if not self._interface.program_state().is_running:
                return False
            self._interface.get_responses(timeout, self._should_log_responses)
        return True

    def finish_function_execution(self, force: bool = False):
        """Finishes current function execution.
        Does nothing if program is running, unless `force` is `True`."""
        if not force and self._interface.program_state().is_running:
            self._logger.warning(
                "Cannot finish function execution because program is already running!"
            )
            return

        self._interface.execute("finish")
        while not self._interface.program_state().function_finished_execution():
            self._get_responses()

    def setup_rtt(self, address: int, section_size: int, section_id: str) -> bool:
        """Configures RTT via `monitor rtt setup`.
        This function is OpenOCD-specific.
        Returns `True` if RTT was setup successfully, `False` otherwise.

        # Parameters
        * `address` - Address of RTT section in MCU memory
        * `section_size` - Expected RTT section size
        * `section_id` - RTT section ID
        """
        self._interface.execute(
            f'monitor rtt setup 0x{address:08X} 0x{section_size:X} "{section_id}"'
        )
        return self._wait_for_command_response("RTT set up!", "RTT setup failed!")

    def start_rtt(self) -> bool:
        """Starts RTT. RTT must be configured via `setup_rtt` before calling this.
        Returns `True` if RTT section has been found successfully, `False` otherwise."""
        self._interface.execute("monitor rtt start")
        return self._wait_for_command_response(
            "RTT started!", "Couldn't start RTT, check section configuration!"
        )

    def start_rtt_server(self, server_port: int, rtt_channel: int):
        """Creates a TCP socket for specified RTT channel.
        Returns `True` if server was started successfully, `False` otherwise.

        # Parameters
        * `server_port` - TCP port of the listening socket
        * `rtt_channel` - RTT channel to be forwarded
        """
        self._interface.execute(f"monitor rtt server start {server_port} {rtt_channel}")
        return self._wait_for_command_response("RTT server started!", "Couldn't start RTT server!")

    def get_variables(self, name_regex: str) -> Optional[List[ProgramSymbol]]:
        """Queries GDB about variable with name specified by provided regular expression.
        Returns a list with all found occurrences, or `None` of request fails.
        """
        self._interface.execute(f"info variables {name_regex}")
        responses = self._wait_for_done_or_error()
        if responses.contains_error():
            self._logger.error(f"Failed to fetch info about variable with pattern '{name_regex}'!")
            return None

        return responses.to_symbols_list()

    def get_variable(self, name_regex: str) -> Optional[ProgramSymbol]:
        """Queries GDB about variable with name specified by provided regular expression.

        Variable name is a regular expression.
        Returns `None` if no variables are found.

        If multiple variables are found, info about first one found is returned, and a warning is logged.
        """
        found_symbols = self.get_variables(name_regex)
        if found_symbols is None:
            return None

        if len(found_symbols) == 0:
            self._logger.info(f"No variable with pattern '{name_regex}' found!")
            return None

        if len(found_symbols) > 1:
            self._logger.warning(
                f"{len(found_symbols)} symbols with pattern '{name_regex}' found!"
                f"Returning first one ({str(found_symbols[0])})."
            )

        return found_symbols[0]

    def _get_responses(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `get_responses` with default arguments."""
        return self._interface.get_responses(
            timeout=self._timeout, log_responses=self._should_log_responses
        )

    def _wait_for_running(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `wait_for_running` with default arguments."""
        return self._interface.wait_for_running(
            timeout=self._timeout, log_responses=self._should_log_responses
        )

    def _wait_for_stopped(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `wait_for_stopped` with default arguments."""
        return self._interface.wait_for_stopped(
            timeout=self._timeout, log_responses=self._should_log_responses
        )

    def _wait_for_done_or_error(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `wait_for_done_or_error` with default arguments."""
        return self._interface.wait_for_done_or_error(
            timeout=self._timeout, log_responses=self._should_log_responses
        )

    def _wait_for_command_response(self, success_message: str, error_message: str) -> bool:
        """Private function. Do not use.
        Helper function for checking command response status and logging appropriate message.
        Returns `True` if response indicates success, `False` otherwise."""
        responses = self._wait_for_done_or_error()
        if responses.contains_error():
            self._logger.error(error_message)
        else:
            self._logger.info(success_message)
        return not responses.contains_error()
