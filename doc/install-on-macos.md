# How to install Radix Babylon App on Ledger Nano S on macOS

## Prerequistes

- docker installed
- python3 installed
- rust installed
- arm-none-eabi installed and path set
  - download and run installer https://developer.arm.com/-/media/Files/downloads/gnu/12.2.mpacbti-rel1/binrel/arm-gnu-toolchain-12.2.mpacbti-rel1-darwin-x86_64-arm-none-eabi.pkg?rev=2f38f68c2683438e895886abee9be5fc&hash=A0BB95236291FB90466A82ED4F7B11B6
  - add path `export PATH="$PATH:/Applications/ArmGNUToolchain/12.2.mpacbti-rel1/arm-none-eabi/bin”`

## Step-by-step

Clone repo and cd into app-builder directory

```
cd ./babylon-ledger-app/app-builder
```

Build the docker image by running

```
docker build -t app-radix-builder:latest .
```

Go back to root directory

```
cd ..
```

Run the container

```
docker run --rm -ti -v "$(realpath .):/app" app-radix-builder:latest
```

Run command

```
sh ./build-nanos.sh
```

Exit container

```
cmd + d
```

Run command

```
python3 -m pip install ledgerblue
python3 -m pip install --upgrade protobuf setuptools ecdsa
python3 -m pip install ledgerwallet
cargo install --git  https://github.com/siy/cargo-ledger.git
```

Connect and unlock ledger nano S and run command

```
sh ./flash-nanos.sh
```
