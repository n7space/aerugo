// Test scenario:
// - Verify that an interrupt can be masked and unmasked.
// - Verify that an interrupt can be triggered via software and behaves correctly while masked.
// - Verify that an interrupt can be pend and unpend.
// - Verify that interrupt's priority can be changed and that interrupts with different priorities
//   behave as expected.

/// @SRS{ROS-FUN-BSP-NVIC-020}
/// @SRS{ROS-FUN-BSP-NVIC-030}
/// @SRS{ROS-FUN-BSP-NVIC-040}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_hal_nvic() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_hal_nvic.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
