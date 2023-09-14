"""Calldwell is a simple testing framework for embedded applications."""

import logging


def init_default_logger(debug_logs_enabled: bool = False) -> None:
    """Initializes default logger instance with Calldwell's log format"""
    logging.basicConfig(
        level=logging.DEBUG if debug_logs_enabled else logging.INFO,
        format="[%(asctime)s] <%(levelname)s|%(name)s>: %(message)s",
    )
