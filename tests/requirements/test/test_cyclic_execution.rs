/// @SRS{ROS-FUN-RTOS-5010}
/// @SRS{ROS-FUN-RTOS-5020}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_cyclic_execution() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_cyclic_execution.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
