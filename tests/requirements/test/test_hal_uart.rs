// Test scenario:
// - Verify UART configuration capabilities
// - Verify UART state transitions (disabled/receiver/transmitter/bidirectional)
// - Verify generic UART functions (retrieving status (errors), taking Reader/Writer instances)
// - Verify UART local loopback in synchronous mode
// - Verify UART operation with external device in asynchronous mode

/// @SRS{ROS-FUN-BSP-UART-020}
/// @SRS{ROS-FUN-BSP-UART-030}
/// @SRS{ROS-FUN-BSP-UART-040}
/// @SRS{ROS-FUN-BSP-UART-050}
/// @SRS{ROS-FUN-BSP-UART-060}
/// @SRS{ROS-FUN-BSP-UART-070}
/// @SRS{ROS-FUN-BSP-UART-080}
/// @SRS{ROS-FUN-BSP-UART-090}
/// @SRS{ROS-FUN-BSP-UART-120}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_hal_uart() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_hal_uart.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
