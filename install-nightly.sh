#!/bin/sh

if [ $# -eq 0 ]; then
    echo "Usage: $0 <target>"
    exit 1
fi

target=nightly-$1

rustup install $target
rustup target add thumbv6m-none-eabi --toolchain $target
rustup component add rust-src --toolchain $target
cargo +$target ledger setup
export RUST_NIGHTLY=$target
