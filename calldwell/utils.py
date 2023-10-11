"""Module with utility functions"""

from __future__ import annotations

import time
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Callable


def wait_until_true(
    function: Callable[[], bool],
    timeout_seconds: float,
    check_delay: float = 0.1,
) -> float | None:
    """Executes provided function in a loop until it returns `True`.

    # Parameters
    * `function` - Function returning `True` on success. Accepts no arguments.
                   Usually this would be a lambda or local function.
    * `timeout_seconds` - Timeout, in seconds.
    * `check_delay` - Amount of time between function calls, 100ms by default.

    # Returns
    Time remaining until timeout (in seconds), or `None` if timeout has been hit.
    """
    elapsed_seconds = 0.0
    while elapsed_seconds < timeout_seconds:
        if not function():
            time.sleep(check_delay)
            elapsed_seconds += check_delay
        else:
            return timeout_seconds - elapsed_seconds

    return None
