# Radix Babylon Ledger App (WIP)
Ledger Nano S/S Plus/X app for Babylon

## WARNING!
Do not touch `Cargo.lock` for now. Some dependencies must be of specified versions.

## Build
Simplest way to build app is to use application builder container. To build container, use script provided 
in the `app-builder` directory. Once container is built, run it using `run-app-builder.sh` and then build 
binaries using `prepare-binaries.sh` script. 

## Local Build Environment Setup
Instructions are provided for Ubuntu 22.04

### Prerequisites
#### Install ARM GCC, Binutils, cross-compilation headers:
```
sudo apt-get install gcc-arm-none-eabi binutils-arm-none-eabi gcc-multilib
```

#### Install Python 3 and Pip3
```
sudo apt-get install python3 python3-pip
```

#### Install Clang
```
sudo apt-get install clang
```

#### Install Rust
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
rustup component add rust-src
rustup target add thumbv6m-none-eabi
```

#### Install Cargo Ledger
`cargo install --git https://github.com/LedgerHQ/cargo-ledger`

## Build Commands
### Production
For building binaries for all supported targets, use following script:
```
prepare-binaries.sh
```
### Loading binaries for testing purposes
For testing purposes also can be used following script:
```
load-nanos.sh
```
It builds binaries for Ledger Nano and makes attempt to load it into attached device.
### Individual Targets
Build commands for individual targets:
```
cargo build --release -Z build-std=core --target=./target-config/nanos.json
cargo build --release -Z build-std=core --target=./target-config/nanosplus.json
cargo build --release -Z build-std=core --target=./target-config/nanox.json
```
Note that these commands build application but do not prepare data necessary for uploading app to hardware wallet.
To prepare necessary data, use one of the following commands:
```
cargo ledger nanos
cargo ledger nanospus
cargo ledger nanox
```
If necessary, binaries also could be uploaded to target device (must be attached and unlocked by the moment, when command is executed).
To do this, add `-l` flag to the `cargo ledger` command, as follows:
```
cargo ledger nanos -l
```

### Testing
#### Testing of "Get API Version" call
- Load binaries into attached device (device may ask several questions)
- Select "Radix Babylon" application on device (device may ask several questions)
- Run `test/get-api-version/show-api-version.sh` script. It should output human-readable version of the installed binaries of the application to the console.
