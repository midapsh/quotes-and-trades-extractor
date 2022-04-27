use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "method")]
    pub _type: SubscribeCmd,
    pub id: IdCmd,
    pub params: Params,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SubscribeCmd {
    Subscribe,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum IdCmd {
    Id(usize),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Params {
    WithProduct(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ParamsType {
    #[serde(rename = "candles")]
    Candles,
    #[serde(rename = "orderbook")]
    Orderbook,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "trades")]
    Trades,
}
