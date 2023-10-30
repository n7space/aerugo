use assert_cmd::Command;
use test_binary::build_test_binary;

/// @SRS{ROS-FUN-RTOS-5010}
/// @SRS{ROS-FUN-RTOS-5020}
/// @SRS{ROS-FUN-RTOS-5030}
/// @SRS{ROS-FUN-RTOS-5050}
/// @SRS{ROS-FUN-RTOS-5060}
#[cfg_attr(not(doc), test)]
fn req_test_execution_monitoring() {
    let test_bin_path = build_test_binary("test-execution-monitoring", "testbins")
        .expect("error building test binary");

    Command::new(test_bin_path)
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .success()
        .code(0)
        .stdout(
            r"Tasklet: #1
Min: 20
Max: 20
Avg: 20
Tasklet: #2
Min: 50
Max: 50
Avg: 50
Tasklet: #0
Min: 0
Max: 0
Avg: 0
Tasklet: #1
Min: 10
Max: 20
Avg: 15
Tasklet: #2
Min: 20
Max: 50
Avg: 35
",
        );
}
