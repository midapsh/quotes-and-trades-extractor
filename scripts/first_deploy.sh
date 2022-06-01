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
chmod a+x entrypoints/binance_coin_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/binance_coin-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start binance-coin-extractor

chmod a+x entrypoints/binance_spot_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/binance_spot-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start binance-spot-extractor

chmod a+x entrypoints/binance_stable_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/binance_stable-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start binance-stable-extractor

chmod a+x entrypoints/bitmex_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/bitmex-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start bitmex-extractor

chmod a+x entrypoints/bitstamp_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/bitstamp-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start bitstamp-extractor

chmod a+x entrypoints/coinbase_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/coinbase-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start coinbase-extractor

chmod a+x entrypoints/deribit_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/deribit-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start deribit-extractor

chmod a+x entrypoints/ftx_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/ftx-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start ftx-extractor

chmod a+x entrypoints/kraken_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/kraken-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start kraken-extractor

chmod a+x entrypoints/kraken_futures_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/kraken-futures-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start kraken-futures-extractor

chmod a+x entrypoints/okx_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/okx-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start okx-extractor


popd
popd