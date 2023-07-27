#!/bin/bash

set -euo pipefail

cargo fmt -- --check --color always

# Check examples if changed
if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached)" =~ "examples/" ]]; then
    for d in examples/*/; do
        pushd $d > /dev/null
        cargo fmt -- --check --color always
        popd > /dev/null
    done
fi

# Check testbins if changed
if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached)" =~ "testbins/" ]]; then
    for d in testbins/*/; do
        pushd $d > /dev/null
        cargo fmt -- --check --color always
        popd > /dev/null
    done
fi
