#!/bin/bash

# First time
PROJECT_HOME="/opt"
PROJECT_HOWTO_TCS_HOME="$PROJECT_HOME/exchange-extractor"
pushd $PROJECT_HOME
git config --global user.name "Henrique Spadim"
git config --global user.email "henrique@spadim.com.br"
git config --global credential.helper cache
git clone -b $PROJECT_HOWTO_TCS_GIT_BRANCH --single-branch https://github.com/midapsh/exchange-extractor exchange-extractor
pushd "$PROJECT_HOWTO_TCS_HOME"
git checkout $PROJECT_HOWTO_TCS_GIT_BRANCH

# Rust things
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
sudo apt-get install -y pkg-config
sudo apt-get install -y libssl-dev
sudo apt-get install -y build-essential
cargo check
cargo build --release

# Systemd things
sudo systemctl enable "$PROJECT_HOWTO_TCS_HOME/services/exchange-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start exchange-extractor
popd
popd