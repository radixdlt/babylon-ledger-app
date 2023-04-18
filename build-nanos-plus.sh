#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config

echo "Building DEBUG for NANO S Plus"
cargo build -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=./target-config/nanosplus.json
arm-none-eabi-size ./target/nanosplus/debug/babylon-ledger-app