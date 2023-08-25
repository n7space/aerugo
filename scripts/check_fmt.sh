#!/bin/bash

set -euo pipefail

cargo fmt -- --check --color always

for d in examples/*/; do
    pushd $d >/dev/null
    if [[ -f "Cargo.toml" ]]; then
        cargo fmt -- --check --color always
    fi
    popd >/dev/null
done

for d in testbins/*/; do
    pushd $d >/dev/null
    if [[ -f "Cargo.toml" ]]; then
        cargo fmt -- --check --color always
    fi
    popd >/dev/null
done
