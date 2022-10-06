# babylon-ledger-app
(Work In Progress)
Ledger Nano S/S Plus/X app for Babylon

## WARNING!
Do not touch Cargo.lock for now. Some dependencies must be of specified versions.

## Build
Simplest way to build app is to use application builder container. To build container, use script provided 
in the `app-builder` directory. Once container is built, run it using `run-app-builder.sh` and then build 
binaries using `prepare-binaries.sh` script.

