"""Module containing GDB data objects. Instead of relying directly on `pygdbmi` output,
it's parsed by `GDBInterface` to proper, strongly typed objects from this module."""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import TYPE_CHECKING, Any, Union

if TYPE_CHECKING:
    from collections.abc import Iterable, Iterator


@dataclass
class ProgramSymbol:
    """Structure representing a symbol from the binary (function, variable, etc.)"""

    name: str
    address: int

    def address_string(self: ProgramSymbol, address_size: int = 4) -> str:
        """Returns address as hex string. Assumes 32-bit addresses by default."""
        stringified_address = f"{self.address:X}".zfill(address_size * 2)
        return f"0x{stringified_address}"

    def __str__(self: ProgramSymbol) -> str:
        return f"{self.name} @ {self.address_string()}"


@dataclass
class GDBResponse:
    """Structure representing GDB response."""

    Payload = Union[dict[str, Any], list[Any], str]

    message: str | None
    """Message is usually a human-readable string."""
    payload: Payload | None
    """Payload can be either a string, list or a dict."""
    token: Any | None
    response_type: Type
    """Response's type, always present."""
    stream: Stream | None
    """Response's stream, always present in GDB responses, but left as `Optional` to allow creating
    GDBResponses for comparison."""

    class Type(Enum):
        """Response type"""

        RESULT = "result"
        NOTIFY = "notify"
        CONSOLE = "console"
        LOG = "log"
        OUTPUT = "output"
        TARGET = "target"
        DONE = "done"

        def __str__(self: GDBResponse.Type) -> str:
            return self.value

        def __repr__(self: GDBResponse.Type) -> str:
            return str(self)

    class Stream(Enum):
        """Stream type"""

        STDOUT = "stdout"
        STDIN = "stdin"
        STDERR = "stderr"

        def __str__(self: GDBResponse.Stream) -> str:
            return self.value

        def __repr__(self: GDBResponse.Stream) -> str:
            return str(self)

    def is_similar(self: GDBResponse, other: GDBResponse) -> bool:
        """Checks if another response is 'similar' to current one.
        Similarity criteria are:
        - Both responses must have the same type to be similar, AND
        - If `other` response has message, then current one must have the same message
          to be similar, AND
        - If `other` response has payload, then current one must have the same payload

        In other words, type must be equal, and message/payload must be equal if set in `other`.
        """
        if self.response_type != other.response_type:
            return False

        if other.message is not None and self.message != other.message:
            return False

        if other.payload is not None and self.payload != other.payload:
            return False

        return True

    def is_any_similar(self: GDBResponse, responses: Iterable[GDBResponse]) -> bool:
        """Checks if any of the provided responses is similar to current one"""
        return any(self.is_similar(response) for response in responses)

    def unescaped_payload(self: GDBResponse) -> str:
        """Returns unescaped version of payload string.
        Currently fixes newlines, tabs and `"`"""
        return str(self.payload).replace("\\n", "\n").replace("\\t", "\t").replace('\\"', '"')

    def payload_is_json(self: GDBResponse) -> bool:
        """Returns `True` if payload is a JSON object, `False` otherwise"""
        return isinstance(self.payload, dict)

    def payload_json(self: GDBResponse) -> dict[str, Any]:
        """Returns payload as a JSON (or rather dict with string keys).
        Raises an exception if payload is not actually a dictionary."""
        if not self.payload_is_json():
            msg = "Payload is not a dictionary!"
            raise ValueError(msg)
        return self.payload  # type: ignore

    @staticmethod
    def with_message(message_type: Type, message: str) -> GDBResponse:
        """Returns a GDBResponse with only `type` and `message` fields set.
        Use this function to create a response object for `is_similar` comparison."""
        return GDBResponse(
            message=message,
            response_type=message_type,
            payload=None,
            token=None,
            stream=None,
        )

    @staticmethod
    def with_payload(message_type: Type, payload: Payload) -> GDBResponse:
        """Returns a GDBResponse with only `type` and `payload` fields set.
        Use this function to create a response object for `is_similar` comparison."""
        return GDBResponse(
            payload=payload,
            response_type=message_type,
            message=None,
            token=None,
            stream=None,
        )


