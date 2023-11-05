use aerugo::hal::drivers::xdmac::Xdmac;
use calldwell::write_str;
use heapless::Vec;

const EXPECTED_NUMBER_OF_PERIPHERAL_REQUESTS: usize = 52;
const EXPECTED_FIFO_SIZE: usize = 128;
const EXPECTED_AVAILABLE_CHANNELS: usize = 24;

pub fn perform_test(xdmac: &mut Xdmac) {
    write_str("Performing XDMAC management tests...");
    check_xdmac_info(xdmac);
    check_status_reader(xdmac);
    check_channel_taking(xdmac);
    write_str("XDMAC management test finished successfully.");
}

fn check_xdmac_info(xdmac: &Xdmac) {
    assert_eq!(
        xdmac.number_of_peripheral_requests(),
        EXPECTED_NUMBER_OF_PERIPHERAL_REQUESTS,
        "unexpected amount of peripheral requests available"
    );
    assert_eq!(
        xdmac.fifo_size(),
        EXPECTED_FIFO_SIZE,
        "unexpected FIFO size detected"
    );
    assert_eq!(
        xdmac.available_channels(),
        EXPECTED_AVAILABLE_CHANNELS,
        "unexpected amount of available channels"
    );

    write_str("XDMAC info test finished successfully.");
}

fn check_status_reader(xdmac: &mut Xdmac) {
    assert!(
        xdmac.is_status_reader_available(),
        "status reader has been taken out of Xdmac and not given back"
    );

    let mut status_reader = xdmac.take_status_reader().unwrap();

    // There should be no pending channels (as all IRQs should be disabled)
    assert_eq!(
        status_reader.get_pending_channels(),
        Vec::<_, { Xdmac::SUPPORTED_CHANNELS }>::from_slice(&[false; Xdmac::SUPPORTED_CHANNELS])
            .unwrap(),
        "unexpectedly, there are channels with pending interrupts"
    );

    assert!(
        !xdmac.is_status_reader_available(),
        "status reader was taken, but it seems available"
    );
    xdmac.return_status_reader(status_reader);

    assert!(
        xdmac.is_status_reader_available(),
        "status reader has been taken out of Xdmac and not given back"
    );

    write_str("XDMAC status reader test finished successfully.");
}

fn check_channel_taking(xdmac: &mut Xdmac) {
    // At this point, all the channels should be available.
    for id in 0..Xdmac::SUPPORTED_CHANNELS {
        assert!(
            xdmac.is_channel_available(id),
            "channel with ID {} is unavailable",
            id
        );
    }

    // Take first available channel - should have ID 0
    let channel = xdmac.take_next_free_channel().unwrap();
    assert_eq!(channel.id(), 0);

    // Check if next taken channel will have ID 1
    let next_channel = xdmac.take_next_free_channel().unwrap();
    assert_eq!(next_channel.id(), 1);

    // Check if both those channels are taken
    assert!(!xdmac.is_channel_available(0));
    assert!(!xdmac.is_channel_available(1));

    // Check if it's not possible to take already taken channel
    let taken_channel = xdmac.take_channel(0);
    assert!(taken_channel.is_none());

    // Check if it's possible to manually take a free channel
    let taken_channel = xdmac.take_channel(10).unwrap();
    assert_eq!(taken_channel.id(), 10);

    // Put all channels back
    xdmac.return_channel(channel);
    xdmac.return_channel(next_channel);
    xdmac.return_channel(taken_channel);

    // Verify that they have been returned
    for id in 0..Xdmac::SUPPORTED_CHANNELS {
        assert!(
            xdmac.is_channel_available(id),
            "channel with ID {} is unavailable",
            id
        );
    }

    // Take channel and mark it as "free". Check if it can be taken again.
    // This requires a new scope to be safe, as the channel should never be marked as "free"
    // if it's object exists.
    {
        xdmac.take_channel(5).unwrap();
    }

    assert!(!xdmac.is_channel_available(5));
    unsafe { xdmac.mark_channel_as_free(5) };
    assert!(xdmac.is_channel_available(5));

    write_str("XDMAC channel take/give test finished successfully.");
}
