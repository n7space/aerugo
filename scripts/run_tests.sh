#!/bin/bash

set -euo pipefail

aerugo_x86() {
    export AERUGO_TASKLET_COUNT=3
    export AERUGO_EVENT_COUNT=3

    cargo test --features=use-aerugo-x86 --target=x86_64-unknown-linux-gnu --package aerugo

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
