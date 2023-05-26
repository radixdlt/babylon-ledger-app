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
RUN echo "Installing Rust"
RUN /install-rust.sh
RUN echo "Installation done"
RUN ls -la /root/

ENV PATH="/root/.cargo/bin:${PATH}"
ENV LEDGER_TARGETS=/app/target-config
RUN /root/.cargo/bin/rustup update
RUN /root/.cargo/bin/rustup target install thumbv7m-none-eabi
RUN /root/.cargo/bin/rustup default nightly
RUN /root/.cargo/bin/rustup component add rust-src
RUN /root/.cargo/bin/cargo install --git https://github.com/radixdlt/cargo-ledger.git
RUN pip3 install --upgrade protobuf setuptools ecdsa && pip3 install ledgerwallet && pip3 install protobuf==3.20.3


WORKDIR /app

CMD ["/bin/bash"]
