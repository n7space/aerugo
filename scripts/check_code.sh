#!/bin/bash

aerugo_x86() {
    cargo clippy -p aerugo -F use-aerugo-x86 -- -D warnings
}

aerugo_cm7() {
    cargo clippy -p aerugo -F use-aerugo-cortex-m -- -D warnings
}

$@
