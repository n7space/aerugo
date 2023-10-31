use assert_cmd::Command;
use test_binary::build_test_binary;

/// @SRS{ROS-FUN-RTOS-120}
#[cfg_attr(not(doc), test)]
fn req_test_tasklet_not_subscribed() {
    let test_bin_path = build_test_binary("test-tasklet-not-subscribed", "testbins")
        .expect("error building test binary");

    Command::new(test_bin_path)
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .failure()
        .code(101);
}
