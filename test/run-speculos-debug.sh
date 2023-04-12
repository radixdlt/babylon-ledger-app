#!/bin/sh
cd ..

# Run Nano S+ version
cp ./target/nanosplus/debug/babylon-ledger-app ./apps/babylon.elf

docker run --rm -it -v "$(pwd)"/apps:/speculos/apps -p 1234:1234 -p 5000:5000 -p 40000:40000 -p 41000:41000 -p 9999:9999 --entrypoint /bin/bash speculos
