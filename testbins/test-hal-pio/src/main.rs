#![no_std]
#![no_main]

extern crate aerugo;
extern crate calldwell;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use aerugo::{
    hal::drivers::{
        embedded_hal::digital::{OutputPin, StatefulOutputPin},
        pio::{
            pin::{DriveMode, OutputMode, PinMode, PinState, PullResistor},
            IoPortMetadata, Pin, Port, SynchronousPort, PIOC,
        },
        pmc::config::PeripheralId,
    },
    Aerugo, InitApi, SystemHardwareConfig,
};
use calldwell::with_rtt_out;
use rt::entry;

fn validate_pin_is_driven_high(pin: &Pin<OutputMode>) {
    if !pin.is_set_high().unwrap() {
        panic!("Pin is driven high, yet `is_set_high` returns `false`");
    }
    if pin.is_set_low().unwrap() {
        panic!("Pin is driven high, yet `is_set_low` returns `true`");
    }
    if !pin.is_high() {
        panic!("Pin is driven high, yet `is_high` returns `false`");
    }
    if pin.is_low() {
        panic!("Pin is driven high, yet `is_low` returns `true`");
    }
    if pin.state() != PinState::High {
        panic!("Pin is driven high, yet `state` returns `PinState::Low`");
    }
}

fn validate_pin_is_driven_low(pin: &Pin<OutputMode>) {
    if pin.is_set_high().unwrap() {
        panic!("Pin is driven low, yet `is_set_high` returns `true`");
    }
    if !pin.is_set_low().unwrap() {
        panic!("Pin is driven low, yet `is_set_low` returns `false`");
    }
    if pin.is_high() {
        panic!("Pin is driven low, yet `is_high` returns `true`");
    }
    if !pin.is_low() {
        panic!("Pin is driven low, yet `is_low` returns `false`");
    }
    if pin.state() != PinState::Low {
        panic!("Pin is driven low, yet `state` returns `PinState::High`");
    }
}

/// Output pin mode is chosen because it provides all pin's functions.
fn check_pin_functionality(pin: &mut Pin<OutputMode>) {
    pin.switch_to_push_pull_mode();

    pin.set_high().unwrap();
    validate_pin_is_driven_high(pin);
    pin.set_low().unwrap();
    validate_pin_is_driven_low(pin);

    pin.set_state(PinState::High);
    validate_pin_is_driven_high(pin);
    pin.set_state(PinState::Low);
    validate_pin_is_driven_low(pin);
}

fn perform_basic_pin_functions_test(mut pin: Pin<OutputMode>) {
    check_pin_functionality(&mut pin);
    with_rtt_out(|w, _| w.write_str("basic pin functions test successful"));
}

fn check_pull_resistor_config<Mode: PinMode>(pin: &Pin<Mode>, expected_resistor: PullResistor) {
    let current_resistor = pin.pull_resistor();
    if current_resistor != PullResistor::None {
        panic!(
            "pin reports {:#?} after setting it to {:#?}",
            current_resistor, expected_resistor
        );
    }
}

fn test_pull_resistor_config<Mode: PinMode>(pin: &mut Pin<Mode>, wanted_resistor: PullResistor) {
    pin.set_pull_resistor(wanted_resistor);
    check_pull_resistor_config(pin, wanted_resistor);
}

fn perform_pull_resistors_test<Mode: PinMode>(mut pin: Pin<Mode>) {
    test_pull_resistor_config(&mut pin, PullResistor::None);
    test_pull_resistor_config(&mut pin, PullResistor::Down);
    test_pull_resistor_config(&mut pin, PullResistor::Up);

    // Sanity check - to make sure that all functions behave correctly
    pin.disable_pull_resistor();
    check_pull_resistor_config(&pin, PullResistor::None);

    pin.pull_up();
    check_pull_resistor_config(&pin, PullResistor::Up);

    pin.pull_down();
    check_pull_resistor_config(&pin, PullResistor::Down);

    pin.disable_pull_resistor();

    with_rtt_out(|w, _| w.write_str("pull resistors config test successful"));
}

fn validate_port_state<Port: IoPortMetadata, const N: usize>(
    port: &SynchronousPort<Port, N>,
    expected_mask: u32,
) {
    let state = port.output_state();
    if state != expected_mask {
        panic!("all synchronous bits driven high, but `output_state` reports otherwise (expected {:#012b}, got {:#012b}", expected_mask, state);
    }
}

