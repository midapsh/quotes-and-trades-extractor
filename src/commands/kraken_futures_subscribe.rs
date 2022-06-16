use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "event")]
    pub _type: SubscribeCmd,
    pub product_ids: Vec<String>,
    pub feed: FeedType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SubscribeCmd {
    Subscribe,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FeedType {
    #[serde(rename = "book")]
    Orderbook,
    #[serde(rename = "ticker")]
    Ticker,
    #[serde(rename = "trade")]
    Trade,
}
