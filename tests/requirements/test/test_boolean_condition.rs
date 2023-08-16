use assert_cmd::Command;
use test_binary::build_test_binary;

/// @SRS{ROS-FUN-RTOS-010}
/// @SRS{ROS-FUN-RTOS-130}
/// @SRS{ROS-FUN-RTOS-1010}
/// @SRS{ROS-FUN-RTOS-1030}
/// @SRS{ROS-FUN-RTOS-1060}
/// @SRS{ROS-FUN-RTOS-1090}
/// @SRS{ROS-FUN-RTOS-1100}
#[cfg_attr(not(doc), test)]
fn req_test_boolean_condition() {
    let test_bin_path = build_test_binary("test-boolean-condition", "testbins")
        .expect("error building test binary");

    Command::new(test_bin_path)
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .success()
        .code(0)
        .stdout(
            r#"TaskA
TaskB: 1
TaskA
TaskB: 1
TaskA
TaskB: 1
TaskC
"#,
        );
}
