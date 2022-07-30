use bincode::config::{AllowTrailing, FixintEncoding, WithOtherIntEncoding, WithOtherTrailing};
use bincode::{DefaultOptions, Options};
use futures::{SinkExt, TryStreamExt};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use tokio::time::{sleep, Instant};

use crate::commands::bitmex_subscribe::Args;
use crate::custom_parsers::bitmex_parser::{
    get_default_timestamp, BitmexParser, ParsedQuote, ParsedTrade,
};
use crate::data_extractors::bitmex_websocket::BitmexWebsocket;
use crate::models::bitmex_models::{Quote, Trade};

type CustomBincode =
    WithOtherTrailing<WithOtherIntEncoding<DefaultOptions, FixintEncoding>, AllowTrailing>;

pub struct QuotesTriple {
    appender: tracing_appender::non_blocking::NonBlocking,
    _guard: tracing_appender::non_blocking::WorkerGuard,
    quotes: Vec<Quote>,
}

pub struct TradesTriple {
    appender: tracing_appender::non_blocking::NonBlocking,
    _guard: tracing_appender::non_blocking::WorkerGuard,
    trades: Vec<Trade>,
}

pub struct BitmexProcess<'a> {
    instrument: &'a str,
    _instrument_parsed: &'a str,
    quotes_triples: QuotesTriple,
    trades_triples: TradesTriple,
    custom_bincode: CustomBincode,
}

impl<'a> BitmexProcess<'a> {
    pub fn new(
        instrument: &'a str,
        instrument_parsed: &'a str,
        data_quotes_path: &PathBuf,
        data_trades_path: &PathBuf,
    ) -> Self {
        let file_name_prefix = format!("{}.dat", instrument_parsed);
        let quotes_appender =
            tracing_appender::rolling::daily(data_quotes_path, file_name_prefix.clone());
        let (non_blocking_quotes_appender, _quotes_guard) =
            tracing_appender::non_blocking(quotes_appender);

        let trades_appender = tracing_appender::rolling::daily(data_trades_path, file_name_prefix);
        let (non_blocking_trades_appender, _trades_guard) =
            tracing_appender::non_blocking(trades_appender);

        let quotes_triples = QuotesTriple {
            appender: non_blocking_quotes_appender,
            _guard: _quotes_guard,
            quotes: Vec::with_capacity(4096),
        };
        let trades_triples = TradesTriple {
            appender: non_blocking_trades_appender,
            _guard: _trades_guard,
            trades: Vec::with_capacity(4096),
        };

        let custom_bincode = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes();

        Self {
            instrument,
            _instrument_parsed: instrument_parsed,
            custom_bincode,
            quotes_triples,
            trades_triples,
        }
    }
}

impl<'a> BitmexProcess<'a> {
    // NOTE(hspadim): I use this because I need to remove the size of the vector,
    // the first value of the byte stream
    const USIZE_LEN: usize = 8;
    pub async fn run(&mut self) -> std::io::Result<()> {
        let subscription = [
            String::from(format!("quote:{}", self.instrument)),
            String::from(format!("trade:{}", self.instrument)),
        ];

        loop {
            let stream =
                match BitmexWebsocket::connect(Args::WithProduct(subscription.to_vec())).await {
                    Ok(it) => it,
                    Err(err) => {
                        // TODO(hspadim): Should try to connect to another symbol
                        eprintln!("{}", err);
                        return Ok(());
                    }
                };

            match self.stream_loop(stream).await {
                Ok(()) => {
                    sleep(tokio::time::Duration::from_secs(5)).await;
                }
                Err(_) => {
                    sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn stream_loop(
        &mut self,
        mut stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    ) -> std::io::Result<()> {
        match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            stream.send(tokio_tungstenite::tungstenite::Message::Ping(vec![0x9])), // Raw Ping Frame
        )
        .await
        {
            Ok(_) => (),
            Err(_) => {
                println!("Couldn't send 'ping' message");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "Couldn't send 'ping' message",
                ));
            }
        }

        let mut has_timed_out: bool = false;
        loop {
            let (default_timestamp, msg) = if let Ok(msg) =
                tokio::time::timeout(std::time::Duration::from_secs(5), stream.try_next()).await
            {
                let default_timestamp = get_default_timestamp();
                (
                    default_timestamp,
                    msg.map_err(|x| {
                        std::io::Error::new(std::io::ErrorKind::Other, format!("error code: {x}"))
                    })?,
                )
            } else {
                if has_timed_out {
                    println!("Couldn't send 'ping' message");
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::BrokenPipe,
                        "Couldn't send 'ping' message",
                    ));
                }
                if let Ok(_) = tokio::time::timeout(
                    std::time::Duration::from_secs(5),
                    stream.send(tokio_tungstenite::tungstenite::Message::Ping(vec![0x9])), // Raw Ping Frame
                )
                .await
                {
                    has_timed_out = true;
                } else {
                    println!("Couldn't send 'ping' message");
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::BrokenPipe,
                        "Couldn't send 'ping' message",
                    ));
                }
                continue;
            };
            match msg {
                Some(tokio_tungstenite::tungstenite::Message::Text(message))
                    if message.find("table").is_some() =>
                {
                    let bitmex_msg: BitmexParser = serde_json::from_str(&message).unwrap();
                    match bitmex_msg {
                        BitmexParser::Quotes(quotes) => {
                            let mut parsed_quotes = quotes.data;

                            for parsed_quote in &mut parsed_quotes {
                                let quote = Quote {
                                    default_timestamp,
                                    exchange_timestamp: parsed_quote.exchange_timestamp,
                                    best_bid_price: parsed_quote.best_bid_price,
                                    best_bid_size: parsed_quote.best_bid_size,
                                    best_ask_price: parsed_quote.best_ask_price,
                                    best_ask_size: parsed_quote.best_ask_size,
                                };
                                self.quotes_triples.quotes.push(quote);
                            }
                            if !self.quotes_triples.quotes.is_empty() {
                                let bin_quotes = self
                                    .custom_bincode
                                    .serialize(&self.quotes_triples.quotes)
                                    .unwrap();

                                self.quotes_triples
                                    .appender
                                    .write_all(&bin_quotes[Self::USIZE_LEN..])?;
                                self.quotes_triples.appender.flush()?;
                                self.quotes_triples.quotes.clear();
                            }
                        }
                        BitmexParser::Trades(trades) => {
                            let mut parsed_trades = trades.data;

                            for parsed_trade in &mut parsed_trades {
                                let trade = Trade {
                                    default_timestamp,
                                    exchange_timestamp: parsed_trade.exchange_timestamp,
                                    size: parsed_trade.size,
                                    price: parsed_trade.price,
                                    side: parsed_trade.side,
                                };
                                self.trades_triples.trades.push(trade);
                            }
                            if !self.trades_triples.trades.is_empty() {
                                let bin_trades = self
                                    .custom_bincode
                                    .serialize(&self.trades_triples.trades)
                                    .unwrap();

                                self.trades_triples
                                    .appender
                                    .write_all(&bin_trades[Self::USIZE_LEN..])?;
                                self.trades_triples.appender.flush()?;
                                self.trades_triples.trades.clear();
                            }
                        }
                        _ => (),
                    }
                }
                // tokio_tungstenite::tungstenite::Message::Binary(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Ping(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Close(_) => todo!(),
                other => println!("{:?}", other),
            }
            has_timed_out = false;
        }
    }
}
