# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.209.6/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && \
    apt-get -y install --no-install-recommends firefox-esr

USER vscode

RUN rustup toolchain install nightly && \
    rustup default nightly && \
    rustup component add rust-src rustfmt clippy

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
