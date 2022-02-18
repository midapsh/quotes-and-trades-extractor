mod commands;
mod data_extractors;
mod processes;

use processes::coinbase_process::coinbase_process;

#[tokio::main]
async fn main() {
    coinbase_process().await;
}

// use commands::subscribe::{Channel, ChannelType, Subscribe, SubscribeCmd};

// #[tokio::main]
// async fn main() {
//     let channel = ChannelType::Level2;
//     let products_ids = ["BTC-USD"];

//     let subscribe = Subscribe {
//         _type: SubscribeCmd::Subscribe,
//         channels: vec![Channel::WithProduct {
//             channel,
//             products_ids: products_ids.into_iter().map(|x| x.to_string()).collect(),
//         }],
//     };
//     println!("{}", serde_json::to_string_pretty(&subscribe).unwrap());
// }
