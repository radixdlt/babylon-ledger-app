#!/bin/sh
cd ..

# Run Nano S+ version
cp ./target/nanosplus/debug/babylon-ledger-app ./apps/babylon.elf

docker run --rm -it -v "$(pwd)"/apps:/speculos/apps -p 1234:1234 -p 5000:5000 -p 9999:9999 speculos --model nanosp ./apps/babylon.elf --seed "equip will roof matter pink blind book anxiety banner elbow sun young" --display headless --apdu-port 9999