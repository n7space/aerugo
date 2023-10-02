// Test scenario:
// - Check systick configurability
// - Validate that systick IRQ frequency can be changed

/// @SRS{ROS-FUN-BSP-SYST-020}
/// @SRS{ROS-FUN-BSP-SYST-030}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_hal_systick() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_hal_systick.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
