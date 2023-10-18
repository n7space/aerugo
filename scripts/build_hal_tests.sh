#!/bin/bash

set -euo pipefail

for d in testbins/test-hal-*/; do
  pushd $d > /dev/null
  echo "Building $d"
  cargo build --release
  popd > /dev/null
done
