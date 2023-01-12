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

#### Install Cargo Ledger and LedgerBlue (format conversion and upload to device tools)

`cargo install --git https://github.com/LedgerHQ/cargo-ledger`
`python3 -m pip install ledgerblue`

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

If necessary, binaries also could be uploaded to target device (must be attached and unlocked by the moment, when
command is executed).
To do this, add `-l` flag to the `cargo ledger` command, as follows:

```
cargo ledger nanos -l
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

The process takes some time (few minutes) to finish. Once

### Testing

#### Testing of "Get API Version" call

- Load binaries into attached device (device may ask several questions)
- Select "Radix Babylon" application on device (device may ask several questions)
- Run `test/get-api-version/show-api-version.sh` script. It should output human-readable version of the installed
  binaries of the application to the console.
