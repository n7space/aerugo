# Calldwell testing framework

This directory contains Calldwell test framework, for running and testing embedded software.

## Features

* Control over GDB (via `pygdbmi`), with functional interface for commonly used operations.
* SSH client (via `paramiko`), allowing remote program execution and file transfer via SFTP.
* RTT client, providing real-time, bidirectional communication with target board.

### GDB-related features

* Monitoring program state via GDB notifications
* Connecting to remote GDB target
* Selecting and loading a program into target memory
* Restarting target platform
* Controlling program execution
* Performing full program startup
* Placing breakpoints
* Setting up RTT
* Fetching symbols from debugged binary

### SSH-related features

* Running commands on remote machine
* Transferring files between local and remote machine
  
### RTT-related features

* Connecting to RTT server via TCP socket
* Bidirectional communication via RTT port
