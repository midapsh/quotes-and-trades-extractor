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
chmod a+x entrypoints/bitmex_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/bitmex-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start bitmex-extractor

chmod a+x entrypoints/coinbase_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/coinbase-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start coinbase-extractor

chmod a+x entrypoints/ftx_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/ftx-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start ftx-extractor


chmod a+x entrypoints/log_cpu_resources.sh
sudo systemctl enable "/opt/exchange-extractor/services/log-cpu-resources.service"
sudo systemctl daemon-reload
sudo systemctl start log-cpu-resources

popd
popd