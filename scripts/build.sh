#!/bin/bash

set -euo pipefail

aerugo_x86_debug() {
    echo "Building Aerugo (x86, debug)"
    cargo build --features=use-aerugo-x86 --target=x86_64-unknown-linux-gnu --package aerugo
}

aerugo_x86_release() {
    echo "Building Aerugo (x86, release)"
    cargo build --release --features=use-aerugo-x86 --target=x86_64-unknown-linux-gnu --package aerugo
}

aerugo_cm7_debug() {
    echo "Building Aerugo (Cortex-M, debug)"
    cargo build --features=use-aerugo-cortex-m --target=thumbv7em-none-eabihf --package aerugo
}

aerugo_cm7_release() {
    echo "Building Aerugo (Cortex-M, release)"
    cargo build --release --features=use-aerugo-cortex-m --target=thumbv7em-none-eabihf --package aerugo
}

$@
