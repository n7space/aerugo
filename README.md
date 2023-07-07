# aerugo
Safety-critical applications oriented Real-Time Operating System written in Rust.

This project is developed as part of the European Space Agency activity
[Evaluation of Rust Usage in Space Applications by Developing BSP and RTOS Targeting SAMV71](https://activities.esa.int/4000140241)

## Features

`aerugo` targets ATSAMV71Q21 microcontroller based on the 32-bit ARM Cortex-M7 processor. It's design is
inspired by purely functional programming paradigm and transputers architecture.

* RTOS is implemented in a form of an executor instead of classic scheduler and doesn't support preemption.
* Executor runs tasklets, which are fine-grained units of computation, that execute a processing step in a
finite amount of time.

## Project structure

The repository structure is as follows:

- `aerugo-hal` - Traits for HAL used in the system.
- `arch` - Code specific for given architecture.
- `examples` - Examples of system usage.
- `scripts` - Handy scripts for automating some of the work.
- `src` - Code of the core system.
- `utils` - Code of additional utils.

## Cloning

## Building

aerugo requires nightly, make sure you have it installed: \
`rustup toolchain install nightly`

Tu build the system for the Cortex-M7 target run: \
`cargo build -p aerugo --features=use-aerugo-cortex-m --target=thumbv7em-none-eabihf`

x86 target is also supported for development purposes: \
`cargo build -p aerugo --features=use-aerugo-x86 --target=x86_64-unknown-linux-gnu`

## Tests

Tests can be build and run using a bash script. For all tests run: \
`./scripts/run_tests.sh`

or for specific package run for example: \
`./scripts/run_tests.sh aerugo_x86`

Tests can also be run using `cargo tests`.

## Examples

The `examples` directory contains examples of system usage. Each of the example is a separate project and can
be build and run with a simple `cargo run`.

## Name

aerugo (*noun*)

1. metallic rust, particularly of brass or copper; verdigris; patina

## License

All source code is licensed under either of
- Apache License, Version 2.0 (see ([LICENSE-APACHE](LICENSE-APACHE)) or
[https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0)).
- MIT License (see ([LICENSE-MIT](LICENSE-MIT)) or
[https://creativecommons.org/licenses/by-sa/4.0/legalcode](https://creativecommons.org/licenses/by-sa/4.0/legalcode)).
