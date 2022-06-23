#!/bin/bash
OPT_PATH="/opt/trading-system"
pushd ~/Documents/quotes-and-trades-extractor
git pull
cp -r -u configuration/* $OPT_PATH/exchange-extractor/configuration &
cp -r -u entrypoints/* $OPT_PATH/exchange-extractor/entrypoints &
cp -r -u services/* $OPT_PATH/exchange-extractor/services &
cp -r -u target/release/* $OPT_PATH/exchange-extractor/bin
# Stop everything
sudo systemctl stop binance-coin-extractor & 
sudo systemctl stop binance-spot-extractor & 
sudo systemctl stop binance-stable-extractor & 
sudo systemctl stop bitmex-extractor & 
sudo systemctl stop bitstamp-extractor & 
sudo systemctl stop coinbase-extractor & 
sudo systemctl stop deribit-extractor & 
sudo systemctl stop ftx-extractor & 
sudo systemctl stop kraken-extractor & 
sudo systemctl stop kraken-futures-extractor & 
sudo systemctl stop okx-extractor

sudo systemctl daemon-reload
# Start everything
sudo systemctl start binance-coin-extractor &
sudo systemctl start binance-spot-extractor &
sudo systemctl start binance-stable-extractor &
sudo systemctl start bitmex-extractor &
sudo systemctl start bitstamp-extractor &
sudo systemctl start coinbase-extractor &
sudo systemctl start deribit-extractor &
sudo systemctl start ftx-extractor &
sudo systemctl start kraken-extractor &
sudo systemctl start kraken-futures-extractor &
sudo systemctl start okx-extractor
popd
popd