fn perform_synchronous_port_test(pins: [Pin<OutputMode>; 4], port_mask: u32, partial_mask: u32) {
    let mut port: SynchronousPort<PIOC, 4> = SynchronousPort::new(pins).unwrap();

    // Check indexed access
    check_pin_functionality(&mut port[2]);

    // Check port access
    port.set_state(u32::MAX);
    validate_port_state(&port, port_mask);

    port.set_state(0);
    validate_port_state(&port, 0);

    port.set_state(partial_mask);
    validate_port_state(&port, partial_mask);

    port.set_state(!partial_mask);
    validate_port_state(&port, !partial_mask);

    port.set_state(partial_mask | 0xBEEF0000);
    validate_port_state(&port, partial_mask);

    port.set_state(port_mask);
    validate_port_state(&port, port_mask);

    port.set_masked_state(u32::MAX, u32::MAX);
    validate_port_state(&port, port_mask);

    port.set_masked_state(u32::MAX, 0);
    validate_port_state(&port, port_mask);

    port.set_masked_state(0, u32::MAX);
    validate_port_state(&port, 0);

    port.set_masked_state(partial_mask, partial_mask);
    validate_port_state(&port, partial_mask);

    port.set_masked_state(!partial_mask, !partial_mask);
    validate_port_state(&port, port_mask);

    with_rtt_out(|w, _| w.write_str("synchronous pin access test successful"));
}

fn validate_pin_is_push_pull(pin: &Pin<OutputMode>) {
    if !pin.in_push_pull_mode() {
        panic!("pin is in push-pull mode, but `in_push_pull_mode` returns `false");
    }
    if pin.in_open_drain_mode() {
        panic!("pin is in push-pull mode, but `in_open_drain_mode` returns `true");
    }
    if pin.drive_mode() != DriveMode::PushPull {
        panic!("pin is in push-pull mode, but `drive_mode` returns `DriveMode::OpenDrain`");
    }
}

fn validate_pin_is_open_drain(pin: &Pin<OutputMode>) {
    if pin.in_push_pull_mode() {
        panic!("pin is in push-pull mode, but `in_push_pull_mode` returns `true");
    }
    if !pin.in_open_drain_mode() {
        panic!("pin is in push-pull mode, but `in_open_drain_mode` returns `false");
    }
    if pin.drive_mode() != DriveMode::OpenDrain {
        panic!("pin is in push-pull mode, but `drive_mode` returns `DriveMode::PushPull`");
    }
}

fn perform_open_drain_test(mut pin: Pin<OutputMode>) {
    pin.switch_to_push_pull_mode();
    validate_pin_is_push_pull(&pin);
    pin.switch_to_open_drain_mode();
    validate_pin_is_open_drain(&pin);
    pin.set_drive_mode(DriveMode::PushPull);
    validate_pin_is_push_pull(&pin);
    pin.set_drive_mode(DriveMode::OpenDrain);
    validate_pin_is_open_drain(&pin);

    with_rtt_out(|w, _| w.write_str("push-pull/open-drain config test successful"));
}

fn perform_pio_test<PortMeta: IoPortMetadata>(port: Port<PortMeta>) {
    let mut pins = port.into_pins();

    perform_basic_pin_functions_test(pins[15].take().unwrap().into_output_pin());
    perform_pull_resistors_test(pins[0].take().unwrap());
    perform_synchronous_port_test(
        [
            pins[5].take().unwrap().into_output_pin(),
            pins[6].take().unwrap().into_output_pin(),
            pins[7].take().unwrap().into_output_pin(),
            pins[8].take().unwrap().into_output_pin(),
        ],
        0b0111100000,
        0b0010100000,
    );
    perform_open_drain_test(pins[20].take().unwrap().into_output_pin());

    with_rtt_out(|w, _| w.write_str("all tests finished successfully"));
}

#[entry]
fn main() -> ! {
    calldwell::start_session();
    let (aerugo, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());
    let mut pmc = peripherals.pmc.take().unwrap();
    pmc.enable_peripheral_clock(PeripheralId::PIOC);
    perform_pio_test(Port::new(peripherals.pio_c.take().unwrap()));
    aerugo.start();
}
