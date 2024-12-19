mkdir -p ../build/nanos/bin/
cp ../target/nanos/debug/babylon-ledger-app ../build/nanos/bin/app.elf
pytest -v --backend speculos --tb=short --device nanos $@
