#!/bin/bash

set -euo pipefail

cargo fmt

for d in examples/*/; do
    pushd $d > /dev/null
    cargo fmt
    popd > /dev/null
done

for d in testbins/*/; do
    pushd $d > /dev/null
    cargo fmt
    popd > /dev/null
done
