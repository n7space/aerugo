use assert_cmd::Command;
use test_binary::build_test_binary;

/// @SRS{ROS-FUN-RTOS-5040}
#[cfg_attr(not(doc), test)]
fn req_test_execution_time_exceeded() {
    let test_bin_path = build_test_binary("test-execution-time-exceeded", "testbins")
        .expect("error building test binary");

    Command::new(test_bin_path)
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .success()
        .code(0)
        .stdout(
            r"Running 20ms
Running 150ms
Time exceeded
",
        );
}
