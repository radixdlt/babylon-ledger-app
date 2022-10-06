#!/bin/sh
docker run --rm -ti -v "$(realpath .):/app" babylon-ledger-app-builder:latest
