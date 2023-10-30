// Test scenario (SRS IDs in parentheses):
// * Check channel management (take/give) capabilities
// * Check interrupts and events configuration capabilities
// * Check transfer configuration capabilities
// * Check if mem2mem transfer can be performed (60, 70, 80)
// * Check if per2mem and mem2per transfers can be performed using UART in loopback mode (20, 30,
//   40, 50, 70, 80, 110)
// * Check if per2mem transfer can be suspended and disabled using UART in loopback mode (90, 100)

/// @SRS{ROS-FUN-BSP-XDMAC-020}
/// @SRS{ROS-FUN-BSP-XDMAC-030}
/// @SRS{ROS-FUN-BSP-XDMAC-040}
/// @SRS{ROS-FUN-BSP-XDMAC-050}
/// @SRS{ROS-FUN-BSP-XDMAC-060}
/// @SRS{ROS-FUN-BSP-XDMAC-070}
/// @SRS{ROS-FUN-BSP-XDMAC-080}
/// @SRS{ROS-FUN-BSP-XDMAC-090}
/// @SRS{ROS-FUN-BSP-XDMAC-100}
/// @SRS{ROS-FUN-BSP-XDMAC-110}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_hal_uart() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_hal_xdmac.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
