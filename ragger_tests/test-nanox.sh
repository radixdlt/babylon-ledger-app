mkdir -p ../build/nanox/bin/
cp ../target/nanox/release/babylon-ledger-app ../build/nanox/bin/app.elf
pytest -v --tb=short --device nanox $@
