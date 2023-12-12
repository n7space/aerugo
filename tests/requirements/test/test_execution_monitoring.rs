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

    let command = Command::new(test_bin_path)
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .success()
        .code(0);

    let stdout = &command.get_output().stdout;
    let output = stdout.iter().map(|&c| c as char).collect::<String>();
    let output_lines = output.lines().collect::<Vec<&str>>();

    // Because of the fact that this test runs on x86, where hard real-time is usually not a thing,
    // there must be some tolerance for timing validated by this test.
    // Considering that the execution times here are short, the margin for error is experimentally
    // designated to be very wide, to avoid false-positive test failures due to OS scheduling.
    // In other words, this test is very unreliable on x86 and checks only the existence and API
    // of execution monitoring, as precise validation of execution times is simply not possible due
    // to how OS scheduling works.

    let expected_time_range = 100u32..=300;
    let exec_time = parse_output_line(output_lines[0]);
    assert_eq!(exec_time.tasklet_id, 1);
    assert!(
        expected_time_range.contains(&exec_time.min),
        "Expected {:?}, got {}",
        expected_time_range,
        exec_time.min
    );
    assert!(
        expected_time_range.contains(&exec_time.max),
        "Expected {:?}, got {}",
        expected_time_range,
        exec_time.max
    );
    assert!(
        expected_time_range.contains(&exec_time.avg),
        "Expected {:?}, got {}",
        expected_time_range,
        exec_time.avg
    );

    // Second line should contain approx 500ms execution of tasklet 2
    let expected_time_range = 500u32..=650;
    let exec_time = parse_output_line(output_lines[1]);
    assert_eq!(exec_time.tasklet_id, 2);
    assert!(
        expected_time_range.contains(&exec_time.min),
        "Expected {:?}, got {}",
        expected_time_range,
        exec_time.min
    );
    assert!(
        expected_time_range.contains(&exec_time.max),
        "Expected {:?}, got {}",
        expected_time_range,
        exec_time.max
    );
    assert!(
        expected_time_range.contains(&exec_time.avg),
        "Expected {:?}, got {}",
        expected_time_range,
        exec_time.avg
    );

    // Third line should contain exactly 0ms execution of tasklet 0, as it wasn't executed
    let exec_time = parse_output_line(output_lines[2]);
    assert_eq!(exec_time.tasklet_id, 0);
    assert_eq!(exec_time.min, 0, "Expected 0, got {}", exec_time.min);
    assert_eq!(exec_time.max, 0, "Expected 0, got {}", exec_time.max);
    assert_eq!(exec_time.avg, 0, "Expected 0, got {}", exec_time.avg);

    // Fourth line contains second exec info from tasklet 1. It takes 100ms second time.
    let exec_time = parse_output_line(output_lines[3]);
    let min_range = 100..=200;
    let max_range = 200..=500;
    let avg_range = 100..=500;
    assert_eq!(exec_time.tasklet_id, 1);
    assert!(
        min_range.contains(&exec_time.min),
        "Expected {:?}, got {}",
        min_range,
        exec_time.min
    );
    assert!(
        max_range.contains(&exec_time.max),
        "Expected {:?}, got {}",
        max_range,
        exec_time.max
    );
    assert!(
        avg_range.contains(&exec_time.avg),
        "Expected {:?}, got {}",
        avg_range,
        exec_time.avg
    );

    // Fifth line contains second exec info from tasklet 2. It takes 200ms second time.
    let exec_time = parse_output_line(output_lines[4]);
    let min_range = 200..=260;
    let max_range = 500..=650;
    let avg_range = 200..=650;
    assert_eq!(exec_time.tasklet_id, 2);
    assert!(
        min_range.contains(&exec_time.min),
        "Expected {:?}, got {}",
        min_range,
        exec_time.min
    );
    assert!(
        max_range.contains(&exec_time.max),
        "Expected {:?}, got {}",
        max_range,
        exec_time.max
    );
    assert!(
        avg_range.contains(&exec_time.avg),
        "Expected {:?}, got {}",
        avg_range,
        exec_time.avg
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ExecutionTime {
    pub tasklet_id: u32,
    pub min: u32,
    pub max: u32,
    pub avg: u32,
}

fn parse_output_line(line: &str) -> ExecutionTime {
    let line_split = line.split('|').collect::<Vec<&str>>();

    // Sanity check
    assert_eq!(line_split.len(), 4);
    assert!(line_split[0].starts_with("T:"));

    // 3rd char is tasklet ID
    let tasklet_id = line_split[0].chars().nth(2).unwrap().to_digit(10).unwrap();
    let min = line_split[1].parse::<u32>().unwrap();
    let max = line_split[2].parse::<u32>().unwrap();
    let avg = line_split[3].parse::<u32>().unwrap();

    ExecutionTime {
        tasklet_id,
        min,
        max,
        avg,
    }
}
