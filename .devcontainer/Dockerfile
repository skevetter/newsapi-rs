ARG VARIANT="bookworm"
FROM mcr.microsoft.com/devcontainers/base:bookworm

ENV DEBIAN_FRONTEND=noninteractive
ENV USERNAME=vscode

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN \
    echo "${USERNAME}:${USERNAME}" | chpasswd \
    && echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/${USERNAME} \
    && chsh -s /usr/bin/zsh ${USERNAME}

COPY --chown=${USERNAME}:${USERNAME} ./dotfiles/. /home/${USERNAME}/
