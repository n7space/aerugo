extern crate heapless;

use core::cell::RefCell;

use aerugo::hal::drivers::nvic::{Interrupt, NVIC};
use aerugo::hal::drivers::uart::config::LoopbackMode;
use aerugo::hal::drivers::uart::reader::Reader;
use aerugo::hal::drivers::uart::writer::Writer;
use aerugo::hal::drivers::uart::{
    Config as UartConfig, Error, Interrupt as UartInterrupt, NotConfigured, ParityBit,
    ReceiverConfig, UART,
};
use aerugo::hal::interrupt;
use aerugo::hal::user_peripherals::UART4;
use aerugo::time::RateExtU32;
use aerugo::Mutex;
use calldwell::write_str;
use heapless::Vec;

/// Test scenario:
/// * Test all methods available in unconfigured state
/// * Test state conversion methods
/// * Test state-specific configuration methods
/// * Test transmission and reception via loopback in blocking mode
/// * Test transmission and reception with test host in interrupt mode
pub fn test_uart(uart: UART<UART4, NotConfigured>, nvic: NVIC) {
    let uart = test_uart_configuration(uart);
    let uart = test_uart_state_transition(uart);
    let uart = test_reader_writer(uart);
    let uart = test_uart_local_loopback(uart);
    test_uart_io(uart, nvic);
    write_str("All UART functional tests finished successfully.");
}

fn test_uart_configuration(uart: UART<UART4, NotConfigured>) -> UART<UART4, NotConfigured> {
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

    // Check receiver configuration
    let mut uart = uart.into_receiver(
        expected_config,
        ReceiverConfig {
            rx_filter_enabled: true,
        },
    );
    assert!(uart.is_rx_filter_enabled());

    uart.set_rx_filter_state(false);
    assert!(!uart.is_rx_filter_enabled());
    uart.set_rx_filter_state(true);
    assert!(uart.is_rx_filter_enabled());

    write_str("UART configuration test finished successfully.");
    uart.disable()
}

fn test_uart_state_transition(uart: UART<UART4, NotConfigured>) -> UART<UART4, NotConfigured> {
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

    write_str("UART state transition test finished successfully.");
    uart.disable()
}

fn test_reader_writer(uart: UART<UART4, NotConfigured>) -> UART<UART4, NotConfigured> {
    let uart_config = UartConfig::new(115200, 12.MHz()).unwrap();
    let mut uart = uart.into_bidirectional(
        uart_config,
        ReceiverConfig {
            rx_filter_enabled: false,
        },
    );

    assert!(uart.has_reader());
    assert!(uart.has_writer());

    // Check if reader and writer can be taken from UART.
    let mut reader = uart.take_reader().expect("UART reader not available!");
    let mut writer = uart.take_writer().expect("UART writer not available!");

    assert!(!uart.has_reader());
    assert!(!uart.has_writer());

    // Disable UART and check if reader and writer will time-out.
    // I/O functions of reader/writer are tested in other test cases.
    let mut uart = uart.disable();

    assert_eq!(
        writer.transmit_byte(0xAA, 1_000_000).unwrap_err(),
        Error::TimedOut
    );
    assert_eq!(reader.receive_byte(1_000_000).unwrap_err(), Error::TimedOut);

    uart.put_reader(reader);
    assert!(uart.has_reader());
    assert!(!uart.has_writer());

    uart.put_writer(writer);
    assert!(uart.has_reader());
    assert!(uart.has_writer());

    write_str("UART Reader/Writer test finished successfully.");
    uart
}

fn test_uart_local_loopback(uart: UART<UART4, NotConfigured>) -> UART<UART4, NotConfigured> {
    let uart_config = UartConfig::new(115200, 12.MHz()).unwrap();
    let mut uart = uart.into_bidirectional(
        uart_config,
        ReceiverConfig {
            rx_filter_enabled: false,
        },
    );

    // Validate default state and loopback config
    assert_eq!(uart.loopback_mode(), LoopbackMode::None);
    uart.switch_to_local_loopback_mode();
    assert_eq!(uart.loopback_mode(), LoopbackMode::LocalLoopback);

    let mut reader = uart.take_reader().expect("UART reader not available!");
    let mut writer = uart.take_writer().expect("UART writer not available!");

    // Validate that all possible byte values can be transmitted via UART
    for byte in u8::MIN..=u8::MAX {
        writer.transmit_byte(byte, 100).unwrap();
        assert_eq!(reader.receive_byte(100).unwrap(), byte);
    }

    // Disabling UART also switches it to normal mode. Let's validate that.
    let mut uart = uart.disable().into_transmitter(uart_config);
    assert_eq!(uart.loopback_mode(), LoopbackMode::None);

    uart.put_reader(reader);
    uart.put_writer(writer);

    write_str("UART local loopback test finished successfully.");
    uart.disable()
}

/// This storage is dedicated to UART IRQ handler.
/// It must be initialized before enabling UART IRQ.
/// This storage can be safely accessed outside of UART IRQ code only when UART IRQ is disabled.
static mut UART_READER_STORAGE: Option<Reader<UART4>> = None;

