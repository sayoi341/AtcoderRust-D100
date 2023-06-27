FROM rust

WORKDIR /workspace/atcoder

RUN cargo install cargo-compete && rustup install 1.42.0