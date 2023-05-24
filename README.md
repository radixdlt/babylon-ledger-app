# Radix Babylon Ledger App (WIP)

Ledger Nano S/S Plus/X app for Babylon

## Build

Simplest way to build app is to use application builder container. To build container, use script provided
in the `app-builder` directory. Once container is built, run it using `run-app-builder.sh` and then build
binaries using `prepare-binaries.sh` script.

## Local Build Environment Setup

Instructions are provided for Ubuntu 22.04. For other operating systems it is suggested to use 
Docker builder image. Refer to builder image [documentation](./app-builder/README.md) for details.

### Prerequisites

#### Install ARM GCC, Binutils, cross-compilation headers:

```shell
sudo apt-get install gcc-arm-none-eabi binutils-arm-none-eabi gcc-multilib
```

#### Install Python 3 and Pip3

```shell
sudo apt-get install python3 python3-pip
```

#### Install Clang

```shell
sudo apt-get install clang
```

#### Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
rustup component add rust-src
rustup target add thumbv6m-none-eabi
```

#### Install Cargo Ledger, LedgerBlue and ledgerctl tools 

```shell
cargo install --git https://github.com/LedgerHQ/cargo-ledger
python3 -m pip install ledgerblue
```

## Build Commands

### Production

For building binaries for all supported targets, use following script:

```shell
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

Build commands for individual targets:

```shell
./build-nanos.sh
./build-nanos-plus.sh
```

Flash commands for individual targets:

```shell
./flash-nanos.sh
./flash-nanos-plus.sh
```

Note that there are no individual scripts for Nano X since it does not support sideloading.

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

```shell
python3 -m ledgerblue.hostOnboard --apdu --id 0 --prefix "" --passphrase "" --pin 5555 --words "equip will roof matter pink blind book anxiety banner elbow sun young"
```

The process takes some time (few minutes) to finish. Once process finishes, device is ready to use for testing/development purposes.

### Testing

For testing there are a number of test scripts provided in `test` directory. 
To run tests, use following command (inside test directory):

```shell
./test-all-release.sh
```
Debug builds have support for additional commands, so there are additional tests for them.
To run tests for debug builds, use following command (inside test directory):

```shell
./test-all-debug.sh
```

There are also a number of tests which require user interaction. They should be run separately,
using following commands (inside test directory):

```shell
python3 -m test-sign-auth-ed25519
python3 -m test-sign-auth-secp256k1
python3 -m test-sign-tx-ed25519
python3 -m test-sign-tx-ed25519-hash
python3 -m test-sign-tx-ed25519-summary
python3 -m test-sign-tx-secp256k1 
python3 -m test-sign-tx-secp256k1-hash
python3 -m test-sign-tx-secp256k1-summary
```

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.