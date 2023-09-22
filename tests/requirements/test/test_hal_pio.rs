// Test scenario:
// - Verify that pin's state can be changed and read (in polling mode)
// - Verify that pin's pull resistors configuration can be changed and read
// - Verify that pins can be used in synchronous mode safely
// - Verify that pins can be switched between push-pull and open-drain mode

/// @SRS{ROS-FUN-BSP-PIO-020}
/// @SRS{ROS-FUN-BSP-PIO-030}
/// @SRS{ROS-FUN-BSP-PIO-040}
/// @SRS{ROS-FUN-BSP-PIO-050}
/// @SRS{ROS-FUN-BSP-PIO-060}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_hal_pio() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_hal_pio.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
