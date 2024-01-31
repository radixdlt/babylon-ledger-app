cp ../target/nanox/debug/babylon-ledger-app ../build/nanox/bin/app.elf
pytest -v --tb=short --device nanox
