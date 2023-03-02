#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

echo "Building RELEASE for NANO S"
cargo build --release -Z sparse-registry -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=./target-config/nanos.json
cargo ledger nanos

echo "Building RELEASE for NANO S Plus"
cargo build --release -Z sparse-registry -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=./target-config/nanosplus.json
cargo ledger nanosplus

echo "Building RELEASE for NANO X"
cargo build --release -Z sparse-registry -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=./target-config/nanox.json
cargo ledger nanox
