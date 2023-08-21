#!/bin/bash

set -euo pipefail

for d in testbins/test-hal-*/; do
  pushd $d > /dev/null
  cargo build
  popd > /dev/null
done
