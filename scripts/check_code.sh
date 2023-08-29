#!/bin/bash

set -euo pipefail

cargo clippy --workspace --tests -F use-aerugo-cortex-m -- -D warnings

for d in examples/*/; do
    pushd $d >/dev/null
    if [[ -f "Cargo.toml" ]]; then
        echo "Checking code: $d"
        cargo clippy -- -D warnings
    fi
    popd >/dev/null
done

for d in testbins/*/; do
    pushd $d >/dev/null
    if [[ -f "Cargo.toml" ]]; then
        echo "Checking code: $d"
        cargo clippy -- -D warnings
    fi
    popd >/dev/null
done
