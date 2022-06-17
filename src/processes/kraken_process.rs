use futures::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt; // for write_all()

use crate::commands::kraken_subscribe::{Subscribe, SubscribeCmd, Subscription, SubscriptionNames, Products};
use crate::data_extractors::kraken_websocket::KrakenWebsocket;

pub async fn kraken_process() {
    
    let stream = KrakenWebsocket::connect(vec![
        Subscribe {
            _type: SubscribeCmd::Subscribe,
            products: Products::Name(vec![String::from("XBT/USD")]),
            subscription: Subscription::Quotes {name:SubscriptionNames::Quotes },
        },
        Subscribe {
            _type: SubscribeCmd::Subscribe,
            products: Products::Name(vec![String::from("XBT/USD")]),
            subscription: Subscription::Trades {
                name: SubscriptionNames::Trades,
            },
        },
    ]
    )
    .await
    .unwrap();

    stream
        .try_for_each(|msg| async {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("/var/lib/trading-system/quotes-and-trades-extractor/v0.1/data/kraken.log")
                .await?;
            match msg {
                tokio_tungstenite::tungstenite::Message::Text(message) => {
                    if let Some(a) = message.chars().nth(1) {
                        if a.is_digit(10) {
                            file.write_all(format!("{}\n", message).as_bytes()).await?
                        }
                    }
                }
                // tokio_tungstenite::tungstenite::Message::Binary(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Ping(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Pong(_) => todo!(),
                // tokio_tungstenite::tungstenite::Message::Close(_) => todo!(),
                _other => println!("other"),
            }
            Ok(())
        })
        .await
        .expect("stream fail");
}
