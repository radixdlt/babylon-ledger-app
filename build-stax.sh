#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config

echo "Building DEBUG for Stax"
cargo build --target=./target-config/stax.json
arm-none-eabi-size ./target/nanos/debug/babylon-ledger-app
