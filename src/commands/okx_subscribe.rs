use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "op")]
    pub _type: SubscribeCmd,
    pub args: Vec<Arg>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SubscribeCmd {
    Subscribe,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Arg {
    pub channel: ChannelsType,
    #[serde(rename = "instId")]
    pub instrument_id: String,
}

// Link: https://www.okx.com/docs/en/#websocket-api-public-channel-order-book-channel
#[derive(Serialize, Deserialize, Debug)]
pub enum ChannelsType {
    // #[serde(rename = "candles")]
    // Candles,
    #[serde(rename = "books-l2-tbt")]
    Orderbook,
    // #[serde(rename = "snapshot")]
    // Snapshot,
    // #[serde(rename = "trades")]
    // Trades,
}
