use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "event")]
    pub _type: SubscribeCmd,
    #[serde(rename = "pair")]
    pub products: Products,
    #[serde(rename = "subscription")]
    pub subscription: Subscription,
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
pub enum Subscription {
    Tickers { name: SubscriptionNames },
    Trades { name: SubscriptionNames },
    Quotes { name: SubscriptionNames },
    Orderbook { name: SubscriptionNames, depth: usize },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscriptionNames {
    #[serde(rename = "ticker")]
    Tickers,
    #[serde(rename = "trade")]
    Trades,
    #[serde(rename = "spread")]
    Quotes,
    #[serde(rename = "book")]
    Orderbook,
}
