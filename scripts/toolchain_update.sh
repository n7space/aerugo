#!/bin/bash

# Run this from main project's directory.
# This script can be used to update nightly toolchain and perform full rebuild + check for all platforms.
# It's useful for checking if there aren't any breaking nightly changes, as updating toolchain automatically
# invalidates all already built binaries and requires a full rebuild anyway.

# This process can take significant amount of time, so it's recommended to do at most daily, as there's
# no point in updating the toolchain mutliple times a day.

# Dependencies:
# * `fd` (faster alternative to `find`)
# * `rustup`
# Aerugo dependencies, listed in README.md

set -euo pipefail

echo "Updating Rust toolchain"
rustup update nightly

echo "Clearing all build directories"
fd --no-ignore-vcs --case-sensitive --type directory target --exec bash -c "echo {} && rm -rf {}"

./scripts/full_build.sh
