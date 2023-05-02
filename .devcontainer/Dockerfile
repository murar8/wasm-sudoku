FROM rust:1

ENV DEBIAN_FRONTEND="noninteractive"

# Installs a set of common command line utilities and sets up a non-root user.
# See https://github.com/microsoft/vscode-dev-containers/blob/master/script-library/docs/common.md

ARG INSTALL_ZSH="false"
ARG USERNAME="rust"
ARG USER_UID="1000"
ARG USER_GID="${USER_UID}"
ARG UPGRADE_PACKAGES="true"
ARG INSTALL_OH_MYS="false"
ARG ADD_NON_FREE_PACKAGES="true"

RUN curl -fsSL "https://raw.githubusercontent.com/microsoft/vscode-dev-containers/main/script-library/common-debian.sh" \
    | bash -s - \
    "${INSTALL_ZSH}" \
    "${USERNAME}" \
    "${USER_UID}" \
    "${USER_GID}" \
    "${UPGRADE_PACKAGES}" \
    "${INSTALL_OH_MYS}" \
    "${ADD_NON_FREE_PACKAGES}"


# Installs Node.js, nvm, yarn, and needed dependencies.
# See https://github.com/microsoft/vscode-dev-containers/blob/v0.209.6/script-library/docs/node.md

ARG NVM_DIR=""
ARG NODE_VERSION="lts"
ARG USERNAME="rust"
ARG UPDATE_RC="true"
ARG INSTALL_TOOLS_FOR_NODE_GYP="false"

RUN curl -fsSL "https://raw.githubusercontent.com/microsoft/vscode-dev-containers/main/script-library/node-debian.sh" \
    | bash -s - \
    "${NVM_DIR}" \
    "${NODE_VERSION}" \
    "${USERNAME}" \
    "${UPDATE_RC}" \
    "${INSTALL_TOOLS_FOR_NODE_GYP}"

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends firefox-esr \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

USER rust

RUN rustup component add rust-src \
    && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
