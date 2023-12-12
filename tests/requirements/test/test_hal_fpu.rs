// Test scenario
// * Check FPU enabling and if lazy stacking is enabled by default.
// * Check if FPU context defaults can be configured (NaN operation, Flush-to-zero, compliance with IEEE standard).
// * Check if current FPU context can be configured (NaN operation, Flush-to-zero, compliance with IEEE standard).
// * Check if current FPU state can be fetched (for handling invalid operands).

/// @SRS{ROS-FUN-BSP-FPU-020}
/// @SRS{ROS-FUN-BSP-FPU-030}
/// @SRS{ROS-FUN-BSP-FPU-040}
/// @SRS{ROS-FUN-BSP-FPU-050}
/// @SRS{ROS-FUN-BSP-FPU-060}
/// @SRS{ROS-FUN-BSP-FPU-070}
#[cfg_attr(not(doc), test)]
#[cfg(feature = "test-aerugo-cortex-m")]
fn req_test_hal_fpu() {
    use assert_cmd::Command;

    // The script will build test binary
    Command::new("python")
        .arg("tests/requirements/test/test_hal_fpu.py")
        .timeout(std::time::Duration::from_secs(60))
        .assert()
        .success()
        .code(0);
}
