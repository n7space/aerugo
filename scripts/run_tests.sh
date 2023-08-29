#!/bin/bash

set -euo pipefail

aerugo_x86() {
    export AERUGO_TASKLET_COUNT=5
    export AERUGO_EVENT_COUNT=3

    echo "Running x86 tests"
    cargo test --features=use-aerugo-x86 --target=x86_64-unknown-linux-gnu --package aerugo

    export -n AERUGO_EVENT_COUNT
    export -n AERUGO_TASKLET_COUNT
}

aerugo_v71() {
    export AERUGO_TASKLET_COUNT=5
    export AERUGO_EVENT_COUNT=3

    echo "Running HAL tests"
    cargo test --features=test-aerugo-cortex-m --target=x86_64-unknown-linux-gnu --package aerugo test_hal -- --test-threads=1

    export -n AERUGO_EVENT_COUNT
    export -n AERUGO_TASKLET_COUNT
}

env_parser() {
    export ENV_PARSER_TEST_INTEGER_OVERRIDE=42
    export ENV_PARSER_TEST_INTEGER_DIFFERENT_NAME=314

    echo "Running env_parser tests"
    cargo test --package env-parser-tests

    export -n ENV_PARSER_TEST_INTEGER_OVERRIDE
    export -n ENV_PARSER_TEST_INTEGER_DIFFERENT_NAME
}

all() {
    export ENV_PARSER_TEST_INTEGER_OVERRIDE=42
    export ENV_PARSER_TEST_INTEGER_DIFFERENT_NAME=314
    export AERUGO_TASKLET_COUNT=5
    export AERUGO_EVENT_COUNT=3

    # This is not simply a call to functions above, as it would produce two test outputs (with some tests ignored in one).
    # This will run all the tests using single runner, and generate a single test report.
    echo "Running all tests"
    cargo test --features=test-aerugo-cortex-m --target=x86_64-unknown-linux-gnu --workspace -- --test-threads=1

    export -n AERUGO_EVENT_COUNT
    export -n AERUGO_TASKLET_COUNT
    export -n ENV_PARSER_TEST_INTEGER_OVERRIDE
    export -n ENV_PARSER_TEST_INTEGER_DIFFERENT_NAME
}

run_ci_tests=$#

if [ $run_ci_tests == 0 ]; then
    env_parser
    aerugo_x86
else
    "$@"
fi
