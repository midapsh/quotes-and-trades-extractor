use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "op")]
    pub _type: SubscribeCmd,
    #[serde(rename = "channel")]
    pub arg: ArgsType,
    #[serde(rename = "market")]
    pub product: Product,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SubscribeCmd {
    Subscribe,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Product {
    Name(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ArgsType {
    #[serde(rename = "candles")]
    Candles,
    #[serde(rename = "orderbook")]
    Orderbook,
    #[serde(rename = "ticker")]
    Quotes,
    #[serde(rename = "trades")]
    Trades,
}
