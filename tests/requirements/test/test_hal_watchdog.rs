use assert_cmd::Command;

// Test scenario:
// - Configure Aerugo with watchdog that will reset the MCU after 3 seconds
// - Execute a task that will run shorter than 3 seconds and send a message to host
// - Execute a task that will run longer than 3 seconds
// - Validate that MCU has rebooted

/// @SRS{ROS-FUN-BSP-WDT-020}
/// @SRS{ROS-FUN-BSP-WDT-030}
#[cfg_attr(not(doc), test)]
#[ignore]
fn req_test_hal_watchdog() {
    // TODO: Fix building the binary
    // build_test_binary("test-hal-watchdog", "testbins").expect("error building test binary");

    Command::new("python")
        .arg("tests/requirements/test/test_hal_watchdog.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
