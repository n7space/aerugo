use aerugo::hal::drivers::xdmac::{
    channel::{Channel, NotConfigured},
    events::ChannelEvents,
    Xdmac,
};
use calldwell::write_str;

pub fn perform_test(xdmac: &mut Xdmac) {
    write_str("Performing channel management tests...");

    let channel = xdmac.take_next_free_channel().unwrap();
    let channel = check_status_reader(channel);
    let channel = check_irq_configuration(channel);
    let channel = check_events_configuration(channel);

    xdmac.return_channel(channel);

    write_str("Channel management tests finished successfully!");
}

fn check_status_reader(mut channel: Channel<NotConfigured>) -> Channel<NotConfigured> {
    assert!(channel.is_status_reader_available());
    let status_reader = channel.take_status_reader().unwrap();
    assert!(!channel.is_status_reader_available());
    assert_eq!(status_reader.id(), channel.id());
    channel.return_status_reader(status_reader);
    assert!(channel.is_status_reader_available());

    // Status reader capabilities are tested in functional tests.
    write_str("Channel status reader management test finished successfully.");
    channel
}

fn check_irq_configuration(mut channel: Channel<NotConfigured>) -> Channel<NotConfigured> {
    channel.disable_interrupt();
    assert!(!channel.is_interrupt_enabled());
    channel.enable_interrupt();
    assert!(channel.is_interrupt_enabled());
    channel.disable_interrupt();
    assert!(!channel.is_interrupt_enabled());

    write_str("Channel IRQ configuration test finished successfully.");
    channel
}

fn check_events_configuration(mut channel: Channel<NotConfigured>) -> Channel<NotConfigured> {
    let events_none = ChannelEvents {
        end_of_block: false,
        end_of_list: false,
        end_of_disable: false,
        end_of_flush: false,
        read_bus_error: false,
        write_bus_error: false,
        request_overflow_error: false,
    };

    let events_all = ChannelEvents {
        end_of_block: true,
        end_of_list: true,
        end_of_disable: true,
        end_of_flush: true,
        read_bus_error: true,
        write_bus_error: true,
        request_overflow_error: true,
    };

    let events_some_a = ChannelEvents {
        end_of_block: true,
        end_of_list: false,
        end_of_disable: true,
        end_of_flush: false,
        read_bus_error: true,
        write_bus_error: false,
        request_overflow_error: true,
    };

    let events_some_b = ChannelEvents {
        end_of_block: false,
        end_of_list: true,
        end_of_disable: false,
        end_of_flush: true,
        read_bus_error: false,
        write_bus_error: true,
        request_overflow_error: false,
    };

    let events_sequence = [
        events_none,
        events_all,
        events_some_a,
        events_some_b,
        events_none,
    ];

    for events in events_sequence {
        channel.set_events_state(events);
        assert_eq!(channel.events_state(), events);
    }

    write_str("Channel events configuration test finished successfully.");
    channel
}
