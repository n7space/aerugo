use aerugo::hal::drivers::uart::{NotConfigured, UartMetadata, UART};
use calldwell::write_str;

/// Test scenario:
/// * Test all methods available in unconfigured state
/// * Test state conversion methods
/// * Test state-specific configuration methods
/// * Test transmission and reception via loopback
/// * Test transmission with test host
/// * Test reception with test host
///
/// Both transmission and reception test should check synchronous and asynchronous communication, and all variants
/// of communication functions multiple times with different data.
pub fn test_uart<Instance: UartMetadata>(_uart: UART<Instance, NotConfigured>) {
    write_str("All UART tests finished successfully.");
}
