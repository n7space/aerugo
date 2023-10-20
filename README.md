# aerugo

Safety-critical applications oriented Real-Time Operating System written in Rust.

This project is developed as part of the European Space Agency activity
[Evaluation of Rust Usage in Space Applications by Developing BSP and RTOS Targeting SAMV71](https://activities.esa.int/4000140241)

## Features

`aerugo` targets ATSAMV71Q21 microcontroller based on the 32-bit ARM Cortex-M7 processor. Its design is
inspired by purely functional programming paradigm and transputers architecture.

* RTOS is implemented in a form of an executor instead of classic scheduler and doesn't support preemption.
* Executor runs tasklets, which are fine-grained units of computation, that execute a processing step in a
finite amount of time.

## Project structure

The repository structure is as follows:

* `aerugo-hal` - Traits for HAL used in the system.
* `arch` - Code specific for given architecture.
* `calldwell` - Our embedded testing framework.
* `examples` - Examples of system usage.
* `scripts` - Handy scripts for automating some of the work.
* `src` - Code of the core system.
* `testbins` - Test binaries
* `tests` - Test scripts
* `utils` - Code of additional utils.

## Building

aerugo requires nightly, make sure you have it installed:

```sh
rustup toolchain install nightly
```

For the Cortex-M7 platform you first need to install that target via `rustup`:

```sh
rustup target add thumbv7em-none-eabihf
```

Then to build the system run:

```sh
cargo build -p aerugo --features=use-aerugo-cortex-m --target=thumbv7em-none-eabihf
```

x86 target is also supported for development purposes:

```sh
cargo build -p aerugo --features=use-aerugo-x86 --target=x86_64-unknown-linux-gnu
```

## Tests

Tests can be built and run using a bash script. For all tests run:

```sh
./scripts/run_tests.sh
```

or for specific package run for example:

```sh
./scripts/run_tests.sh aerugo_x86
```

Tests can also be run using `cargo test` with `--features` and `--target` flags.

### Running SAMV71 tests

Tests for SAMV71 are ignored by default, to prevent running them with CI, as they require additional environment setup and access to a development board.
All tests are configured to be ran on a remote machine that has the development board connected to it via OpenOCD-supported debugger probe.

#### Preparing the environment

To run SAMV71 tests, you have to set these environmental variables first:

* `AERUGO_BOARD_LOGIN` - SSH login of remote debugging setup
* `AERUGO_BOARD_PASSWORD` - SSH password of remote debugging setup
* `AERUGO_BOARD_HOSTNAME` - Hostname or IP address of remote debugging setup
* `AERUGO_BOARD_GDB_PORT` - TCP port of the GDB server that will be started on remote debugging setup
* `AERUGO_BOARD_RTT_PORT` - TCP port for RTT server

Both Calldwell, and all helper scripts, are guaranteed to be backwards-compatible with Python 3.9, however, they should also work fine with Python 3.10 and 3.11.

Use [`pyproject.toml`](./pyproject.toml) to create virtual environment (Poetry is recommended, but any virtualenv manager that can understand `pyproject.toml` will do fine).

When using Poetry, run `poetry shell` from this directory. **When running this command for the first time, you should also run `poetry install` to install required dependencies in virtual environment!**

#### Running the tests

**All commands from this section should be run from project's root directory (the one with this README)!**
You can either run each test manually, by invoking `./tests/requirements/test/test_*.py` script via virtualenv shell. Test script will automatically build the test, if needed.

```sh
python ./tests/requirements/test/test_hal_watchdog.py
```

Or you can run them all at once, with

```sh
./scripts/run_tests.sh aerugo_v71
```

**Warning: If running test via `cargo` for the first time, be aware that their build time will be contained in test execution time, which can potentially cause a timeout. It's recommended to build (or run) all test binaries manually for the first time.**
Test binaries can be built either by running [`./scripts/build_hal_tests.sh`](./scripts/build_hal_tests.sh), or by invoking `cargo build` in test's project directory (in [`testbins`](./testbins/))!

## Examples

The `examples` directory contains examples of system usage. Each of the examples is a separate project and can
be built and run with a simple `cargo run`.

### Running SAMV71 examples

SAMV71 examples are using RTT to communicate - hence, you won't see any output if you'll run them with GDB manually, without setting up RTT server and client correctly.
However, a helper script is provided that makes running the examples very quick and easy.

**Follow the steps from [Preparing the environment](#preparing-the-environment) section, if you haven't done that already, before continuing.**

**All commands from this section should be ran from project's root directory (the one with this README)!**

To run an example, simply run [`./scripts/run_v71_example.py`](./scripts/run_v71_example.py) script via virtualenv shell, providing the name of the example as an argument.
For example:

```sh
python ./scripts/run_v71_example.py samv71-fizz-buzz
```

This script will automatically build and run the example, provided that it's name is valid and it can be successfully built. By default, the example is built in release mode with debugging symbols.

## Name

aerugo (*noun*)

1. metallic rust, particularly of brass or copper; verdigris; patina

## License

All source code is licensed under either of

* Apache License, Version 2.0 (see ([LICENSE-APACHE](LICENSE-APACHE)) or
[https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0)).
* MIT License (see ([LICENSE-MIT](LICENSE-MIT)) or
[https://creativecommons.org/licenses/by-sa/4.0/legalcode](https://creativecommons.org/licenses/by-sa/4.0/legalcode)).
