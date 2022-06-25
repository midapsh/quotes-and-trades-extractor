use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "table")]
pub enum BitmexParser {
    #[serde(rename = "trade")]
    Trades(Trades),
    #[serde(rename = "quote")]
    Quotes(Quotes),
    #[serde(skip)]
    WithOther(String),
}

#[derive(Deserialize, Debug)]
pub struct Trades {
    // #[serde(skip_deserializing)]
    // action: String,
    pub data: Vec<Trade>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    #[serde(default)]
    pub default_timestamp: i64,
    #[serde(with = "exchange_date_format")]
    #[serde(rename = "timestamp")]
    pub exchange_timestamp: i64,
    // pub symbol: String,
    pub size: f64,
    pub price: f64,
    #[serde(with = "exchange_side")]
    pub side: u8,
    // #[serde(skip_deserializing)]
    // trdMatchID: String,
    // #[serde(rename = "trdMatchID")]
    // exchange_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Quotes {
    // #[serde(skip_deserializing)]
    // action: String,
    pub data: Vec<Quote>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    #[serde(default)]
    pub default_timestamp: i64,
    #[serde(with = "exchange_date_format")]
    #[serde(rename = "timestamp")]
    pub exchange_timestamp: i64,
    // pub symbol: String,
    #[serde(rename = "bidPrice")]
    pub best_bid_price: f64,
    #[serde(rename = "bidSize")]
    pub best_bid_size: f64,
    #[serde(rename = "askPrice")]
    pub best_ask_price: f64,
    #[serde(rename = "askSize")]
    pub best_ask_size: f64,
}

pub fn get_default_timestamp() -> i64 {
    return Utc::now().timestamp_nanos();
}

mod exchange_date_format {
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";
    const A_BILLION: i64 = 1_000_000_000;

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(*date)
        // let dt = DateTime::<Utc>::from_utc(
        //     NaiveDateTime::from_timestamp(date / A_BILLION, (date % A_BILLION) as u32),
        //     Utc,
        // );
        // let s = format!("{}", dt.format(FORMAT));
        // serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
            .map(|dt| dt.timestamp_nanos())
    }
}

mod exchange_side {
    use serde::{self, Deserialize, Deserializer, Serializer};

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<'a, S>(side: &u8, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*side)
        // match side {
        //     b's' => serializer.serialize_str("Sell"),
        //     b'b' => serializer.serialize_str("Buy"),
        //     _ => Err(serde::ser::Error::custom("unknow variant")),
        // }
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de>,
    {
        let side = Deserialize::deserialize(deserializer)?;
        match side {
            "Sell" => Ok(b's'),
            "Buy" => Ok(b'b'),
            _ => Err(serde::de::Error::unknown_variant(side, &["Sell", "Buy"])),
        }
    }
}
