# Radix Babylon Ledger App (WIP)

Ledger Nano S/S Plus/X app for Babylon

## Branching Policy
This repository follows quite traditional branching policy:
- `main` contains the current released version
- `develop` collects all changes planned for inclusion into the next release

## Future Version Changes Summary
The code in this branch contains following changes:
- Separation of the UI components. This is necessary for the future implementation of the Stax device support.
- (WIP) Fixes for UI according to the comments from Ledger team:

## Build

Simplest way to build app is to use [application builder container](https://github.com/LedgerHQ/ledger-app-builder) provided by Ledger. 
In particular, most convenient variant is the one which enables testing along with the building.
For convenience, one can use script provided in the project root directory:

```shell
run-dev-tools.sh
```
Once container is running, use following commands to build and tests the application:

Build for Nano S:
```shell
./build-nanos.sh
```
Build for Nano S Plus:
```shell
./build-nanosplus.sh
```
Build for Nano X:
```shell
./build-nanox.sh
``` 

To test application, change directory inside container to the one where tests are residing:

```shell
cd ragger_tests
```

Then run tests for each target:

For Nano S:
```shell
./test-nanos.sh
```
For Nano S Plus:
```shell
./test-nanosplus.sh
```
For Nano X:
```shell
./test-nanox.sh
```

## Local Build Environment Setup

This type of setup in details described [here](https://github.com/LedgerHQ/app-boilerplate-rust).

## Development Device Setup (Nano S)

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

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.
