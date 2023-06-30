#!/bin/bash

aerugo_x86_tests() {
    cargo test --features=use-aerugo-x86 --target=x86_64-unknown-linux-gnu --package aerugo
}

aerugo_x86_tests
