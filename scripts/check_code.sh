#!/bin/bash

aerugo_x86() {
    cargo clippy -p aerugo -F use-aerugo-x86 --tests -- -D warnings
}

aerugo_cm7() {
    cargo clippy -p aerugo -F use-aerugo-cortex-m --tests -- -D warnings
}

samv71q21_pac() {
    cargo clippy -p samv71q21-pac -- -D warnings
}

$@
