mkdir -p ../build/nanos/bin/
cp ../target/nanos/release/babylon-ledger-app ../build/nanos/bin/app.elf
pytest -v --tb=short --device nanos $@
