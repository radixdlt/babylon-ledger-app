#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

echo "Building DEBUG & RELEASE for NANO S"
cargo build -Z sparse-registry -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=./target-config/nanos.json
cargo ledger nanos -l
