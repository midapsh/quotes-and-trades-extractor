use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "type")]
    pub _type: SubscribeCmd,
    pub channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SubscribeCmd {
    Subscribe,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Channel {
    WithProduct {
        #[serde(rename = "name")]
        channel: ChannelType,
        #[serde(rename = "product_ids")]
        products_ids: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChannelType {
    #[serde(rename = "heartbeat")]
    Heartbeat,
    #[serde(rename = "ticker")]
    Ticker,
    #[serde(rename = "level2")]
    Level2,
}
