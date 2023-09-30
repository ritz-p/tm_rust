FROM rust:latest

RUN apt update
WORKDIR /workspace
ENV RUST_BACKTRACE=1