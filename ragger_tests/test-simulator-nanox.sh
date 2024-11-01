mkdir -p ../build/nanox/bin/
cp ../target/nanox/release/babylon-ledger-app ../build/nanox/bin/app.elf
pytest -v --backend speculos --tb=short --device nanox $@
