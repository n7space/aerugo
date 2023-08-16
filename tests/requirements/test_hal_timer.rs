use assert_cmd::Command;

// Test scenario:
// - Configure Timer to use non-default clock source
// - Enable timer's interrupt, and count it's overflows
// - Enable and start timer's clock
// - Check if IRQ count is increasing
// - Stop and disable the timer
// - Check if IRQ count stopped increasing
// - Change timer's clock source, enable and start it
// - Check if IRQ rate changed compared to previous check

/// @SRS{ROS-FUN-BSP-TIC-020}
/// @SRS{ROS-FUN-BSP-TIC-030}
/// @SRS{ROS-FUN-BSP-TIC-040}
/// @SRS{ROS-FUN-BSP-TIC-050}
/// @SRS{ROS-FUN-BSP-TIC-060}
/// @SRS{ROS-FUN-BSP-TIC-070}
#[cfg_attr(not(doc), test)]
#[ignore]
fn req_test_hal_timer() {
    // TODO: Fix building the binary
    // build_test_binary("test-hal-timer", "testbins").expect("error building test binary");

    Command::new("python")
        .arg("tests/requirements/test_hal_timer.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
