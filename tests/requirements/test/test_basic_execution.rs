use assert_cmd::Command;
use test_binary::build_test_binary;

/// @SRS{ROS-FUN-RTOS-10}
/// @SRS{ROS-FUN-RTOS-110}
#[cfg_attr(not(doc), test)]
fn req_test_basic_execution() {
    let test_bin_path =
        build_test_binary("test-basic-execution", "testbins").expect("error building test binary");

    Command::new(test_bin_path)
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .success()
        .code(0)
        .stdout(
            r#"TaskA
TaskB
TaskA
TaskB
"#,
        );
}
