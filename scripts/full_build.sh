#!/bin/bash

# This scripts performs full format-check-build-test process of Aerugo.
# It will format whole project, check the code, build Aerugo, tests and examples, run all unit tests (except those requiring hardware), and generate documentation
# Useful for first-time runs (for example, after toolchain update) or major sanity checks.

set -euo pipefail

echo "Formatting the code"
./scripts/format_code.sh

echo "Checking the code"
./scripts/check_code.sh

echo "Building Aerugo"
./scripts/build.sh aerugo_x86_debug
./scripts/build.sh aerugo_x86_release
./scripts/build.sh aerugo_cm7_debug
./scripts/build.sh aerugo_cm7_release

echo "Building and running all tests (except HAL tests)"
./scripts/run_tests.sh

echo "Building HAL tests"
./scripts/build_hal_tests.sh

echo "Building examples"
./scripts/build_examples.sh

echo "Generating documentation"
./scripts/generate_docs.sh
