# Radix Babylon Ledger App (WIP)

Ledger Nano S/S Plus/X app for Babylon

## Branching Policy
This repository follows quite traditional branching policy:
- `main` contains the current released version
- `develop` collects all changes planned for inclusion into the next release

## Changes Summary
- Separation of the UI components. This is necessary for the future implementation of the Stax device support.
- Fixes for UI according to the comments from Ledger team:
  - Dashboard icon position (should be at the right)
  - Double press on "Done" for verify address action
  - No right arrow on the Nano S+/X on the "Review Transaction" screen
  - Missing icon on Nano S+/X on the "Blind signing.." screen
- Support for pre-auth subintent hash signing

## Build

Simplest way to build app is to use [application builder container](https://github.com/LedgerHQ/ledger-app-builder) shell provided by Ledger. 
In particular, most convenient variant is the one which enables testing along with the building.
For convenience, one can use script provided in the project root directory:

```shell
run-dev-tools.sh
```
Once the container shell is running, use following commands to build and tests the application:

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

## Sideloading
> [!IMPORTANT]
> Sideloading does not work on Ledger Nano X

> [!IMPORTANT]
> Sideload **not** from *shell* (you can run `exit` to get out from `./run-dev-tools.sh`), but rather from *host*.

You can install the locally built binaries onto your physical Ledger Nano S, or Nano S Plus by running a flash script from *host*, not from *shell*. Note that Nano X does not support this.

But before doing so you must install the prerequisites on your host. This requires Python3.

```sh
python3 -m pip config set global.break-system-packages true && pip3 install protobuf==3.20.3 && pip3 install ledgerwallet==0.5.1
```

> [!NOTE]
> I did not manage to get it working using `venv`. But when I used `global.break-system-packages true` it worked.
> Verified to work with Secure Elements firmware version `1.1.2` on Nano S Plus.

If you get hit by the notorious [Invalid status 6512 (Unknown Reason)](https://github.com/LedgerHQ/ledgerctl/issues/65) error, then you might need to try a newer version of [`ledgerwallet`](https://github.com/LedgerHQ/ledgerctl/releases) and or newer firmware.

Then you can finally sideload the built binary, like so:

```sh
./flash-nanosplus.sh
```

or alternatively for Nano S

```sh
./flash-nanos.sh
```


If you see seomthing like:
```sh
Dumping APDU installation file to /Users/alexandercyon/Developer/babylon-ledger-app/target/nanosplus/release/babylon-ledger-app.apdu
[WARNING] JSON files will be deprecated in future version
[WARNING] JSON files will be deprecated in future version
```
then you probably have succeeded.

> [!NOTE]
> Flashing of Nano S Plus seemed harded than Nano S. So if you start with Nano S, there are no guarantuees that it will work with Nano S Plus.

## Local Build Environment Setup

This type of setup in details described [here](https://github.com/LedgerHQ/app-boilerplate-rust).

## Development Device Setup (Nano S)

> [!CAUTION]
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

> [!NOTE]
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
