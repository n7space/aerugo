use core::slice;

use rtt_target::{DownChannel, UpChannel};

/// Enumeration representing stream markers
enum StreamMarkers {
    /// Start of stream
    Start = 0xDD,
    /// End of stream
    End = 0xDE,
}

/// Structure representing a streaming DownChannel.
/// Should never be constructed manually by the user.
pub struct DownStream {
    /// Stream's channel
    channel: DownChannel,
}

/// Enumeration representing DownStream errors.
pub enum ReceptionError {
    /// Provided buffer is too small for received data.
    BufferTooSmall,
    /// Received unexpected marker of stream start (received two start markers before end marker)
    UnexpectedStreamStartMarker,
}

/// Structure representing a streaming UpChannel.
/// Should never be constructed manually by the user.
///
/// # Generic arguments
/// * `State` - Current state of the stream.
pub struct UpStream {
    /// Stream's channel
    channel: UpChannel,
}

impl DownStream {
    /// Creates new idle stream from channel.
    pub(super) fn new(channel: DownChannel) -> Self {
        Self { channel }
    }

    /// Fetches a single byte via RTT channel. Does not block.
    ///
    /// # Returns
    /// `true` if data was copied to buffer, `false` otherwise.
    #[inline(always)]
    fn receive_byte(&mut self, buffer: &mut u8) -> bool {
        self.channel.read(slice::from_mut(buffer)) > 0
    }

    /// Waits until a start marker is received via RTT channel.
    fn wait_for_start_marker(&mut self) {
        let mut received_byte: u8 = 0; // assuming start marker's value is not 0

        while received_byte != (StreamMarkers::Start as u8) {
            self.receive_byte(&mut received_byte);
        }
    }

    /// Blocks until a valid Calldwell stream is received from test host.
    ///
    /// This function assumes that start marker was already detected, and should be
    /// used only in that scenario - for example, to receive remaining stream data into
    /// a buffer that was too small to accommodate whole stream in previous reception.
    ///
    /// If this function returns [`ReceptionError::BufferTooSmall`], you can still recover
    /// the complete message by calling it again, until it returns an `Ok`, assuming that
    /// a valid end-of-stream marker will eventually come.
    ///
    /// # Parameters
    /// * `buffer` - Buffer for received data.
    ///
    /// # Returns
    /// `Ok(usize)` with length of received data, or `ReceptionError` if communication error has happened,
    /// or the buffer was too small to contain received data.
    pub fn finish_reception(&mut self, buffer: &mut [u8]) -> Result<usize, ReceptionError> {
        let mut received_bytes: usize = 0;
        let mut received_byte: u8 = 0;

        // Assuming that start marker was already found, read until you either find end of stream
        // or fill the buffer.
        loop {
            // Read into temporary buffer to avoid reducing user's buffer effective capacity by 1 byte.
            // Otherwise, last byte would be wasted on end marker.
            // Might be a bit slower, as additional copy is performed, but we don't care for now.
            if self.receive_byte(&mut received_byte) {
                // Double start marker detected
                if received_byte == (StreamMarkers::Start as u8) {
                    return Err(ReceptionError::UnexpectedStreamStartMarker);
                }

                if received_byte == (StreamMarkers::End as u8) {
                    return Ok(received_bytes);
                }

                if received_bytes != buffer.len() {
                    buffer[received_bytes] = received_byte;
                    received_bytes += 1;
                } else {
                    // Buffer is filled, but there might be more data incoming.
                    // Read this function's description to learn how you can handle this case.
                    return Err(ReceptionError::BufferTooSmall);
                }
            }
        }
    }

    /// Blocks until a valid Calldwell stream is received from test host.
    /// Automatically detects start and end marker. This is probably the function
    /// you're looking for.
    ///
    /// If this function returns [`ReceptionError::BufferTooSmall`], you can still recover the
    /// complete message by calling [`finish_reception`], which won't perform start marker detection,
    /// and receive data until an end marker is found, or buffer is filled again, in which case you
    /// have to repeat this process. Otherwise, you can call `receive` again to wait until next valid
    /// start marker is received.
    ///
    /// # Parameters
    /// * `buffer` - Buffer for received data.
    ///
    /// # Returns
    /// `Ok(usize)` with length of received data, or `ReceptionError` if communication error has happened,
    /// or the buffer was too small to contain received data.
    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, ReceptionError> {
        self.wait_for_start_marker();
        self.finish_reception(buffer)
    }
}

impl UpStream {
    /// Creates new idle stream from channel.
    pub(super) fn new(channel: UpChannel) -> Self {
        Self { channel }
    }
}
