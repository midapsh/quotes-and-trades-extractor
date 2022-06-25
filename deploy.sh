#!/bin/bash
OPT_PATH="/opt/trading-system"
pushd ~/Documents/quotes-and-trades-extractor
git pull
# Get Cargo envs to run it 
source ~/.cargo/env
cargo build --release --bins
# Stop everything
sudo systemctl stop binance-coin-extractor.service
sudo systemctl stop binance-spot-extractor.service
sudo systemctl stop binance-stable-extractor.service
sudo systemctl stop bitmex-extractor.service
sudo systemctl stop bitstamp-extractor.service
sudo systemctl stop coinbase-extractor.service
sudo systemctl stop deribit-extractor.service
sudo systemctl stop ftx-extractor.service
sudo systemctl stop kraken-extractor.service
sudo systemctl stop kraken-futures-extractor.service
sudo systemctl stop okx-extractor.service

cp -r -u configuration/* $OPT_PATH/exchange-extractor/configuration
cp -r -u entrypoints/* $OPT_PATH/exchange-extractor/entrypoints
cp -r -u services/* $OPT_PATH/exchange-extractor/services
cp -r -u target/release/* $OPT_PATH/exchange-extractor/bin
sudo systemctl daemon-reload
# Start everything
sudo systemctl start binance-coin-extractor.service
sudo systemctl start binance-spot-extractor.service
sudo systemctl start binance-stable-extractor.service
sudo systemctl start bitmex-extractor.service
sudo systemctl start bitstamp-extractor.service
sudo systemctl start coinbase-extractor.service
sudo systemctl start deribit-extractor.service
sudo systemctl start ftx-extractor.service
sudo systemctl start kraken-extractor.service
sudo systemctl start kraken-futures-extractor.service
sudo systemctl start okx-extractor.service
popd
