#!/bin/bash

set -euo pipefail

echo "Generating documentation"
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items -F use-aerugo-cortex-m $@
