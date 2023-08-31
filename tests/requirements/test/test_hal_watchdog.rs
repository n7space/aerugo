// Test scenario:
// - Configure Aerugo with watchdog that will reset the MCU after 5 seconds
// - Execute a task that will run shorter than 5 seconds and send a message to host
// - Execute a task that will run longer than 5 seconds
// - Validate that MCU has rebooted

/// @SRS{ROS-FUN-BSP-WDT-020}
/// @SRS{ROS-FUN-BSP-WDT-030}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_hal_watchdog() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_hal_watchdog.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
