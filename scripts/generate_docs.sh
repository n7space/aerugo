#!/bin/bash

set -euo pipefail

RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items -F use-aerugo-cortex-m $@
