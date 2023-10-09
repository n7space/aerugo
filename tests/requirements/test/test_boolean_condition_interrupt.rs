/// @SRS{ROS-FUN-RTOS-1010}
/// @SRS{ROS-FUN-RTOS-1040}
/// @SRS{ROS-FUN-RTOS-1050}
/// @SRS{ROS-FUN-RTOS-1060}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_boolean_condition_interrupt() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_boolean_condition_interrupt.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
