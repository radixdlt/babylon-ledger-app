mkdir -p ../build/nanos2/bin/
cp ../target/nanosplus/release/babylon-ledger-app ../build/nanos2/bin/app.elf
pytest -v --backend speculos --tb=short --device nanosp $@
