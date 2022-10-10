FROM amd64/ubuntu:22.04
ENV LANG C.UTF-8

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get upgrade -qy && \
    apt-get install -qy \
        curl \
        git \
        clang \
	gcc-arm-none-eabi \
        binutils-arm-none-eabi \
        gcc-multilib \
        python3 \
        python3-pip \
	python3-dev \
        libusb-1.0-0-dev \
        libudev-dev && \
    apt-get autoclean -y && \
    apt-get autoremove -y && \
    apt-get clean

RUN echo "(curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly --no-modify-path)" > /install-rust.sh && chmod 755 /install-rust.sh
RUN /install-rust.sh
ENV PATH="/root/.cargo/bin:${PATH}"
ENV LEDGER_TARGETS=/app/target-config
RUN rustup update && \
    rustup target install thumbv7m-none-eabi && \
    cargo install --git https://github.com/LedgerHQ/cargo-ledger && \
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN pip3 install --upgrade protobuf setuptools ecdsa && pip3 install ledgerwallet


WORKDIR /app

CMD ["/bin/bash"]
