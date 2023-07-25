#!/bin/bash

aerugo_x86_debug() {
    cargo build --features=use-aerugo-x86 --target=x86_64-unknown-linux-gnu --package aerugo
}

aerugo_x86_release() {
    cargo build --release --features=use-aerugo-x86 --target=x86_64-unknown-linux-gnu --package aerugo
}

aerugo_cm7_debug() {
    cargo build --features=use-aerugo-cortex-m --target=thumbv7em-none-eabihf --package aerugo
}

aerugo_cm7_release() {
    cargo build --release --features=use-aerugo-cortex-m --target=thumbv7em-none-eabihf --package aerugo
}

aerugo_cm7_docs() {
    cargo doc --workspace --no-deps --document-private-items -F use-aerugo-cortex-m
}

$@
