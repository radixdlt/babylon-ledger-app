# How to install Radix Babylon App on Ledger Nano S on macOS

## Prerequistes

- docker installed
- python3 installed
- rust installed
- arm-none-eabi installed and path set
  - download and run installer https://developer.arm.com/-/media/Files/downloads/gnu/12.2.mpacbti-rel1/binrel/arm-gnu-toolchain-12.2.mpacbti-rel1-darwin-x86_64-arm-none-eabi.pkg?rev=2f38f68c2683438e895886abee9be5fc&hash=A0BB95236291FB90466A82ED4F7B11B6
  - add path `export PATH="$PATH:/Applications/ArmGNUToolchain/12.2.mpacbti-rel1/arm-none-eabi/bin”`

## Step-by-step

Clone the repo and cd into app-builder directory

```
cd ./babylon-ledger-app/app-builder
```

Build the docker image by running

```
./build-image.sh
```

Go back to the repo root directory

```
cd ..
```

Run the app builder. **NOTE!** This will open the app builder shell.

```
./run-app-builder.sh
```

Whilst in the app builder shell. Run command

```
./build-nanos.sh
```

Exit the app builder shell

```
ctrl + d
```

Run command. **NOTE!** at this point you are in the host shell.

```
python3 -m pip install ledgerblue
python3 -m pip install --upgrade protobuf setuptools ecdsa
python3 -m pip install ledgerwallet
python3 -m pip install protobuf==3.20.3
cargo install --git https://github.com/radixdlt/cargo-ledger.git
```

Connect and unlock ledger nano S and run command. **NOTE!** at this point you are in the host shell.

```
./flash-nanos.sh
```
