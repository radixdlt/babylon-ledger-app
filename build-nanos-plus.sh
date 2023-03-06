#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

echo "Building DEBUG for NANO S Plus"
cargo build -Z sparse-registry -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=./target-config/nanosplus.json
