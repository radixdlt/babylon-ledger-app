#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config

echo "Building DEBUG for NANO S"
cargo build --target=./target-config/nanos.json
arm-none-eabi-size ./target/nanos/debug/babylon-ledger-app
