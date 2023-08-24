#!/bin/bash

set -euo pipefail

aerugo_x86() {
    export AERUGO_TASKLET_COUNT=5
    export AERUGO_EVENT_COUNT=3

    cargo test --features=use-aerugo-x86 --target=x86_64-unknown-linux-gnu --package aerugo

    export -n AERUGO_EVENT_COUNT
    export -n AERUGO_TASKLET_COUNT
}

aerugo_v71() {
    export AERUGO_TASKLET_COUNT=5
    export AERUGO_EVENT_COUNT=3

    cargo test --features=test-aerugo-cortex-m --target=x86_64-unknown-linux-gnu --package aerugo test_hal -- --test-threads=1

    export -n AERUGO_EVENT_COUNT
    export -n AERUGO_TASKLET_COUNT
}

env_parser() {
    export ENV_PARSER_TEST_INTEGER_OVERRIDE=42
    export ENV_PARSER_TEST_INTEGER_DIFFERENT_NAME=314

    cargo test --package env-parser-tests

    export -n ENV_PARSER_TEST_INTEGER_OVERRIDE
    export -n ENV_PARSER_TEST_INTEGER_DIFFERENT_NAME
}

run_all_tests=$#

if [ $run_all_tests == 0 ]; then
    env_parser
    aerugo_x86
else
    "$@"
fi
