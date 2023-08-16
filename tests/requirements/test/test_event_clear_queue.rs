use assert_cmd::Command;
use test_binary::build_test_binary;

/// @SRS{ROS-FUN-RTOS-010}
/// @SRS{ROS-FUN-RTOS-130}
/// @SRS{ROS-FUN-RTOS-3010}
/// @SRS{ROS-FUN-RTOS-3020}
/// @SRS{ROS-FUN-RTOS-3060}
/// @SRS{ROS-FUN-RTOS-3070}
/// @SRS[ROS-FUN-RTOS-3080]
/// @SRS{ROS-FUN-RTOS-3090}
#[cfg_attr(not(doc), test)]
fn req_test_events() {
    let test_bin_path = build_test_binary("test-event-clear-queue", "testbins")
        .expect("error building test binary");

    Command::new(test_bin_path)
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .success()
        .code(0)
        .stdout(
            r#"TaskB: 42
TaskC: 42
TaskD: 42
TaskB: 13
TaskA
"#,
        );
}
