# This dockerfile is for testing only
# Use this dockerfile to simulate linux build in github action

FROM pactfoundation/rust-musl-build:latest

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update -y

WORKDIR /root/code
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add x86_64-apple-darwin

RUN cargo build --release --target x86_64-unknown-linux-musl --locked
RUN cargo build --release --target x86_64-apple-darwin --locked
