// Test scenario:
// - Verify that I-Cache can be disabled/enabled/invalidated
// - Verify that D-Cache can be disabled/enabled/cleared/invalidated

/// @SRS{ROS-FUN-BSP-SCB-020}
/// @SRS{ROS-FUN-BSP-SCB-030}
/// @SRS{ROS-FUN-BSP-SCB-040}
/// @SRS{ROS-FUN-BSP-SCB-050}
/// @SRS{ROS-FUN-BSP-SCB-060}
/// @SRS{ROS-FUN-BSP-SCB-070}
/// @SRS{ROS-FUN-BSP-SCB-080}
/// @SRS{ROS-FUN-BSP-SCB-090}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_hal_scb() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_hal_scb.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
