use std::io::Write;

use async_trait::async_trait;
use bincode::config::{AllowTrailing, FixintEncoding, WithOtherIntEncoding, WithOtherTrailing};
use bincode::{DefaultOptions, Options};
use futures::{SinkExt, TryStreamExt};
use tokio::time::{self, sleep};
use tokio_util::sync::CancellationToken;

use crate::commands::bitmex_subscribe::Args;
use crate::configs::configuration::{get_configuration, ExchangeSettings};
use crate::custom_parsers::bitmex_parser::{get_default_timestamp, BitmexParser};
use crate::data_extractors::bitmex_websocket::BitmexWebsocket;
use crate::models::bitmex_models::{Quote, Trade};
use crate::utils::instrument_info::InstrumentInfo;
use crate::utils::path_info::PathInfo;
use crate::workers::base_worker::Worker;

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

pub struct BitmexWorker {
    exchange_settings: ExchangeSettings,
    cancellation_token: CancellationToken,
    instrument_info: InstrumentInfo,
    quotes_triples: QuotesTriple,
    trades_triples: TradesTriple,
    custom_bincode: CustomBincode,
}

impl BitmexWorker {
    // NOTE(hspadim): I use this because I need to remove the size of the vector,
    // the first value of the byte stream
    const USIZE_LEN: usize = 8;

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
                println!(
                    "{} - Couldn't send 'ping' message",
                    self.instrument_info.instrument
                );
                return Err(std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "Couldn't send 'ping' message",
                ));
            }
        }

        let mut has_timed_out: bool = false;
        while !self.cancellation_token.is_cancelled() {
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
                    println!(
                        "{} - Couldn't send 'ping' message",
                        self.instrument_info.instrument
                    );
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
                    continue;
                } else {
                    println!(
                        "{} - Couldn't send 'ping' message",
                        self.instrument_info.instrument
                    );
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::BrokenPipe,
                        "Couldn't send 'ping' message",
                    ));
                }
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
                        } // _ => (),
                    }
                }
                // tokio_tungstenite::tungstenite::Message::Binary(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Ping(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Close(_) => todo!(),
                Some(tokio_tungstenite::tungstenite::Message::Pong(_)) => {}
                None => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!(
                            "Couldn't receive any message. Reconnecting {}",
                            self.instrument_info.instrument_parsed
                        ),
                    ));
                }
                other => {
                    println!(
                        "Not parsed message from {} - {:?}",
                        self.instrument_info.instrument_parsed, other
                    )
                }
            }
            has_timed_out = false;
        }
        Ok(())
    }
}

#[async_trait]
impl Worker for BitmexWorker {
    fn new(
        cancellation_token: CancellationToken,
        instrument_info: InstrumentInfo,
        path_info: PathInfo,
    ) -> Self {
        // NOTE(hspadim):
        let settings = get_configuration().unwrap();
        let exchange_settings = settings.bitmex;
        // NOTE(hspadim): File saving stuff
        let file_name_prefix = format!("{}.dat", instrument_info.instrument_parsed);
        let quotes_appender =
            tracing_appender::rolling::daily(path_info.data_quotes_path, file_name_prefix.clone());
        let (non_blocking_quotes_appender, _quotes_guard) =
            tracing_appender::non_blocking(quotes_appender);

        let trades_appender =
            tracing_appender::rolling::daily(path_info.data_trades_path, file_name_prefix);
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
            exchange_settings,
            //
            cancellation_token,
            instrument_info,
            // path_info,
            custom_bincode,
            quotes_triples,
            trades_triples,
        }
    }

    async fn execute(&mut self) -> std::io::Result<()> {
        let subscription = [
            String::from(format!("quote:{}", self.instrument_info.instrument)),
            String::from(format!("trade:{}", self.instrument_info.instrument)),
        ];

        while !self.cancellation_token.is_cancelled() {
            let stream = match BitmexWebsocket::connect(
                &self.exchange_settings.websocket_url,
                Args::WithProduct(subscription.to_vec()),
            )
            .await
            {
                Ok(it) => it,
                Err(err) => {
                    // TODO(hspadim): Should try to connect to another symbol
                    eprintln!("{}", err);
                    time::sleep(time::Duration::from_secs(60)).await;
                    continue;
                }
            };

            match self.stream_loop(stream).await {
                Ok(()) => {}
                Err(e) => match e.kind() {
                    std::io::ErrorKind::InvalidData => (),
                    _ => sleep(tokio::time::Duration::from_secs(5)).await,
                },
            }
        }
        Ok(())
    }

    fn stop(&self) {
        if !self.cancellation_token.is_cancelled() {
            self.cancellation_token.cancel();
        }
    }
}