/// Amount of data transmitted during the I/O tests.
/// Must be larger than the length of incoming handshake message.
const TEST_DATA_LENGTH: usize = 1024;

/// IRQ code should put all received data into this buffer.
static UART_RX_BUFFER: Mutex<RefCell<Vec<u8, TEST_DATA_LENGTH>>> =
    Mutex::new(RefCell::new(Vec::new()));

fn test_uart_io(uart: UART<UART4, NotConfigured>, mut nvic: NVIC) {
    let uart_config = UartConfig::new(57600, 12.MHz()).unwrap();
    let mut uart = uart.into_bidirectional(
        uart_config,
        ReceiverConfig {
            rx_filter_enabled: true,
        },
    );

    // Enable all RX-related interrupts.
    uart.disable_all_interrupts();
    uart.enable_interrupt(UartInterrupt::RxReady);
    uart.enable_interrupt(UartInterrupt::FramingError);
    uart.enable_interrupt(UartInterrupt::OverrunError);
    uart.enable_interrupt(UartInterrupt::ParityError);

    let reader = uart.take_reader().expect("UART reader not available!");
    let mut writer = uart.take_writer().expect("UART writer not available!");

    // Safety: This is safe, because UART IRQ is disabled.
    unsafe { UART_READER_STORAGE.replace(reader) };

    // From this point, accessing UART_READER_STORAGE from non-IRQ code is unsafe.
    nvic.enable(Interrupt::UART4);

    // Run all I/O tests now.
    perform_uart_handshake(&mut writer);
    test_data_reception();
    test_data_transmission(writer);

    // This should be the last test, so disable everything.
    nvic.disable(Interrupt::UART4);
    uart.disable_all_interrupts();
    uart.disable();
}

fn perform_uart_handshake(writer: &mut Writer<UART4>) {
    let expected_message = b"hello, this is the test suite handshake message!";
    let response = b"hello back, this is Aerugo's handshake message!";

    write_str("Waiting for handshake...");

    if !wait_for_handshake_message(expected_message) {
        UART_RX_BUFFER.lock(|buffer_ref| {
            let buffer = buffer_ref.borrow();
            panic!(
                "unexpected handshake message received, expected {:?}, got {:?}",
                expected_message,
                buffer.as_slice()
            )
        })
    }

    write_str("Handshake message received, responding...");

    writer
        .transmit_bytes(response, u32::MAX)
        .expect("transmitting UART handshake response has failed");

    write_str("Handshake done!");
}

fn wait_for_handshake_message(expected_message: &[u8]) -> bool {
    let timeout_limit: u32 = 1_000_000;
    let mut timeout_counter: u32 = 0;
    let mut handshake_received = false;
    while timeout_counter < timeout_limit && !handshake_received {
        UART_RX_BUFFER.lock(|buffer_ref| {
            let buffer = buffer_ref.get_mut();
            // We're looking for '!' character, as it indicates end of message.
            // This is a quickest check, as we want to make this critical section
            // as short as possible in case the reception is ongoing.
            if !buffer.contains(&b'!') {
                return;
            }

            if buffer.as_slice() == expected_message {
                handshake_received = true;
                buffer.clear();
            }
        });
        timeout_counter += 1;
    }

    handshake_received
}

fn test_data_reception() {
    let mut reception_successful = false;
    while !reception_successful {
        UART_RX_BUFFER.lock(|buffer_ref| {
            let buffer = buffer_ref.get_mut();

            // Early return, same reason as in `wait_for_handshake_message`
            if buffer.len() != TEST_DATA_LENGTH {
                return;
            }

            for (index, &value) in buffer.iter().enumerate() {
                if value != index as u8 {
                    panic!(
                        "Received invalid byte @ index {}, expected {}, got {}",
                        index, index as u8, value
                    );
                }
            }

            reception_successful = true;
        });
    }

    write_str("Data reception test successful!");
}

fn test_data_transmission(mut writer: Writer<UART4>) {
    let tx_data: Vec<u8, TEST_DATA_LENGTH> = (0..TEST_DATA_LENGTH).map(|i| i as u8).collect();
    writer.transmit_bytes(tx_data.as_slice(), u32::MAX).unwrap();
    write_str("Test data transmitted!");
}

#[interrupt]
fn UART4() {
    // Safety: This is safe as long as non-IRQ code doesn't access the reader while this IRQ is
    // enabled. Panicking when reader is not available is intended.
    let reader = unsafe { UART_READER_STORAGE.as_mut().unwrap() };

    let status = reader.status();

    if status.framing_error {
        panic!("UART framing error detected.")
    }

    if status.overrun_error {
        panic!("UART overrun error detected.")
    }

    if status.parity_error {
        panic!("UART parity error detected.")
    }

    if status.receiver_ready {
        UART_RX_BUFFER.lock(|buffer_ref| {
            buffer_ref
                .get_mut()
                // Safety: This is safe, because we verified that receiver is ready.
                .push(unsafe { reader.get_received_byte() })
                .unwrap()
        });
    }
}
