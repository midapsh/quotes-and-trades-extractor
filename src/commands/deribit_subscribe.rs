use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: JsonRpc,
    pub id: IdCmd,
    #[serde(rename = "method")]
    pub _type: MethodCmd,
    #[serde(rename = "params")]
    pub args: Args,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MethodCmd {
    #[serde(rename = "public/subscribe")]
    Subscribe,
    #[serde(rename = "public/set_heartbeat")]
    Heartbeat,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum JsonRpc {
    Version(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum IdCmd {
    Id(usize),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Products {
    Name(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Args {
    Heartbeat { interval: usize },
    Orderbook { channels: Vec<String> },
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
