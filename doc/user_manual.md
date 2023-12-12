  
`aerugo` is implemented in a form of an executor instead of classic scheduler and doesn't support preemption. Lack of
preemption simplifies the system design and analysis and can significantly reduce memory footprint as there is no need
of separate stack for each tasklet. Executor runs tasklets, which are fine-grained units of computation, that execute
a processing step in a finite amount of time. They are meant to be designed as small, indivisible and non-blocking units
of operations which communicate between each other via messages, events and conditions.

## Getting started

### Building

aerugo requires nightly Rust toolchain, to install it run:

```sh
rustup toolchain install nightly
```

For the Cortex-M7 platform install that target via `rustup`:

```sh
rustup target add thumbv7em-none-eabihf
```

Then to build the system run:

```sh
cargo build -p aerugo --features=use-aerugo-cortex-m --target=thumbv7em-none-eabihf
```

### Tests

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

Tests for SAMV71 are ignored by default, to prevent running them with CI, as they require additional environment setup
and access to a development board. All tests are configured to be ran on a remote machine that has the development board
connected to it via OpenOCD-supported debugger probe.

#### Preparing the environment

To run SAMV71 tests, these environmental variables have to be set:

* `AERUGO_BOARD_LOGIN` - SSH login of remote debugging setup
* `AERUGO_BOARD_PASSWORD` - SSH password of remote debugging setup
* `AERUGO_BOARD_HOSTNAME` - Hostname or IP address of remote debugging setup
* `AERUGO_BOARD_GDB_PORT` - TCP port of the GDB server that will be started on remote debugging setup
* `AERUGO_BOARD_RTT_PORT` - TCP port for RTT server

Both Calldwell, and all helper scripts, are guaranteed to be backwards-compatible with Python 3.9, however, they should
also work fine with Python 3.10 and 3.11.

Use [`pyproject.toml`](./pyproject.toml) to create virtual environment (Poetry is recommended, but any virtualenv
manager that can understand `pyproject.toml` will do fine).

When using Poetry, run `poetry shell` from this directory. **When running this command for the first time, you should
run `poetry install` first to install required dependencies in virtual environment!**

#### Running the tests

**All commands from this section should be run from project's root directory!**
To invoke each test manually run `./tests/requirements/test/test_*.py` script via virtualenv shell. Test script will
automatically build the test, if needed. For example:

```sh
python ./tests/requirements/test/test_hal_watchdog.py
```

Or you run them all at once, with

```sh
./scripts/run_tests.sh aerugo_v71
```

**Warning: If running test via `cargo` for the first time, be aware that their build time will be contained in test
execution time, which can potentially cause a timeout. It's recommended to build (or run) all test binaries manually
for the first time.** \
Test binaries can be built either by running [`./scripts/build_hal_tests.sh`](./scripts/build_hal_tests.sh), or by
invoking `cargo build` in test's project directory (in [`testbins`](./testbins/))!

## Usage

### RTOS

Basic building blocks of `aerugo` are tasklets. Each tasklet can be thought of as an one operation that is performed over some data. In `aerugo` tasklet is defined whith a free function and a struct for its context.

```rust,ignore
struct TaskletContext {
    counter: u8,
}

fn tasklet_function(data: u8, context: &mut TaskletContext, api: &'static dyn RuntimeApi) {
    context.counter = context.counter.wrapping_add(data);
}
```

\
Tasklet has to be statically allocated by in [TaskletStorage](crate::tasklet::TaskletStorage) structure. This can be
done using [InitApi](crate::api::InitApi) which can be obtained via
[system initialization](crate::aerugo::Aerugo::initialize). After that user can reference tasklet using a handle
that can be created using the initialized storage.

```rust,ignore
static TASKLET_STORAGE: TaskletStorage<u8, TaskletContext, 0> = TaskletStorage::new();

fn main() -> ! {
    let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());

    let tasklet_config = TaskletConfig {
        name: "MyTasklet",
        ..Default::default()
    };
    let tasklet_context = TaskletContext { counter: 0 };

    aerugo.create_tasklet_with_context(
        tasklet_config,
        tasklet_function,
        tasklet_context,
        &TASKLET_STORAGE
    );

    let tasklet_handle = TASKLET_STORAGE.create_handle().unwrap();
}
```

\
Each tasklet is required to be subscribed to a data provider. This can be:
* [message queue](crate::aerugo::Aerugo::subscribe_tasklet_to_queue)
* [events](crate::aerugo::Aerugo::subscribe_tasklet_to_events)
* [condition](crate::aerugo::Aerugo::subscribe_tasklet_to_condition)
* [cyclic execution](crate::aerugo::Aerugo::subscribe_tasklet_to_cyclic)

\
Additionally tasklet can have a [set of conditions assigned](crate::aerugo::Aerugo::set_tasklet_conditions)
to it. They are controlling whether tasklet shall be executed.

\
After all initialization system scheduler can be started with [start](crate::aerugo::Aerugo::start) function.

### Hardware

Currently `aerugo` targets SAMV71Q21 board. To access peripherals use the second value returned from the
[initialize](crate::aerugo::Aerugo::initialize) function.

```rust,ignore
fn main() -> ! {
    let (_, mut peripherals) = Aerugo::initialize(SystemHardwareConfig::default());
}
```

\
Using user peripherals obtained this way it is possible to create instances of the concrete hardware peripherals.
The following peripherals are implemented:
* [FPU](../samv71_hal/fpu/struct.Fpu.html)
* [NVIC](../samv71_hal/nvic/struct.NVIC.html)
* [PIO](../samv71_hal/pio/index.html)
* [PMC](../samv71_hal/pmc/struct.PMC.html)
* [SCB](../samv71q21_pac/struct.SCB.html)
* [SPI](../samv71_hal/spi/struct.Spi.html)
* [SYST](../samv71q21_pac/struct.SYST.html)
* [TC](../samv71_hal/timer/struct.Timer.html)
* [UART](../samv71_hal/uart/struct.Uart.html)
* [XDMAC](../samv71_hal/xdmac/struct.Xdmac.html)

### Implementing new target

`aerugo` itself is platform agnostic, that means that it doesn't explicitly depends on any specific hardware target.
To support a new platform the HAL for that target has to implement the provided trait
[AerugoHal](../aerugo_hal/trait.AerugoHal.html).\
For that implementation only two basic elements are required: timer (system time is required by the OS for scheduling)
and executing code in critical section (which for the single-core purposes can be just disabling/enabling interrupts).

## Examples

In the root of the project there is a directory `examples` containing showcases how to use each part of the system
and also some of the SAMV71 drivers.

## Demo

In the root of the project there is a directory `demos` contatining demonstration applications showcasing usage of
the system in a bigger scope. Currently there is a one demo which uses LSM6DSO accelerometer/gyroscope over SPI
and UART communication for SAMV71Q21 target.
