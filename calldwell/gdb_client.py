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

        self.logger = logging.getLogger(self.__class__.__name__)
        self.log_responses = log_responses
        self.interface = GDBInterface(gdb_executable, default_timeout)
        self.timeout = default_timeout

    def connect_to_remote(self, hostname: str) -> bool:
        """Connects to remote GDB instance.
        Returns `True` on successful connection, `False` otherwise.

        # Parameters
        * `hostname` - Hostname of remote target, usually in `host:port` format.
        """
        self.logger.info(f"Connecting to remote GDB instance @ {hostname}")
        self.interface.execute(f"target extended-remote {hostname}")
        responses = self._wait_for_done_or_error()
        if responses.contains_error():
            self.logger.info("Connection failed, check the hostname!")
        else:
            self.logger.info("Connected!")
        return responses.contains_error()

    def select_executable(self, executable_path: str):
        """Selects an executable file to be used by GDB via `file` command.

        # Parameters
        * `executable_path` [str] - Path to debugged file on local system.
        """
        self.logger.info(f"Selecting '{executable_path}' as debugged program")
        self.interface.execute(f"file {executable_path}")
        self._wait_for_done()
        self.logger.info("Executable selected!")

    def load_executable(self, executable_path: Optional[str] = None):
        """Loads an executable file to target memory. If `executable_path` is provided, it will also
        select it, otherwise it must be selected manually before calling this function.

        # Parameters
        * `executable_path` [Optional[str]] - Optional path to debugged executable file that will
                                              be loaded. If not provided, must be set via
                                              `select_executable()` before calling this function.
        """
        if executable_path is not None:
            self.select_executable(executable_path)

        self.logger.info("Loading executable into memory")
        self.interface.execute("load")
        self._wait_for_done()
        self.logger.info("Executable loaded!")

    def restart_platform(self, reset_type: str = "init"):
        """Resets the platform via GDB.

        # Parameters
        * `reset_type` [str] - Type of reset to be performed, can be "run", "halt" or "init".
        """
        self.logger.info("Restarting the platform")
        self.interface.execute(f"monitor reset {reset_type}")
        self._wait_for_done()
        self.logger.info("Platform restarted!")

    def pause_program(self):
        """Stops the execution of debugged program immediately."""
        self.logger.info("Pausing execution...")
        self.interface.interrupt()
        self._wait_for_stopped()
        self.logger.info("Execution paused!")

    def continue_program(self):
        """Continues the execution of debugged program."""
        self.logger.info("Continuing execution...")
        self.interface.execute("-exec-continue")
        self.interface.wait_for_running()
        self.logger.info("Execution continued!")

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
        self.logger.info("Starting program from the beginning...")
        self.restart_platform()
        if break_on is not None:
            self.interface.breakpoint(break_on)
            self.logger.info(f"Init breakpoint placed on {break_on}")
        self.interface.execute("-exec-run")
        self._wait_for_running()
        self.logger.info("Program is running!")

        if break_on is not None:
            self.logger.info("Waiting for breakpoint...")
            self._wait_for_stopped()
            self.logger.info(f"Program stopped on {break_on}, ready to continue!")

    def _get_responses(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `get_responses` with default arguments."""
        return self.interface.get_responses(timeout=self.timeout, log_responses=self.log_responses)

    def _wait_for_done(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `wait_for_done` with default arguments."""
        return self.interface.wait_for_done(timeout=self.timeout, log_responses=self.log_responses)

    def _wait_for_running(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `wait_for_running` with default arguments."""
        return self.interface.wait_for_running(
            timeout=self.timeout, log_responses=self.log_responses
        )

    def _wait_for_stopped(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `wait_for_stopped` with default arguments."""
        return self.interface.wait_for_stopped(
            timeout=self.timeout, log_responses=self.log_responses
        )

    def _wait_for_done_or_error(self) -> GDBResponsesList:
        """Private function. Do not use.
        Helper function for calling `wait_for_done_or_error` with default arguments."""
        return self.interface.wait_for_done_or_error(
            timeout=self.timeout, log_responses=self.log_responses
        )
