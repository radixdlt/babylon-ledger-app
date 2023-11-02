#!/bin/sh
sudo docker run --rm -ti -v "$(realpath .):/app" --privileged -v "/dev/bus/usb:/dev/bus/usb" babylon-ledger-app-builder:latest
