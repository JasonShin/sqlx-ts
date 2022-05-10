# This dockerfile is for testing only
# Use this dockerfile to simulate linux build in github action

FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y
RUN apt install -y libssl-dev
RUN apt install -y pkg-config
RUN apt install -y build-essential
RUN apt install -y git
RUN apt install -y curl

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

WORKDIR /root/code
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add x86_64-apple-darwin

RUN cargo build --release --target x86_64-unknown-linux-musl --locked
RUN cargo build --release --target x86_64-apple-darwin --locked
