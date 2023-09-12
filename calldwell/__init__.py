"""Calldwell is a simple testing framework for embedded applications."""

import logging


def init_default_logger():
    """Initializes default logger instance with Calldwell's log format"""
    logging.basicConfig(
        level=logging.INFO, format="[%(asctime)s] <%(levelname)s|%(name)s>: %(message)s"
    )
