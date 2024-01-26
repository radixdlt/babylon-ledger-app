#!/bin/sh
sudo docker run --rm -ti -v "$(realpath .):/app" --user $(id -u):$(id -g) -v "/tmp/.X11-unix:/tmp/.X11-unix" -e DISPLAY=$DISPLAY ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools:latest
