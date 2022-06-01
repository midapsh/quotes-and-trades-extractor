# Connect
ssh -i /path/to/pem-file.pem user@host

# Configs

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
sudo apt-get install -y pkg-config
sudo apt-get install -y libssl-dev
cargo check
cargo build --release
sudo apt-get install build-essential


heaptrack

sudo apt-get update -y
sudo apt-get install -y heaptrack



chmod a+x entrypoints/binance_coin_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/binance_coin-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start binance-coin-extractor
sudo systemctl start status-coin-extractor
sudo systemctl start stopce-coin-extractor

chmod a+x entrypoints/binance_spot_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/binance_spot-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start binance-spot-extractor
sudo systemctl start status-spot-extractor
sudo systemctl start stopce-spot-extractor

chmod a+x entrypoints/binance_stable_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/binance_stable-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start binance-stable-extractor
sudo systemctl start status-stable-extractor
sudo systemctl start stopce-stable-extractor

chmod a+x entrypoints/bitmex_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/bitmex-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start bitmex-extractor
sudo systemctl status bitmex-extractor
sudo systemctl stop bitmex-extractor

chmod a+x entrypoints/bitstamp_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/bitstamp-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start bitstamp-extractor
sudo systemctl status bitstamp-extractor
sudo systemctl stop bitstamp-extractor

chmod a+x entrypoints/coinbase_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/coinbase-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start coinbase-extractor
sudo systemctl status coinbase-extractor
sudo systemctl stop coinbase-extractor

chmod a+x entrypoints/deribit_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/deribit-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start deribit-extractor
sudo systemctl status deribit-extractor
sudo systemctl stop deribit-extractor

chmod a+x entrypoints/ftx_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/ftx-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start ftx-extractor
sudo systemctl status ftx-extractor
sudo systemctl stop ftx-extractor

chmod a+x entrypoints/kraken_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/kraken-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start kraken-extractor
sudo systemctl status kraken-extractor
sudo systemctl stop kraken-extractor

chmod a+x entrypoints/kraken_futures_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/kraken-futures-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start kraken-futures-extractor
sudo systemctl status kraken-futures-extractor
sudo systemctl stop kraken-futures-extractor

chmod a+x entrypoints/okx_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/okx-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start okx-extractor
sudo systemctl status okx-extractor
sudo systemctl stop okx-extractor

