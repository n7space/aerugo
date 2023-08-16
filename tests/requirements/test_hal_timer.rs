use assert_cmd::Command;
// use test_binary::build_test_binary;

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
