use assert_cmd::Command;
// use test_binary::build_test_binary;

/// @SRS{ROS-FUN-BSP-WDT-020}
/// @SRS{ROS-FUN-BSP-WDT-030}
#[cfg_attr(not(doc), test)]
#[ignore]
fn req_test_hal_watchdog() {
    // TODO: Fix building the binary
    // build_test_binary("test-hal-watchdog", "testbins").expect("error building test binary");

    Command::new("python")
        .arg("tests/requirements/test_hal_watchdog.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
