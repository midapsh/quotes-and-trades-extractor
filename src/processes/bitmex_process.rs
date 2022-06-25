use std::io::Write;

use bincode::config::{AllowTrailing, FixintEncoding, WithOtherIntEncoding, WithOtherTrailing};
use bincode::{DefaultOptions, Options};
use futures::{StreamExt, TryStreamExt};

use crate::commands::bitmex_subscribe::Args;
use crate::configs::configuration;
use crate::custom_parsers::bitmex_parser::{get_default_timestamp, BitmexParser};
use crate::data_extractors::bitmex_websocket::BitmexWebsocket;

type CustomBincode =
    WithOtherTrailing<WithOtherIntEncoding<DefaultOptions, FixintEncoding>, AllowTrailing>;

pub struct BitmexProcess<'a> {
    coin: &'a str,
    quotes_appender: tracing_appender::non_blocking::NonBlocking,
    trades_appender: tracing_appender::non_blocking::NonBlocking,
    _quotes_guard: tracing_appender::non_blocking::WorkerGuard,
    _trades_guard: tracing_appender::non_blocking::WorkerGuard,
    custom_bincode: CustomBincode,
}

impl<'a> BitmexProcess<'a> {
    pub fn new(coin: &'a str) -> Self {
        let file_name_prefix = format!("bitmex-{}.dat", coin);
        let settings = configuration::get_configuration().unwrap();
        let quotes_appender =
            tracing_appender::rolling::daily(settings.data_quotes_path, file_name_prefix.clone());
        let (non_blocking_quotes_appender, _quotes_guard) =
            tracing_appender::non_blocking(quotes_appender);

        let trades_appender =
            tracing_appender::rolling::daily(settings.data_trades_path, file_name_prefix);
        let (non_blocking_trades_appender, _trades_guard) =
            tracing_appender::non_blocking(trades_appender);

        let custom_bincode = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes();

        Self {
            coin,
            custom_bincode,
            quotes_appender: non_blocking_quotes_appender,
            trades_appender: non_blocking_trades_appender,
            _quotes_guard,
            _trades_guard,
        }
    }
}

impl<'a> BitmexProcess<'a> {
    // NOTE(hspadim): I use this because I need to remove the size of the vector,
    // the first value of the byte stream
    const USIZE_LEN: usize = 8;
    pub async fn run(&mut self) -> std::io::Result<()> {
        let mut stream = BitmexWebsocket::connect(Args::WithProduct(vec![
            String::from(format!("quote:{}", self.coin)),
            String::from(format!("trade:{}", self.coin)),
        ]))
        .await
        .unwrap();

        loop {
            let msg = stream.try_next().await.map_err(|x| {
                std::io::Error::new(std::io::ErrorKind::Other, format!("error code: {x}"))
            })?;

            let default_timestamp = get_default_timestamp();
            match msg {
                Some(tokio_tungstenite::tungstenite::Message::Text(message))
                    if message.find("table").is_some() =>
                {
                    let bitmex_msg: BitmexParser = serde_json::from_str(&message).unwrap();
                    match bitmex_msg {
                        BitmexParser::Quotes(quotes) => {
                            let mut quotes = quotes.data;
                            for quote in &mut quotes {
                                quote.default_timestamp = default_timestamp;
                            }
                            let bin_quotes = self.custom_bincode.serialize(&quotes).unwrap();
                            // f_write_quotes.write_all(&bin_quotes[USIZE_LEN..]).await?;
                            // f_write_quotes.flush().await?;
                            self.quotes_appender
                                .write_all(&bin_quotes[Self::USIZE_LEN..])?;
                            self.quotes_appender.flush()?;
                        }
                        BitmexParser::Trades(trades) => {
                            let mut trades = trades.data;
                            for trade in &mut trades {
                                trade.default_timestamp = default_timestamp;
                            }
                            let bin_trades = self.custom_bincode.serialize(&trades).unwrap();
                            self.trades_appender
                                .write_all(&bin_trades[Self::USIZE_LEN..])?;
                            self.trades_appender.flush()?;
                        }
                        _ => (),
                    }
                }
                // tokio_tungstenite::tungstenite::Message::Binary(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Ping(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Pong(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Close(_) => todo!(),
                _other => (),
            }
        }
    }
}
