#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config

echo "Building DEBUG for NANO X"
cargo build --target=./target-config/nanox.json
arm-none-eabi-size ./target/nanox/debug/babylon-ledger-app
