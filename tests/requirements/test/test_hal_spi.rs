// Test scenario
// * Check SPI configuration capabilities
// * Check SPI in blocking mode via loopback
// * Check SPI in interrupt mode via loopback
// * Check SPI with XDMAC by performing transaction with LSM6DSO sensor

/// @SRS{ROS-FUN-BSP-SPI-020}
/// @SRS{ROS-FUN-BSP-SPI-030}
/// @SRS{ROS-FUN-BSP-SPI-040}
/// @SRS{ROS-FUN-BSP-SPI-050}
/// @SRS{ROS-FUN-BSP-SPI-060}
/// @SRS{ROS-FUN-BSP-SPI-070}
/// @SRS{ROS-FUN-BSP-SPI-080}
/// @SRS{ROS-FUN-BSP-SPI-090}
/// @SRS{ROS-FUN-BSP-SPI-100}
/// @SRS{ROS-FUN-BSP-SPI-110}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_hal_spi() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_hal_spi.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
