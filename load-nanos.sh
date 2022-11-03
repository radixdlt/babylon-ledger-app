#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config

echo "Building DEBUG & RELEASE for NANO S"
cargo build -Z build-std=core --target=./target-config/nanos.json
cargo ledger nanos -l
