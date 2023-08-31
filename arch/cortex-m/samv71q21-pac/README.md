# SAMV71Q21 Peripheral Access Crate

This crate contains code generated with [`svd2rust`](https://github.com/rust-embedded/svd2rust) using SVD file downloaded from [Microchip repository](https://packs.download.microchip.com/).

Peripheral Access Crate (PAC) contains definitions, structures and traits for MCU peripherals registers, and provides a relatively safe abstraction over typical register operations. Content of this library is supposed to be used as a building block of HAL, using it directly is not recommended (with rare exceptions, where PAC driver has safe to use and readable interface for some peripherals that does not require further abstraction).

For more information about hardware abstraction architecture used in Rust, [consult this page](https://docs.rust-embedded.org/book/portability/index.html)
