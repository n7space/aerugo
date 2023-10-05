/// @SRS{ROS-FUN-RTOS-2010}
/// @SRS{ROS-FUN-RTOS-2040}
/// @SRS{ROS-FUN-RTOS-2060}
/// @SRS{ROS-FUN-RTOS-2070}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_boolean_condition_interrupt() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_message_queue_interrupt.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
