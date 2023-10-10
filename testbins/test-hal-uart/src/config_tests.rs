use aerugo::hal::drivers::uart::{
    config::{calculate_baudrate, calculate_clock_divider},
    ClockSource, Config, Frequency, ParityBit,
};

use aerugo::time::RateExtU32;
use calldwell::write_str;

/// Tests UART configuration structures, as they are responsible for validating UART config.
pub fn test_uart_config() {
    test_divider_calculation_functions();
    test_uart_config_creation();
    test_uart_config_methods();

    write_str("UART `Config` and calcs tests successful");
}

fn test_divider_calculation_functions() {
    assert_eq!(calculate_baudrate(1, 100.kHz()), 100_000 / 16);
    assert_eq!(calculate_baudrate(2, 100.kHz()), 100_000 / 32);
    assert_eq!(calculate_baudrate(3, 100.kHz()), 100_000 / 48);
    assert_eq!(calculate_baudrate(100, 100.kHz()), 100_000 / 1600);

    assert_eq!(
        calculate_clock_divider(1, 100.kHz()).unwrap(),
        (100_000 / 16) as u16
    );
    assert_eq!(
        calculate_clock_divider(2, 100.kHz()).unwrap(),
        (100_000 / 32) as u16
    );
    assert_eq!(
        calculate_clock_divider(3, 100.kHz()).unwrap(),
        (100_000 / 48) as u16
    );
    assert_eq!(
        calculate_clock_divider(100, 100.kHz()).unwrap(),
        (100_000 / 1600) as u16
    );

    calculate_clock_divider(1, 10.MHz())
        .expect_err("Reaching 1bps @ 10MHz would require divider equal to 625000, which is larger than 65535, therefore should not be possible");
    calculate_clock_divider(1_000_000, 100.kHz())
        .expect_err("Reaching 1Mbps @ 100kHz would require divider lesser than 1, therefore should not be possible");

    write_str("UART baudrate and divider calculation test successful");
}

fn test_uart_config_creation() {
    // Check if valid configurations can be created.
    Config::new(9600, 10.MHz()).expect("9600bps @ 10MHz should be valid, but it's apparently not");
    Config::new(115200, 15.MHz())
        .expect("115.200kbps @ 15MHz should be valid, but it's apparently not");

    // Check if creating configuration resulting in invalid divider fails
    Config::new(999_999, 1.Hz()).expect_err(
        "999.999kbps @ 1Hz should not be valid, because baudrate is larger than source clock frequency",
    );
    Config::new(1, 2.MHz()).expect_err(
        "1bps @ 2MHz should not be valid, as baudrate is smaller than (clock frequency)/(16*65535)",
    );

    // Check edge cases - maximum baudrate is 1/16th of source clock frequency.
    // That's 1Mbps baudrate for 16MHz clock.
    Config::new(999_999, 16.MHz()).expect(
        "999.999kbps @ 16MHz should be valid, as baudrate is less than 1/16th source clock frequency",
    );
    Config::new(1_000_000, 16.MHz()).expect(
        "1Mbps @ 16MHz should be valid, as baudrate is exactly 1/16th source clock frequency",
    );
    Config::new(1_000_001, 16.MHz()).expect_err(
        "1.000001Mbps @ 16MHz should be valid, as baudrate is more than 1/16th source clock frequency",
    );
    // Minimum baudrate is 1/(16*65535) of source clock frequency.
    // That's 10bps for 10485600Hz clock.
    Config::new(9, 10_485_600.Hz()).expect_err("9bps @ 10485600Hz should be invalid, as it's less than (source clock frequency)/(16*65535)");
    Config::new(10, 10_485_600.Hz()).expect(
        "10bps @ 10485600Hz should be valid, as it's exactly (source clock frequency)/(16*65535)",
    );
    Config::new(11, 10_485_600.Hz()).expect(
        "10bps @ 10485600Hz should be valid, as it's more than (source clock frequency)/(16*65535)",
    );

    write_str("UART `Config` creation test successful");
}

fn test_uart_config_methods() {
    // Nice, precise config.
    let config = Config::new(100_000, 16.MHz()).unwrap();
    assert_eq!(config.baudrate(), 100_000);
    assert_eq!(config.clock_source(), ClockSource::PeripheralClock);
    assert_eq!(config.clock_source_frequency(), Frequency::MHz(16));
    assert_eq!(config.parity_bit(), ParityBit::None);
    assert_eq!(config.clock_divider(), 10);

    // Bit more ugly config, as 9600 cannot be nicely made out of 6MHz clock.
    let config = Config::new(9_600, 6.MHz())
        .unwrap()
        .with_clock_source(ClockSource::ProgrammableClock)
        .with_parity_bit(ParityBit::Mark);
    assert_eq!(config.baudrate(), 9_615);
    assert_eq!(config.clock_source(), ClockSource::ProgrammableClock);
    assert_eq!(config.clock_source_frequency(), Frequency::MHz(6));
    assert_eq!(config.parity_bit(), ParityBit::Mark);
    assert_eq!(config.clock_divider(), 39);

    // Another seemingly ugly config, but this time, tail-change it's baudrate
    let config = Config::new(11_111, 12_345_678.Hz())
        .unwrap()
        .with_baudrate(123456)
        .unwrap();
    assert_eq!(config.baudrate(), 128_600);
    assert_eq!(config.clock_source(), ClockSource::PeripheralClock);
    assert_eq!(config.clock_source_frequency(), Frequency::Hz(12_345_678));
    assert_eq!(config.parity_bit(), ParityBit::None);
    assert_eq!(config.clock_divider(), 6);

    // And also change the clock source frequency. Baudrate should be close to original.
    // "Close" is a relative term.
    let config = config.with_clock_source_frequency(20.MHz()).unwrap();
    assert_eq!(config.baudrate(), 138_888);
    assert_eq!(config.clock_divider(), 9);

    // Check if divider can be set and if baudrate correctly updates.
    let config = unsafe { config.with_clock_divider(123) };
    assert_eq!(config.clock_divider(), 123);
    assert_eq!(config.baudrate(), 10_162);

    // Check if config cannot be tail-changed to an invalid value
    Config::new(100_000, 16.MHz())
        .unwrap()
        .with_baudrate(1)
        .expect_err(
            "1bps @ 16MHz should be invalid, as it would produce divider larger than 65535",
        );
    Config::new(100_000, 16.MHz())
        .unwrap()
        .with_baudrate(10_000_000)
        .expect_err("10Mbps @ 16MHz should be invalid, as it would produce divider lesser than 1");

    Config::new(100, 16.MHz())
        .unwrap()
        .with_clock_source_frequency(200.MHz())
        .expect_err(
            "100bps @ 200MHz should be invalid, as it would produce divider larger than 65535",
        );
    Config::new(100_000, 16.MHz())
        .unwrap()
        .with_clock_source_frequency(100.Hz())
        .expect_err("100kbps @ 100Hz should be invalid, as it would produce divider lesser than 1");

    write_str("UART `Config` methods test successful");
}
