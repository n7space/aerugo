"""This is an example test suite that can be used for debugging or as template."""

from __future__ import annotations

from test_utils import finish_test, init_test, wait_for_messages

from calldwell import init_default_logger

TEST_NAME = "test-hal-xdmac"


def main() -> None:
    """Main function of integration test."""
    _, rtt, ssh = init_test(TEST_NAME)

    wait_for_messages(
        rtt,
        ssh,
        [
            "Performing XDMAC tests...",
            "Performing XDMAC management tests...",
            "XDMAC info test finished successfully.",
            "XDMAC status reader test finished successfully.",
            "XDMAC channel take/give test finished successfully.",
            "XDMAC management test finished successfully.",
            "Performing channel management tests...",
            "Channel status reader management test finished successfully.",
            "Channel IRQ configuration test finished successfully.",
            "Channel events configuration test finished successfully.",
            "Channel management tests finished successfully!",
            "Performing transfer config tests...",
            "Block length creation test finished successfully.",
            "Microblock length creation test finished successfully.",
            "Transfer block creation test finished successfully.",
            "Transfer config tests finished successfully!",
            "Performing transfer tests...",
            "Mem2mem 32-bit polling transfer test successful!",
            "Mem2mem 16-bit polling transfer test successful!",
            "Mem2mem 8-bit polling transfer test successful!",
            "Mem2mem interrupt transfer test successful!",
            "Per2mem and mem2per transfers test successful!",
            "Channel management operations (suspend/flush/disable) test successful!",
            "Transfer tests finished successfully!",
            "All XDMAC tests done!",
        ],
    )

    finish_test(ssh)


if __name__ == "__main__":
    init_default_logger()
    main()
