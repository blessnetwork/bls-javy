FROM ubuntu:20.04

COPY . .
COPY target/wasm32-wasip1/release/plugin.wasm target/wasm32-wasip1/release/plugin.wasm
ENV TARGET=x86_64-unknown-linux-gnu
RUN apt-get update -y && apt-get install -y gcc-x86-64-linux-gnu g++-x86-64-linux-gnu curl
SHELL ["/bin/bash", "-c"]
RUN echo $HOME
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |  bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
RUN source $HOME/.cargo/env && cargo build --release --target ${TARGET} --package javy-cli
