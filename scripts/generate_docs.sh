#!/bin/bash

set -euo pipefail

cargo doc --workspace --no-deps --document-private-items -F use-aerugo-cortex-m $@
