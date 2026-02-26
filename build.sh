#!/bin/bash
set -e

# Install Rust toolchain if not present
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Add WASM target
rustup target add wa32-unknown-unknown

# Download and install trunk
wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.6/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
chmod +x trunk

# Build the project for release
./trunk build --release
