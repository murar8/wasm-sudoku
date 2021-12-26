FROM mcr.microsoft.com/vscode/devcontainers/rust:latest

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && \
    apt-get -y install --no-install-recommends firefox-esr

USER vscode

RUN rustup toolchain install nightly && \
    rustup default nightly && \
    rustup component add rust-src rustfmt clippy

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
