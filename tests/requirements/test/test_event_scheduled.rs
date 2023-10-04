use assert_cmd::Command;
use test_binary::build_test_binary;

/// @SRS{ROS-FUN-RTOS-010}
/// @SRS{ROS-FUN-RTOS-130}
/// @SRS{ROS-FUN-RTOS-3010}
/// @SRS{ROS-FUN-RTOS-3030}
/// @SRS{ROS-FUN-RTOS-3070}
/// @SRS[ROS-FUN-RTOS-3080]
/// @SRS{ROS-FUN-RTOS-3090}
#[cfg_attr(not(doc), test)]
fn req_test_event_scheduled() {
    let test_bin_path =
        build_test_binary("test-event-scheduled", "testbins").expect("error building test binary");

    Command::new(test_bin_path)
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .success()
        .code(0)
        .stdout(
            r"Current time is: 3s; Got event
",
        );
}
