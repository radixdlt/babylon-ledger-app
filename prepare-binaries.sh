#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config

echo "Building RELEASE for NANO S"
cargo build --release -Z build-std=core --target=./target-config/nanos.json
echo "Building RELEASE for NANO S Plus"
cargo build --release -Z build-std=core --target=./target-config/nanosplus.json
echo "Building RELEASE for NANO X"
cargo build --release -Z build-std=core --target=./target-config/nanox.json
