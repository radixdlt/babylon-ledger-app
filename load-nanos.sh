#!/bin/sh
set -e
export LEDGER_TARGETS=./target-config

echo "Building And Loading RELEASE to NANO S"
cargo build --release -Z build-std=core --target=./target-config/nanos.json
cargo ledger nanos -l