class GDBResponsesList:
    """Class representing a list of GDB responses.

    Besides providing a list-like access to individual responses, it also provides some commonly
    performed operations on them, like checking if a specific type of response is on the list,
    or concatenating the console messages into singular string.
    """

    def __init__(self: GDBResponsesList, responses: list[GDBResponse]) -> None:
        """Initializes the list with responses received from GDB"""
        self._items = responses

    def of_type(self: GDBResponsesList, response_type: GDBResponse.Type) -> GDBResponsesList:
        """Return all the responses on the list that have specified type"""
        return GDBResponsesList(
            list(filter(lambda response: response.response_type == response_type, self)),
        )

    def results(self: GDBResponsesList) -> GDBResponsesList:
        """Returns a list containing only `result`-type messages"""
        return self.of_type(GDBResponse.Type.RESULT)

    def notifications(self: GDBResponsesList) -> GDBResponsesList:
        """Returns a list containing only `notify`-type messages"""
        return self.of_type(GDBResponse.Type.NOTIFY)

    def console(self: GDBResponsesList) -> GDBResponsesList:
        """Returns a list containing only `console`-type messages"""
        return self.of_type(GDBResponse.Type.CONSOLE)

    def logs(self: GDBResponsesList) -> GDBResponsesList:
        """Returns a list containing only `log`-type messages"""
        return self.of_type(GDBResponse.Type.LOG)

    def outputs(self: GDBResponsesList) -> GDBResponsesList:
        """Returns a list containing only `output`-type messages"""
        return self.of_type(GDBResponse.Type.OUTPUT)

    def target(self: GDBResponsesList) -> GDBResponsesList:
        """Returns a list containing only `target`-type messages"""
        return self.of_type(GDBResponse.Type.TARGET)

    def payload_string_list(self: GDBResponsesList, unescape: bool = True) -> list[str]:
        """Returns a list of stringified payloads from all responses.

        # Parameters
        * `unescape` - If `True`, escaped characters in payloads will be unescaped to
                       produce human-readable string.
        """
        if unescape:
            return [response.unescaped_payload() for response in self]
        return [str(response.payload) for response in self]

    def payload_string(self: GDBResponsesList, separator: str = "", unescape: bool = True) -> str:
        """Returns a single stringified payload from all responses. Returned string is stripped
        of whitespace at the beginning and the end.

        # Parameters
        * `separator` [str] - A separator inserted between each payload on the list.
        * `escape` [bool] - If `True`, escaped characters in payloads will be unescaped to
                            produce human-readable string.
        """
        return separator.join(self.payload_string_list(unescape)).strip()

    def extend(self: GDBResponsesList, other: GDBResponsesList) -> None:
        """Adds items from different response list to current one."""
        # This uses another instance of itself, so protected access should be allowed.
        # pylint: disable=protected-access
        self._items.extend(other._items)  # noqa: SLF001

    def contains_any(self: GDBResponsesList, responses: Iterable[GDBResponse]) -> bool:
        """Returns `True` if any of the provided responses is on the list. `False` otherwise.
        To see how the items are compared, see `GDBResponse.is_similar()`."""
        return any(response.is_any_similar(responses) for response in self)

    def contains_all(self: GDBResponsesList, responses: Iterable[GDBResponse]) -> bool:
        """Returns `True` if all of the provided responses are on the list. `False` otherwise.
        To see how the items are compared, see `GDBResponse.is_similar()`."""
        return all(response.is_any_similar(responses) for response in self)

    def contains_error(self: GDBResponsesList) -> bool:
        """Returns `True` if any of the stored responses is a result with `error` message."""
        return GDBResponse.with_message(GDBResponse.Type.RESULT, "error") in self

    def to_symbols_list(self: GDBResponsesList) -> list[ProgramSymbol]:
        """Looks for symbols in stored responses and returns them.
        Symbols are assumed to be stored in `console` output, in `address  symbol_name` format,
        one per response."""
        found_symbols: list[ProgramSymbol] = []
        for response in self.console():
            # Symbols are separated with double space
            split_payload = response.unescaped_payload().strip().split("  ")
            if len(split_payload) == 2:  # noqa: PLR2004 (magic number self-explanatory)
                try:
                    symbol_address = int(split_payload[0], 16)
                    symbol_name = split_payload[1]
                    found_symbols.append(ProgramSymbol(name=symbol_name, address=symbol_address))
                except ValueError:
                    # Ignore ValueError, as it means that this is simply not a symbol.
                    continue
        return found_symbols

    def __contains__(self: GDBResponsesList, expected: GDBResponse) -> bool:
        """Returns `True` if any response on the list is similar to provided one.
        To see how the items are compared, see `GDBResponse.is_similar()`."""
        return any(response.is_similar(expected) for response in self)

    def __len__(self: GDBResponsesList) -> int:
        """Returns amount of elements on the list"""
        return len(self._items)

    def __getitem__(self: GDBResponsesList, key: int) -> GDBResponse:
        """Returns a specific element of the list by it's index"""
        return self._items[key]

    def __iter__(self: GDBResponsesList) -> Iterator[GDBResponse]:
        """Returns an iterator over the elements on the list"""
        return self._items.__iter__()

    def __str__(self: GDBResponsesList) -> str:
        return "\n".join([f"[{i}] {response}" for i, response in enumerate(self)])

    def __repr__(self: GDBResponsesList) -> str:
        return str(self)
