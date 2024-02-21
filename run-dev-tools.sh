#!/bin/sh
docker pull ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest
docker run --rm -it --publish 5010:5000 -v "$(pwd -P):/app" ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest
