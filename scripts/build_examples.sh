#!/bin/bash

set -euo pipefail

if [ $# -eq 0 ]
then
    for d in examples/*/; do
        pushd $d > /dev/null
        cargo build
        popd > /dev/null
    done
else
    pushd examples/$1/ > /dev/null
    cargo build
    popd > /dev/null
fi
