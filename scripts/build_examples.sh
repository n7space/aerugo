#!/bin/bash

set -euo pipefail

if [ $# -eq 0 ]; then
    for d in examples/*/; do
        pushd $d > /dev/null
        echo "Building $d"
        cargo build
        popd > /dev/null
    done
else
    pushd examples/$1/ > /dev/null
    echo "Building examples/$1/"
    cargo build
    popd > /dev/null
fi
