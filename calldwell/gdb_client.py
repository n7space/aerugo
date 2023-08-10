import logging
from typing import Optional

from gdb_interface import GDBInterface
from gdb_response import GDBResponsesList


class GDBClient:
    """Convenience class for managing GDB client. Provides high-level functionality."""

    DEFAULT_TIMEOUT = 10.0
    """Default GDB operation timeout, in seconds"""

    def __init__(
        self,
        gdb_executable: str = "gdb",
        default_timeout: Optional[float] = None,
        log_responses: bool = True,
    ) -> None:
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
        self._log_responses = log_responses
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
        if not force and not self._interface.is_program_running():
            self._logger.warning("Cannot stop execution of already stopped program!")
            return

        self._logger.info("Pausing execution...")
        self._interface.interrupt()
        self._wait_for_stopped()
        self._logger.info("Execution paused!")

    def continue_program(self, force: bool = False):
        """Continues the execution of debugged program.
        Does nothing if program is already running, unless `force` is `True."""
        if not force and self._interface.is_program_running():
            self._logger.warning("Cannot continue execution of already running program!")
            return

        self._logger.info("Continuing execution...")
        self._interface.execute("-exec-continue")
        self._wait_for_running()
        self._logger.info("Execution continued!")

    def start_program(self, break_on: Optional[str] = "main"):
        """Reset the target and start execution of loaded program.
        Program must be loaded first with `load_executable()`.

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
            self.breakpoint(break_on)
            self._logger.info(f"Init breakpoint placed on {break_on}")
        self._interface.execute("-exec-run")
        self._wait_for_running()
        self._logger.info("Program is running!")

        if break_on is not None:
            self._logger.info("Waiting for breakpoint...")
            self._wait_for_stopped()
            self._logger.info(f"Program stopped on {break_on}, ready to continue!")

    def breakpoint(
        self,
        location: str,
        temporary: bool = True,
    ) -> bool:
        """Create a breakpoint at specified location.

        # Parameters
        * `location` - Location of the breakpoint.
        * `temporary` - If `True`, breakpoint will be deleted after first hit.
        """
        arguments = "-t -h" if temporary else "-h"
        self._interface.execute(f"-break-insert {arguments} {location}")
        return self._wait_for_command_response("Breakpoint inserted!", "Cannot insert breakpoint!")

    def _get_responses(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `get_responses` with default arguments."""
        return self._interface.get_responses(
            timeout=self._timeout, log_responses=self._log_responses
        )

    def _wait_for_running(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `wait_for_running` with default arguments."""
        return self._interface.wait_for_running(
            timeout=self._timeout, log_responses=self._log_responses
        )

    def _wait_for_stopped(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `wait_for_stopped` with default arguments."""
        return self._interface.wait_for_stopped(
            timeout=self._timeout, log_responses=self._log_responses
        )

    def _wait_for_done_or_error(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `wait_for_done_or_error` with default arguments."""
        return self._interface.wait_for_done_or_error(
            timeout=self._timeout, log_responses=self._log_responses
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
