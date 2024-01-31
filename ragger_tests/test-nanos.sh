cp ../target/nanos/debug/babylon-ledger-app ../build/nanos/bin/app.elf
pytest -v --tb=short --device nanos
