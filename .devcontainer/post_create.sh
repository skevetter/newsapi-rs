#!/bin/bash

set -e

sudo cp .devcontainer/welcome.txt /usr/local/etc/vscode-dev-containers/first-run-notice.txt

sudo chown -R vscode:vscode /cmd_history

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

sudo apt update && sudo apt install pkg-config -y
cargo build && cargo test --all-features

sudo ln -s /home/vscode/.cargo/bin/cargo /usr/bin/cargo

pre-commit install --install-hooks
pre-commit run --all-files

TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"
LAZYGIT_VERSION=$(curl -s "https://api.github.com/repos/jesseduffield/lazygit/releases/latest" | grep -Po '"tag_name": *"v\K[^"]*')
curl -Lo lazygit.tar.gz "https://github.com/jesseduffield/lazygit/releases/download/v${LAZYGIT_VERSION}/lazygit_${LAZYGIT_VERSION}_Linux_x86_64.tar.gz"
tar xf lazygit.tar.gz lazygit
sudo install lazygit -D -t /usr/local/bin/
cd - >/dev/null
rm -rf "$TMP_DIR"
