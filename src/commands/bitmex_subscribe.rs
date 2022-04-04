use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "op")]
    pub _type: SubscribeCmd,
    pub args: Args,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SubscribeCmd {
    Subscribe,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Args {
    WithProduct(Vec<String>),
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
