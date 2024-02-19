#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config

echo "Building DEBUG for NANO S Plus"
cargo clippy -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=./target-config/nanosplus.json
