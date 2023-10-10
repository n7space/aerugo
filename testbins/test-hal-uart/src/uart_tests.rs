use aerugo::hal::drivers::uart::{
    Config as UartConfig, NotConfigured, ParityBit, ReceiverConfig, UartMetadata, UART,
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
    let uart = test_uart_configuration(uart);
    let _uart = test_uart_state_transition(uart);
    write_str("All UART functional tests finished successfully.");
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

    write_str("UART configuration test finished successfully");
    uart.disable()
}

fn test_uart_state_transition<Instance: UartMetadata>(
    uart: UART<Instance, NotConfigured>,
) -> UART<Instance, NotConfigured> {
    // "dummy" configs can be set, as long as they are not used. It's user's responsibility to
    // make sure that the settings they enter are valid, the code can just check whether they are "reasonably"
    // valid. At least until UART driver gets integrated with PMC.
    let test_config_a = UartConfig::new(125_000, 12.MHz()).unwrap();
    let test_config_b = UartConfig::new(9600, 16.MHz()).unwrap();
    let test_config_c = UartConfig::new(115200, 20.MHz()).unwrap();
    let test_receiver_config_a = ReceiverConfig {
        rx_filter_enabled: true,
    };
    let test_receiver_config_b = ReceiverConfig {
        rx_filter_enabled: false,
    };

    // Check if UART can be successfully converted into every available state.
    // This is more of a compile-time check, than runtime, as these transitions should never fail.
    // Check UART config after every transition to confirm it's been applied correctly.
    let uart = uart.into_receiver(test_config_a, test_receiver_config_a);
    assert_eq!(uart.config(), test_config_a);
    assert_eq!(
        uart.is_rx_filter_enabled(),
        test_receiver_config_a.rx_filter_enabled
    );

    let uart = uart.into_transmitter(test_config_b);
    assert_eq!(uart.config(), test_config_b);

    let uart = uart.into_bidirectional(test_config_c, test_receiver_config_b);
    assert_eq!(uart.config(), test_config_c);
    assert_eq!(
        uart.is_rx_filter_enabled(),
        test_receiver_config_b.rx_filter_enabled
    );

    // Check that again, but this time de-initialize UART before changing the state.
    let uart = uart.disable();

    let uart = uart.into_receiver(test_config_a, test_receiver_config_a);
    assert_eq!(uart.config(), test_config_a);
    assert_eq!(
        uart.is_rx_filter_enabled(),
        test_receiver_config_a.rx_filter_enabled
    );

    let uart = uart.disable();
    let uart = uart.into_transmitter(test_config_b);
    assert_eq!(uart.config(), test_config_b);

    let uart = uart.disable();
    let uart = uart.into_bidirectional(test_config_c, test_receiver_config_b);
    assert_eq!(uart.config(), test_config_c);
    assert_eq!(
        uart.is_rx_filter_enabled(),
        test_receiver_config_b.rx_filter_enabled
    );

    write_str("UART state transition test finished successfully");
    uart.disable()
}
