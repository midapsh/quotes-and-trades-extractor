use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub default_timestamp: i64,
    pub exchange_timestamp: i64,
    pub best_bid_price: f64,
    pub best_bid_size: f64,
    pub best_ask_price: f64,
    pub best_ask_size: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub default_timestamp: i64,
    pub exchange_timestamp: i64,
    pub size: f64,
    pub price: f64,
    pub side: u8,
}
