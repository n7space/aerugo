#!/bin/bash

set -euo pipefail

echo "Generating requirements documentation"
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items -F use-aerugo-cortex-m --target-dir docs
RUSTDOCFLAGS="-D warnings" cargo rustdoc --test requirement_tests -F use-aerugo-cortex-m --target-dir docs-test -- --document-private-items
