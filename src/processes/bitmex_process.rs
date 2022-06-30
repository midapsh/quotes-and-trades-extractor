use bincode::config::{AllowTrailing, FixintEncoding, WithOtherIntEncoding, WithOtherTrailing};
use bincode::{DefaultOptions, Options};
use futures::TryStreamExt;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

use crate::commands::bitmex_subscribe::Args;
use crate::custom_parsers::bitmex_parser::{get_default_timestamp, BitmexParser, ParsedQuote, ParsedTrade};
use crate::data_extractors::bitmex_websocket::BitmexWebsocket;
use crate::models::bitmex_models::{Quote, Trade};

type CustomBincode =
    WithOtherTrailing<WithOtherIntEncoding<DefaultOptions, FixintEncoding>, AllowTrailing>;

pub struct QuotesTriple {
    appender: tracing_appender::non_blocking::NonBlocking,
    guard: tracing_appender::non_blocking::WorkerGuard,
    quotes: Vec<Quote>,
}

pub struct TradesTriple {
    appender: tracing_appender::non_blocking::NonBlocking,
    guard: tracing_appender::non_blocking::WorkerGuard,
    trades: Vec<Trade>,
}

pub struct BitmexProcess<'a> {
    instruments: Vec<&'a str>,
    _instruments_parsed: Vec<&'a str>,
    quotes_process_dict: HashMap<&'a str, QuotesTriple>,
    trades_process_dict: HashMap<&'a str, TradesTriple>,
    custom_bincode: CustomBincode,
}

impl<'a> BitmexProcess<'a> {
    pub fn new(
        instruments: Vec<&'a str>,
        instruments_parsed: Vec<&'a str>,
        data_quotes_path: &PathBuf,
        data_trades_path: &PathBuf,
    ) -> Self {
        let mut quotes_process_dict: HashMap<&'a str, QuotesTriple> = HashMap::with_capacity(1_000);
        let mut trades_process_dict: HashMap<&'a str, TradesTriple> = HashMap::with_capacity(1_000);

        for (instrument, instrument_parsed) in instruments.iter().zip(instruments_parsed.iter()) {
            let file_name_prefix = format!("{}.dat", instrument_parsed);
            let quotes_appender =
                tracing_appender::rolling::daily(data_quotes_path, file_name_prefix.clone());
            let (non_blocking_quotes_appender, _quotes_guard) =
                tracing_appender::non_blocking(quotes_appender);

            let trades_appender =
                tracing_appender::rolling::daily(data_trades_path, file_name_prefix);
            let (non_blocking_trades_appender, _trades_guard) =
                tracing_appender::non_blocking(trades_appender);

            quotes_process_dict.insert(
                instrument,
                QuotesTriple {
                    appender: non_blocking_quotes_appender,
                    guard: _quotes_guard,
                    quotes: Vec::with_capacity(4096),
                },
            );
            trades_process_dict.insert(
                instrument,
                TradesTriple {
                    appender: non_blocking_trades_appender,
                    guard: _trades_guard,
                    trades: Vec::with_capacity(4096),
                },
            );
        }

        let custom_bincode = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes();

        Self {
            instruments,
            _instruments_parsed: instruments_parsed,
            custom_bincode,
            quotes_process_dict,
            trades_process_dict,
        }
    }
}

impl<'a> BitmexProcess<'a> {
    // NOTE(hspadim): I use this because I need to remove the size of the vector,
    // the first value of the byte stream
    const USIZE_LEN: usize = 8;
    pub async fn run(&mut self) -> std::io::Result<()> {
        let subscriptions = self
            .instruments
            .iter()
            .flat_map(|instrument| {
                return [
                    String::from(format!("quote:{}", instrument)),
                    String::from(format!("trade:{}", instrument)),
                ];
            })
            .collect();
        let mut stream = BitmexWebsocket::connect(Args::WithProduct(subscriptions))
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
                            for parsed_quote in &mut quotes {
                                let quote = Quote {
                                    default_timestamp,
                                    exchange_timestamp: parsed_quote.exchange_timestamp,
                                    best_bid_price: parsed_quote.best_bid_price,
                                    best_bid_size: parsed_quote.best_bid_size,
                                    best_ask_price: parsed_quote.best_ask_price,
                                    best_ask_size: parsed_quote.best_ask_size,
                                };
                                let symbol = parsed_quote.symbol.as_str();
                                self.quotes_process_dict
                                    .get_mut(symbol)
                                    .unwrap()
                                    .quotes
                                    .push(quote);
                            }
                            
                            for (_, QuotesTriple { appender, quotes, guard: _ }) in self.quotes_process_dict.iter_mut() {
                                if !quotes.is_empty() {
                                    // NOTE(hspadim): I want to fail hard when I don't find the key
                                    // because it's suposed to have all symbols inside of it.
                                    let bin_quotes = self.custom_bincode.serialize(&quotes).unwrap();

                                    appender.write_all(&bin_quotes[Self::USIZE_LEN..])?;
                                    appender.flush()?;
                                    quotes.clear();
                                }
                            }
                        }
                        BitmexParser::Trades(trades) => {
                            let mut trades = trades.data;
                            for parsed_trade in &mut trades {
                                let trade = Trade {
                                    default_timestamp,
                                    exchange_timestamp: parsed_trade.exchange_timestamp,
                                    size: parsed_trade.size,
                                    price: parsed_trade.price,
                                    side: parsed_trade.side,
                                };
                                let symbol = parsed_trade.symbol.as_str();
                                self.trades_process_dict
                                    .get_mut(symbol)
                                    .unwrap()
                                    .trades
                                    .push(trade);
                            }
                            
                            for (_, TradesTriple { appender, trades, guard: _ }) in self.trades_process_dict.iter_mut() {
                                if !trades.is_empty() {
                                    // NOTE(hspadim): I want to fail hard when I don't find the key
                                    // because it's suposed to have all symbols inside of it.
                                    let bin_quotes = self.custom_bincode.serialize(&trades).unwrap();

                                    appender.write_all(&bin_quotes[Self::USIZE_LEN..])?;
                                    appender.flush()?;
                                    trades.clear();
                                }
                            }
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
