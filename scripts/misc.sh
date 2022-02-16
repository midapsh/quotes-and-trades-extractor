
sudo systemctl enable "/opt/exchange-extractor/services/exchange-extractor.service"
sudo systemctl daemon-reload
sudo systemctl start exchange-extractor


sudo systemctl status exchange-extractor
sudo systemctl stop exchange-extractor


journalctl -u exchange-extractor


chmod a+x /opt/exchange-extractor/entrypoints/exchange_extractor_entrypoint.sh



sudo heaptrack target/release/exchange_extractor
heaptrack_gui "heaptrack.exchange_extractor.76992.gz"

####

valgrind \
    --leak-check=full \
    --show-leak-kinds=all \
    target/release/exchange_extractor

####

sudo heaptrack target/release/exchange_extractor
heaptrack_gui "heaptrack.exchange_extractor.60314.gz"

####

RUSTFLAGS="-Z sanitizer=memory" cargo run --target x86_64-unknown-linux-gnu

