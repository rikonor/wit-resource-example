FROM debian:12-slim

RUN \
    apt update && apt install -y \
        curl \
        build-essential \
        make \
        git

RUN \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add wasm32-wasip2
RUN cargo install cargo-binstall
RUN cargo binstall cargo-component
