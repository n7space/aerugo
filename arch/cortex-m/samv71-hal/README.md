# SAMV71 HAL

This crate contains HAL modules for SAMV71 MCU peripherals.

Currently, it supports only SAMV71Q21 MCU, but it's possible to add support for other MCUs from SAMV71 series, if needed.

HAL modules provide relatively safe and easy to use abstraction over microcontroller's peripherals. This should be preferred method of interfacing with MCU hardware.

For time types, `fugit` crate is used. You can find aliases for time types in `lib.rs`.

For more information about hardware abstraction architecture used in Rust, [consult this page](https://docs.rust-embedded.org/book/portability/index.html)

## Philosophy

This crate uses SAMV71Q21 PAC as register interface. It makes the development much quicker and provides abstraction over registers, which makes adding support for similar part numbers easy.

In typical scenario, the user should use HAL to fetch a structure containing all the available (and supported) peripherals of the MCU. This structure should be fetched only once during program's lifetime, as having multiple instances of them would break safety rules.

These instances are marked as `Send` automatically, due to the fact that PAC peripherals are also marked as `Send` explicitly. `Send` is enforced correctly as long as a type is safe to send to another thread - or, in context of microcontrollers, [an interrupt](https://docs.rust-embedded.org/book/concurrency/#concurrency). Since there can be only one instance of each peripheral, as long as we're using safe method of getting them, `Send` is enforced because there's at most one pointer to each peripheral's registers at once, therefore no memory is ever shared.

However, HAL peripherals do not implement `Sync`, as sharing references between threads/interrupts is not safe for the same reason `Send` stops being enforced when more than one instance of a peripheral exists. User of this crate must enforce the interrupt safety manually, either by design, or by wrapping the peripherals in safe `Sync` wrapper. [Consult Rust Embedded book for an example](https://docs.rust-embedded.org/book/concurrency/#sharing-peripherals)
