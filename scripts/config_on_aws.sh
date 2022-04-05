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

chmod a+x entrypoints/bitmex_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/bitmex-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start bitmex-extractor
sudo systemctl status bitmex-extractor
sudo systemctl stop bitmex-extractor

chmod a+x entrypoints/coinbase_extractor_entrypoint.sh
sudo systemctl enable "/opt/exchange-extractor/services/coinbase-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start coinbase-extractor
sudo systemctl status coinbase-extractor
sudo systemctl stop coinbase-extractor



chmod a+x entrypoints/log_cpu_resources.sh
sudo systemctl enable "/opt/exchange-extractor/services/log-cpu-resources.service"
sudo systemctl daemon-reload
sudo systemctl start log-cpu-resources
sudo systemctl status log-cpu-resources
sudo systemctl stop log-cpu-resources



