use aerugo::hal::drivers::uart::{
    ClockSource, Config as UartConfig, NotConfigured, ParityBit, ReceiverConfig, UartMetadata, UART,
};
use aerugo::time::RateExtU32;
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
pub fn test_uart<Instance: UartMetadata>(uart: UART<Instance, NotConfigured>) {
    let test_default_config = UartConfig::new(125_000, 12.MHz()).unwrap();
    let _test_pck_config = test_default_config
        .with_clock_source(ClockSource::ProgrammableClock)
        .with_clock_source_frequency(4.MHz())
        .unwrap();
    let _test_receiver_config = ReceiverConfig {
        rx_filter_enabled: true,
    };

    let _uart = test_uart_configuration(uart);
    write_str("All UART tests finished successfully.");
}

fn test_uart_configuration<Instance: UartMetadata>(
    uart: UART<Instance, NotConfigured>,
) -> UART<Instance, NotConfigured> {
    let expected_config = UartConfig::new(125_000, 12.MHz()).unwrap();
    let mut uart = uart.into_transmitter(expected_config);

    // Check config validity
    assert_eq!(uart.config(), expected_config);
    assert_eq!(uart.baudrate(), expected_config.baudrate());
    assert_eq!(uart.clock_source(), expected_config.clock_source());
    assert_eq!(
        uart.clock_source_frequency(),
        expected_config.clock_source_frequency()
    );
    assert_eq!(uart.parity_bit(), expected_config.parity_bit());
    assert_eq!(uart.clock_divider(), expected_config.clock_divider());

    // Check if UART can be re-configured.
    // In this state, only baudrate and parity bit settings can be changed safely.
    uart.set_baudrate(9600).unwrap();
    let expected_clock_divider = 78u16; // 12MHz/(16*9600)
    let expected_baudrate = 9615; // 12MHz/(16*78)
    assert_eq!(uart.baudrate(), expected_baudrate);
    assert_eq!(uart.clock_divider(), expected_clock_divider);

    uart.set_parity_bit(ParityBit::Even);
    assert_eq!(uart.parity_bit(), ParityBit::Even);
    uart.set_parity_bit(ParityBit::Mark);
    assert_eq!(uart.parity_bit(), ParityBit::Mark);
    uart.set_parity_bit(ParityBit::None);
    assert_eq!(uart.parity_bit(), ParityBit::None);
    uart.set_parity_bit(ParityBit::Odd);
    assert_eq!(uart.parity_bit(), ParityBit::Odd);
    uart.set_parity_bit(ParityBit::Space);
    assert_eq!(uart.parity_bit(), ParityBit::Space);

    // Check if invalid baudrate cannot be configured (and also edge cases)
    uart.set_baudrate(1)
        .expect_err("1bps @ 12MHz is invalid, as it would result in divider larger than 65535");
    uart.set_baudrate(1_000_000)
        .expect_err("1Mbps @ 12MHz is invalid, as it would result in divider smaller than 1");

    write_str("UART configuration tests finished successfully");
    uart.disable()
}
