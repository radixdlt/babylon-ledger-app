mkdir -p ../build/nanos2/bin/
cp ../target/nanosplus/release/babylon-ledger-app ../build/nanos2/bin/app.elf
pytest -v --tb=short --device nanosp $@
