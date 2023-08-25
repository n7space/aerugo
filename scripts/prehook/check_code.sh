#!/bin/bash

set -euo pipefail

cargo clippy --workspace --tests -F use-aerugo-cortex-m -- -D warnings

# Check examples if changed
if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached)" =~ "examples/" ]]; then
    for d in examples/*/; do
        pushd $d > /dev/null
        if [[ -f "Cargo.toml" ]]; then
            cargo clippy -- -D warnings
        fi
        popd > /dev/null
    done
fi

# Check testbins if changed
if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached)" =~ "testbins/" ]]; then
    for d in testbins/*/; do
        pushd $d > /dev/null
        if [[ -f "Cargo.toml" ]]; then
            cargo clippy -- -D warnings
        fi
        popd > /dev/null
    done
fi
