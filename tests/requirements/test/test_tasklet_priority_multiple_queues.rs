use assert_cmd::Command;
use test_binary::build_test_binary;

/// @SRS{ROS-FUN-RTOS-010}
/// @SRS{ROS-FUN-RTOS-090}
/// @SRS{ROS-FUN-RTOS-100}
/// @SRS{ROS-FUN-RTOS-110}
#[cfg_attr(not(doc), test)]
fn req_test_tasklet_priority_multiple_queues() {
    let test_bin_path = build_test_binary("test-tasklet-priority-multiple-queues", "testbins")
        .expect("error building test binary");

    Command::new(test_bin_path)
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .success()
        .code(0)
        .stdout(
            r#"TaskB: 1
TaskB: 2
TaskC: 1
TaskB: 3
TaskB: 4
TaskC: 2
TaskB: 5
TaskB: 6
TaskC: 3
TaskC: 4
TaskC: 5
"#,
        );
}
