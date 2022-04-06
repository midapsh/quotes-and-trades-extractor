use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "event")]
    pub _type: SubscribeCmd,
    #[serde(rename = "pair")]
    pub products: Products,
    #[serde(rename = "subscription")]
    pub options: Options,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SubscribeCmd {
    Subscribe,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Products {
    Name(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Options {
    Orderbook { name: String, depth: usize },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ArgsType {
    #[serde(rename = "candles")]
    Candles,
    #[serde(rename = "orderbook")]
    Orderbook,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "trades")]
    Trades,
}
