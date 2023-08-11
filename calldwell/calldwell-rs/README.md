# Calldwell - Rust back-end

This crate contains Rust backend for Calldwell testing framework.
It provides facilities required to communicate with Calldwell's front-end.

## Communication protocol

Calldwell uses RTT as communication interface between the MCU and host. RTT was chosen due to the fact that it's simple to use, and very fast, compared to alternative methods (like semihosting).

Calldwell uses data streams to avoid issues with packet fragmentation - data (of arbitrary length) going through Calldwell must be prefixed and suffixed with specific byte sequence.
Calldwell currently supports only ASCII text transmission, and this method of stream state management is safe as long, as only the standard ASCII characters (in 0x00 - 0x7F range) are transmitted.
Due to the scope and time constraints of this project, current solution is qualified as "good enough".
However, in the future, to support binary data transmission, a more robust stream state management method should be implemented.
