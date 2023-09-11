// Test scenario:
// - Verify that PMC status can be read correctly
// - Verify that interrupts can be configured
// - Verify that internal RC oscillator frequency can be changed and measured
// - Verify that master clock configuration can be changed
// - Verify that processor clock status can be read
// - Verify that programmable clocks can be configured
// - Verify that peripheral clocks can be configured and their configuration is retained

/// @SRS{ROS-FUN-BSP-PMC-020}
/// @SRS{ROS-FUN-BSP-PMC-030}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_hal_pmc() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_hal_pmc.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
