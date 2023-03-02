FROM amd64/ubuntu:22.04
ENV LANG C.UTF-8

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get upgrade -qy && \
    apt-get install -qy \
	curl \
	git \
	clang \
	gcc-multilib \
	gcc-arm-none-eabi \
	binutils-arm-none-eabi \
	python3 \
	python3-pip \
	libusb-1.0-0-dev \
	libudev-dev && \
    apt-get autoclean -y && \
    apt-get autoremove -y && \
    apt-get clean

RUN echo "(curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly --no-modify-path)" > /install-rust.sh && chmod 755 /install-rust.sh
RUN /install-rust.sh
ENV PATH="/root/.cargo/bin:${PATH}"
ENV LEDGER_TARGETS=/app/target-config
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN rustup update && \
    rustup target install thumbv7m-none-eabi && \
    rustup default nightly-2023-01-31 && \
    cargo install -Z sparse-registry --git https://github.com/LedgerHQ/cargo-ledger && \
    rustup component add rust-src
RUN pip3 install --upgrade protobuf setuptools ecdsa && pip3 install ledgerwallet

WORKDIR /app

CMD ["/bin/bash"]
