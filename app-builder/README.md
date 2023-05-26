# RDX Works Application Builder

Setting up a build environment for the Radix Ledger App is quite painful process and the setup works only for
Linux and only on x86 hardware. To overcome these limitations, developer can use a Docker image with pre-configured
development environment setup.

This directory contains Docker configuration file and necessary files to build an image with build environment inside.
Once image is built, it can be used to build Radix Ledger App binaries.

### Application Builder Image Build

In order to build the image, it is enough to issue following command in the directory where `Dockerfile` resides:

```shell
docker build -t app-radix-builder:latest .
```
For convenience this command also present as a shell script:
```shell
build-image.sh
```
In some cases command may require `sudo` to obtain necessary privileges.

### Using Application Builder Image

Go to the root of the project directory and execute following command:
(just like image building, this command may require use of `sudo` to obtain necessary privileges)

```shell
docker run --rm -ti -v "$(realpath .):/app" app-radix-builder:latest
```

The command above opens a shell with project directory linked to `/app` directory inside image.
This enables convenient building of the binaries and transparent sharing of the files between project directory on the
host machine and `/app` directory inside the image.
For convenience command above also present as shell script located in project root directory:
```shell
run-app-builder.sh
```

For development purposes it is convenient to use following command to build debug binaries for Ledger Nano S:
```shell
build-nanos.sh
```
Similar command for Nano S Plus:
```shell
build-nanos-plus.sh
```

### Flashing Built Firmware
Since flashing requires access to USB, it should be done from the host operating system.
This means that following tools should be installed on the __host OS__:
- Python 3, ledgerblue and ledgerctl
- Rust
- cargo-ledger utility

Commands below can be used on any OS, but they assume that Python 3 and Rust already installed in the OS-specific way.

Installing `ledgerblue` utility:
```shell
python3 -m pip install ledgerblue
```
Installing `ledgerctl` utility:
```shell
python3 -m pip install --upgrade protobuf setuptools ecdsa
python3 -m pip install ledgerwallet
python3 -m pip install protobuf==3.20.3
```
Installing `cargo-ledger`:

```shell
cargo install --git https://github.com/radixdlt/cargo-ledger.git
```
Once necessary tools successfully installed, following command can be used to flash firmware into Ledger Nano S:
```shell
flash-nanos.sh
```
Similar command for Ledger Nano S Plus:
```shell
flash-nanos-plus.sh
```
__WARNING:__ Binaries for Nano S and Nano S Plus are incompatible, hence it is mandatory to use correct build and flash 
commands which correspond actual device used for development.
