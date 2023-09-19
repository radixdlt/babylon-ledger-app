#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config

echo "Building DEBUG for NANO S Plus"
cargo build --target=./target-config/nanosplus.json
arm-none-eabi-size ./target/nanosplus/debug/babylon-ledger-app