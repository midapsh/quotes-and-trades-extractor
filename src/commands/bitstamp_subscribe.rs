use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "event")]
    pub _type: SubscribeCmd,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SubscribeCmd {
    #[serde(rename = "bts:subscribe")]
    Subscribe,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub channel: String,
}

// Link: https://www.okx.com/docs/en/#websocket-api-public-channel-order-book-channel
#[derive(Serialize, Deserialize, Debug)]
pub enum ArgsType {
    // #[serde(rename = "candles")]
    // Candles,
    #[serde(rename = "orderbook")]
    Orderbook,
    // #[serde(rename = "snapshot")]
    // Snapshot,
    // #[serde(rename = "trades")]
    // Trades,
}
