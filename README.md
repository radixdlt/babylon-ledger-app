# Radix Babylon Ledger App (WIP)

Ledger Nano S/S Plus/X app for Babylon

## WARNING!

Do not touch `Cargo.lock` for now. Some dependencies must be of specified versions.

## Build

Simplest way to build app is to use application builder container. To build container, use script provided
in the `app-builder` directory. Once container is built, run it using `run-app-builder.sh` and then build
binaries using `prepare-binaries.sh` script.

## Local Build Environment Setup

Instructions are provided for Ubuntu 22.04. For other operating systems it is suggested to use 
Docker builder image. Refer to builder image [documentation](./app-builder/README.md) for details.

### Prerequisites

__WARNING__: At the moment of writing (2023-03-01) Rust nightly build can't properly link binaries. As a workaround,
use `rustup default nightly-2023-01-31` instead of plan `rustup default nightly`.

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

#### Install Cargo Ledger, LedgerBlue and ledgerctl tools 

`cargo install --git https://github.com/LedgerHQ/cargo-ledger`
`python3 -m pip install ledgerblue`
``

## Build Commands

### Production

For building binaries for all supported targets, use following script:

```
prepare-binaries.sh
```

### Compiling and loading binaries for development/testing purposes

Following script builds debug binary for Ledger Nano S:
```shell
build-nanos.sh
```
Following script loads pre-built debug binary into Ledger Nano S:
```shell
flash-nanos.sh
```
For Ledger Nano S Plus corresponding commands are following:
```shell
build-nanos-plus.sh
```
and
```shell
flash-nanos-plus.sh
```
__WARNING:__ Binaries for different devices are incompatible. So, build and flash scripts
should be used in pairs and correspond to actual device.

### Individual Targets

#### Prerequisite
In order to work commands below require following environment variables to be set:
```shell
export LEDGER_TARGETS=./target-config
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

```

Build commands for individual targets:

```
cargo build --release -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=./target-config/nanos.json
cargo build --release -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=./target-config/nanosplus.json
cargo build --release -Z build-std=core -Z build-std-features=compiler-builtins-mem --target=./target-config/nanox.json
```

Note that these commands build application but do not prepare data necessary for flashing app to hardware wallet.
To prepare necessary data, use one of the following commands:

```
cargo ledger nanos
cargo ledger nanospus
cargo ledger nanox
```

### Development Device Setup (Nano S)

> ☣️ ONLY Use a dedicated Ledger device for development. Don't use one with "funds on".

The device used for development is configured with specific seed phrase, so generated keys could be predicted.
This is necessary for testing purposes.

#### Hardware Reset (needed only if device was in use)

In order to reset device to factory defaults, follow steps below:

- Plug device and enter PIN to unlock
- Enter Setting
    - Navigate to "Settings" menu and long press both buttons
- Select Settings -> Security -> Reset device
- Press right button until "Reset device" text appears
- Press both buttons to confirm choice
- Enter PIN to confirm hardware reset

#### Enter Recovery Mode

> ⚠️ Recovery mode could be entered only if device is brand new or after hardware reset. If device fails to enter
> recovery mode (shows PIN entry screen shortly after `Recovery``message), then device must be reset to factory settings.️

- Unplug device, press right button and while keeping it pressed, plug device back.
- Wait until "Recovery" word appears and release right button

#### Load development seed phrase and PIN

Use following command to load development seed phrase and set PIN on the development device to `5555`:

```sh
python3 -m ledgerblue.hostOnboard --apdu --id 0 --prefix "" --passphrase "" --pin 5555 --words "equip will roof matter pink blind book anxiety banner elbow sun young"
```

The process takes some time (few minutes) to finish. Once process finishes, device is ready to use for testing/development purposes.

### Testing

For testing there are a number of test scripts provided in `test` directory. To run tests, use following command (inside test directory):

```sh
./test-all.sh
```
